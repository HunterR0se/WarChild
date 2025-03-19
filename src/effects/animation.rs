use macroquad::prelude::*;
use crate::assets::SpriteSheet;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AnimationState {
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Attacking1,
    Attacking2,
    Attacking3,
    Attacking4,
    Hurt,
    Dead,
    Charging,
    Teleporting,
    Climbing,
    Hanging,
    PullUp,
    Resting,
    Rolling,      // Player's rolling dodge animation
    Special,      // For special attacks
    AreaAttack,   // For area attacks
    Poison,       // For poison effect overlay
    Shoot         // Player's shoot animation (not the arrow projectile)
}

#[derive(Debug, Clone)]
pub struct Animation {
    sprite_sheets: HashMap<AnimationState, SpriteSheet>,
    current_state: AnimationState,
    // Effect fields
    flash_timer: f32,
    is_flashing: bool,
    original_scale: Vec2,
    target_scale: Vec2,
    scale_duration: f32,
    is_scaling: bool,
    current_scale: Vec2,
    flash_color: Color,
}

impl Animation {
    pub async fn new() -> Option<Self> {
        // Initialize sprite sheets
        let mut sprite_sheets = HashMap::new();
        
        // Load spritesheet textures
        let idle_texture = load_texture("assets/Player/Idle.png").await;
        let walking_texture = load_texture("assets/Player/Walk.png").await;
        let running_texture = load_texture("assets/Player/Run.png").await;
        let jumping_texture = load_texture("assets/Player/Jump.png").await;
        let attack1_texture = load_texture("assets/Player/Attack_1.png").await;
        let attack2_texture = load_texture("assets/Player/Attack_2.png").await;
        let attack3_texture = load_texture("assets/Player/Attack_3.png").await;
        let attack4_texture = load_texture("assets/Player/Attack_4.png").await;
        let dead_texture = load_texture("assets/Player/Dead.png").await;
        let hang_texture = load_texture("assets/Player/Hang.png").await;
        let pullup_texture = load_texture("assets/Player/PullUp.png").await;
        let shoot_texture = load_texture("assets/Player/Shoot.png").await;
        
        // Load roll animation
        let roll_texture = load_texture("assets/Player/Roll.png").await;
        
        // Only continue if all textures loaded
        if idle_texture.is_ok() && walking_texture.is_ok() && running_texture.is_ok() 
            && jumping_texture.is_ok() && attack1_texture.is_ok() && attack2_texture.is_ok()
            && attack3_texture.is_ok() && attack4_texture.is_ok() && dead_texture.is_ok()
            && hang_texture.is_ok() && pullup_texture.is_ok() && shoot_texture.is_ok()
            && roll_texture.is_ok() {
            
            // Create sprite sheets for each animation
            let attack2_sheet = SpriteSheet::from_texture(attack2_texture.unwrap(), 128.0, 128.0);
            sprite_sheets.insert(AnimationState::Attacking2, attack2_sheet);

            sprite_sheets.insert(AnimationState::Idle, 
                SpriteSheet::from_texture(idle_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Walking,
                SpriteSheet::from_texture(walking_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Running,
                SpriteSheet::from_texture(running_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Jumping,
                SpriteSheet::from_texture(jumping_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Attacking1,
                SpriteSheet::from_texture(attack1_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Attacking3,
                SpriteSheet::from_texture(attack3_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Attacking4,
                SpriteSheet::from_texture(attack4_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Dead,
                SpriteSheet::from_texture(dead_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Hanging,
                SpriteSheet::from_texture(hang_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::PullUp,
                SpriteSheet::from_texture(pullup_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Shoot,
                SpriteSheet::from_texture(shoot_texture.unwrap(), 128.0, 128.0));
            sprite_sheets.insert(AnimationState::Rolling,
                SpriteSheet::from_texture(roll_texture.unwrap(), 128.0, 128.0));
            
            // Return initialized animation with effects
            Some(Animation {
                sprite_sheets,
                current_state: AnimationState::Idle,
                flash_timer: 0.0,
                is_flashing: false,
                original_scale: Vec2::ONE,
                target_scale: Vec2::ONE,
                scale_duration: 0.0,
                is_scaling: false,
                current_scale: Vec2::ONE,
                flash_color: WHITE,
            })
        } else {
            None
        }
    }

    pub fn new_with_texture(texture: Texture2D, frame_width: i32, frame_height: i32) -> Self {
        let mut sprite_sheets = HashMap::new();
        let sheet = SpriteSheet::from_texture(texture, frame_width as f32, frame_height as f32);
        sprite_sheets.insert(AnimationState::Idle, sheet);
        
        Self {
            sprite_sheets,
            current_state: AnimationState::Idle,
            flash_timer: 0.0,
            is_flashing: false,
            original_scale: Vec2::ONE,
            target_scale: Vec2::ONE,
            scale_duration: 0.0,
            is_scaling: false,
            current_scale: Vec2::ONE,
            flash_color: WHITE,
        }
    }

    pub fn set_state_fps(&mut self, state: AnimationState, fps: f32) {
        if let Some(sheet) = self.sprite_sheets.get_mut(&state) {
            sheet.set_animation_fps(fps);
        }
    }

    pub fn set_animation_fps(&mut self, fps: f32) {
        for sheet in self.sprite_sheets.values_mut() {
            sheet.set_animation_fps(fps);
        }
    }

    pub fn set_state(&mut self, new_state: AnimationState) {
        // Don't interrupt attack animations unless it's another attack
        let current_is_attack = matches!(self.current_state, 
            AnimationState::Attacking1 | AnimationState::Attacking2 | 
            AnimationState::Attacking3 | AnimationState::Attacking4 |
            AnimationState::Shoot);  // Include Shoot as non-interruptible
        
        let new_is_attack = matches!(new_state,
            AnimationState::Attacking1 | AnimationState::Attacking2 | 
            AnimationState::Attacking3 | AnimationState::Attacking4 |
            AnimationState::Shoot);  // Include Shoot as an attack type

        // If we're in an attack and not finished, only allow changing to another attack
        if current_is_attack && !new_is_attack {
            if let Some(sheet) = self.sprite_sheets.get(&self.current_state) {
                if !sheet.is_finished() {
                    return;
                }
            }
        }

        if self.current_state != new_state {
            self.current_state = new_state;
            if let Some(sheet) = self.sprite_sheets.get_mut(&new_state) {
                sheet.force_frame(0);  // Always start from first frame
                // Don't loop attacks or jumping animations
                let should_loop = match new_state {
                    AnimationState::Jumping |
                    AnimationState::Attacking1 |
                    AnimationState::Attacking2 |
                    AnimationState::Attacking3 |
                    AnimationState::Attacking4 |
                    AnimationState::Rolling |
                    AnimationState::Shoot => false,  // Non-looping animations
                    _ => true
                };
                sheet.set_looping(should_loop);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update sprite sheet animation
        if let Some(sheet) = self.sprite_sheets.get_mut(&self.current_state) {
            sheet.update(dt);
            
            // Check if we need to auto-transition back to Idle
            let should_transition = matches!(self.current_state, 
                AnimationState::Attacking1 | AnimationState::Attacking2 | 
                AnimationState::Attacking3 | AnimationState::Attacking4 |
                AnimationState::Rolling);  // Auto-transition these animations
                
            if should_transition {
                if sheet.is_finished() && sheet.current_frame() == sheet.get_frame_count() - 1 {
                    self.current_state = AnimationState::Idle;
                    if let Some(idle_sheet) = self.sprite_sheets.get_mut(&AnimationState::Idle) {
                        idle_sheet.force_frame(0);
                        idle_sheet.set_looping(true);
                    }
                }
            }
        }

        // Update visual effects
        self.update_effects(dt);
    }

    pub fn flash_white(&mut self) {
        self.is_flashing = true;
        self.flash_timer = 0.0;
        self.flash_color = WHITE;
    }

    pub fn grow_by_percent(&mut self, percent: f32) {
        self.is_scaling = true;
        self.scale_duration = 0.0;
        self.original_scale = Vec2::ONE; // Always start from base size
        self.target_scale = Vec2::ONE * (1.0 + percent / 100.0); // Grow from base size
    }

    pub fn update_effects(&mut self, dt: f32) {
        // Update flash effect
        if self.is_flashing {
            self.flash_timer += dt;
            if self.flash_timer >= 0.1 { // Flash duration: 0.1 seconds
                self.is_flashing = false;
                self.flash_timer = 0.0;
            }
        }

        // Update scale effect
        if self.is_scaling {
            self.scale_duration += dt;
            let total_duration = 0.4; // 0.2s grow + 0.2s shrink
            let scale_progress = (self.scale_duration / total_duration).min(1.0);
            
            if scale_progress < 0.5 {
                // First half: grow to target size
                let grow_progress = scale_progress * 2.0; // Convert 0-0.5 to 0-1
                self.current_scale = Vec2::lerp(
                    self.original_scale,
                    self.target_scale,
                    grow_progress
                );
            } else {
                // Second half: shrink back to original
                let shrink_progress = (scale_progress - 0.5) * 2.0; // Convert 0.5-1 to 0-1
                self.current_scale = Vec2::lerp(
                    self.target_scale,
                    self.original_scale,
                    shrink_progress
                );
            }

            if scale_progress >= 1.0 {
                self.is_scaling = false;
                self.scale_duration = 0.0;
                self.current_scale = self.original_scale; // Ensure we're back to original size
            }
        }
    }

    pub fn get_current_scale(&self) -> Vec2 {
        self.current_scale
    }

    pub fn get_flash_color(&self) -> Option<Color> {
        if self.is_flashing {
            Some(self.flash_color)
        } else {
            None
        }
    }

    pub fn force_frame(&mut self, frame: usize) {
        if let Some(sheet) = self.sprite_sheets.get_mut(&self.current_state) {
            sheet.force_frame(frame);
        }
    }

    pub fn draw(&self, position: Vec2, flip_x: bool, scale: Vec2, color: Color) {
        if let Some(sheet) = self.sprite_sheets.get(&self.current_state) {
            let final_scale = if self.is_scaling {
                scale * self.current_scale
            } else {
                scale
            };

            // When scaling, adjust Y position to keep feet at same position
            let scaled_pos = if self.is_scaling {
                // Calculate how much taller the sprite is with current scale
                let height_increase = 128.0 * (self.current_scale.y - 1.0);
                // Move position up by that amount to keep feet in same place
                Vec2::new(position.x, position.y - height_increase)
            } else {
                position
            };

            let final_color = if self.is_flashing {
                self.flash_color
            } else {
                color
            };

            sheet.draw(scaled_pos, flip_x, final_scale, final_color);
        }
    }

    pub fn get_current_frame(&self) -> Option<usize> {
        self.sprite_sheets.get(&self.current_state).map(|sheet| sheet.current_frame())
    }

    pub fn is_in_state(&self, state: &AnimationState) -> bool {
        self.current_state == *state
    }

    pub fn set_looping(&mut self, should_loop: bool) {
        if let Some(sheet) = self.sprite_sheets.get_mut(&self.current_state) {
            sheet.set_looping(should_loop);
        }
    }

    pub fn reset(&mut self) {
        if let Some(sheet) = self.sprite_sheets.get_mut(&self.current_state) {
            sheet.force_frame(0);  // Reset to first frame
        }
    }

    #[allow(dead_code)]
    pub fn advance_frame(&mut self) -> bool {
        // Returns true if we advanced, false if we're at the end
        if let Some(sheet) = self.sprite_sheets.get_mut(&self.current_state) {
            sheet.advance_frame()
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(sheet) = self.sprite_sheets.get(&self.current_state) {
            sheet.is_finished()
        } else {
            false
        }
    }

    pub fn get_frame_count(&self) -> usize {
        self.sprite_sheets.get(&self.current_state)
            .map(|sheet| sheet.get_frame_count())
            .unwrap_or(0)
    }

    pub fn current_frame(&self) -> usize {
        self.sprite_sheets.get(&self.current_state)
            .map(|sheet| sheet.current_frame())
            .unwrap_or(0)
    }
}