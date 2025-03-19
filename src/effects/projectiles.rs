use crate::assets::SpriteSheet;
use macroquad::prelude::*;

pub struct Projectile {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub facing_left: bool,
    pub owner_type: ProjectileOwner,
    pub owner_id: usize,
    pub damage: f32,
    pub state: ProjectileState,
    animation: Option<ProjectileAnimation>,
}

#[derive(Debug)]
pub struct ProjectileAnimation {
    sprite_sheet: SpriteSheet,
    current_frame: usize,
    frame_time: f32,
    frame_timer: f32,
    opacity: f32,
}

impl ProjectileAnimation {
    pub async fn new(owner: ProjectileOwner) -> Option<Self> {
        // Load from correct directory based on owner
        let texture_path = match owner {
            ProjectileOwner::Player => String::from("assets/Player/Arrow.png"),
            ProjectileOwner::Enemy(enemy_path) => format!("{}/Arrow.png", enemy_path),
        };

        if let Ok(texture) = load_texture(&texture_path).await {
            let mut sprite_sheet = SpriteSheet::from_texture(texture, 64.0, 128.0);
            sprite_sheet.set_looping(false); // Don't loop animation
            let _total_frames = sprite_sheet.get_frame_count();
            Some(Self {
                sprite_sheet,
                current_frame: 0,
                frame_time: 0.4, // 400ms per frame for arrow animation
                frame_timer: 0.0,
                opacity: 1.0,
            })
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f32, state: ProjectileState) {
        self.frame_timer += dt;

        match state {
            ProjectileState::Active => {
                if self.frame_timer >= self.frame_time {
                    self.frame_timer -= self.frame_time;
                    // Get actual frame count from SpriteSheet
                    let max_frames = self.sprite_sheet.get_frame_count();

                    if self.current_frame < max_frames - 1 {
                        // Still animating, use correct frame range (0 to max_frames-1)
                        self.current_frame += 1;
                        self.sprite_sheet.force_frame(self.current_frame);
                    } else {
                        // Animation complete, start fading
                        self.opacity = (self.opacity - dt * 4.0).max(0.0);
                    }
                }
            }
            ProjectileState::Fading => {
                // For collision-based fading
                self.opacity = (self.opacity - dt * 4.0).max(0.0);
            }
            ProjectileState::Done => {}
        }
    }

    pub fn draw(&self, pos: Vec2, facing_left: bool) {
        let color = Color::new(1.0, 1.0, 1.0, self.opacity);
        self.sprite_sheet.draw(pos, facing_left, Vec2::ONE, color);
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_timer = 0.0;
        self.opacity = 1.0;
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        self.opacity <= 0.0
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ProjectileOwner {
    Player,
    Enemy(String), // Contains path to enemy assets (e.g. "assets/Enemies/Ghost_1")
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectileState {
    Active,
    Fading,
    Done,
}

impl Projectile {
    pub fn new(
        pos: Vec2,
        facing_left: bool,
        owner: ProjectileOwner,
        owner_id: usize,
        damage: f32,
    ) -> Self {
        let base_velocity = Vec2::new(200.0, 0.0); // Reduced from 400.0 for slower movement
        let velocity = if facing_left {
            Vec2::new(-base_velocity.x, base_velocity.y)
        } else {
            base_velocity
        };

        Self {
            pos,
            velocity,
            facing_left,
            owner_type: owner,
            owner_id,
            damage,
            state: ProjectileState::Active,
            animation: None,
        }
    }

    pub async fn initialize_animation(&mut self) {
        if let Some(anim) = ProjectileAnimation::new(self.owner_type.clone()).await {
            self.animation = Some(anim);
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            ProjectileState::Active => {
                self.pos += self.velocity * dt;
                if let Some(anim) = &mut self.animation {
                    anim.update(dt, self.state);
                    // Check if completely faded out
                    if anim.opacity <= 0.0 {
                        self.state = ProjectileState::Done;
                    }
                }
            }
            ProjectileState::Fading => {
                // Stop moving when fading from collision
                if let Some(anim) = &mut self.animation {
                    anim.update(dt, self.state);
                    if anim.opacity <= 0.0 {
                        self.state = ProjectileState::Done;
                    }
                }
            }
            ProjectileState::Done => {}
        }
    }

    pub fn draw(&self) {
        if let Some(anim) = &self.animation {
            if self.state != ProjectileState::Done {
                // Draw the sprite
                anim.draw(self.pos, self.facing_left);

                // Draw bounding box
                let projectile_size = Vec2::new(64.0, 128.0); // Match sprite and collision size
                draw_rectangle_lines(
                    self.pos.x,
                    self.pos.y,
                    projectile_size.x,
                    projectile_size.y,
                    2.0,
                    YELLOW, // Distinct color for arrow hitbox
                );
            }
        }
    }

    pub fn check_collision(&self, target_pos: Vec2, target_size: Vec2) -> Option<Vec2> {
        if self.state != ProjectileState::Active {
            println!("Skipping collision check - projectile not active");
            return None;
        }

        let projectile_size = Vec2::new(64.0, 128.0); // Match sprite size

        // Basic collision check
        let collides = self.pos.x < target_pos.x + target_size.x
            && self.pos.x + projectile_size.x > target_pos.x
            && self.pos.y < target_pos.y + target_size.y
            && self.pos.y + projectile_size.y > target_pos.y;

        if !collides {
            return None;
        }

        // Calculate collision point at the edge of the enemy bounding box
        let collision_x = if self.facing_left {
            // Moving left, collision at right edge of arrow with left edge of target
            target_pos.x + target_size.x
        } else {
            // Moving right, collision at left edge of arrow with right edge of target
            target_pos.x - projectile_size.x
        };

        // Keep the same Y position
        Some(Vec2::new(collision_x, self.pos.y))
    }

    pub fn handle_collision(&mut self, collision_point: Vec2) {
        // Update position to collision point and start fade
        self.pos = collision_point;
        self.state = ProjectileState::Fading;
        if let Some(anim) = &mut self.animation {
            anim.reset();
        }
    }

    pub fn get_state(&self) -> ProjectileState {
        self.state
    }

    pub fn is_done(&self) -> bool {
        self.state == ProjectileState::Done
    }
}
