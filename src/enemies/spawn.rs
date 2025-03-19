use macroquad::prelude::*;

// Simple spawn animation - just the sprite
pub struct SpawnAnimation {
    texture: Texture2D,
    current_frame: usize,
    animation_timer: f32,
}

impl SpawnAnimation {
    pub async fn new() -> Option<Self> {
        match load_texture("assets/Magic/Spawn_Enemy.png").await {
            Ok(texture) => Some(Self {
                texture,
                current_frame: 0,
                animation_timer: 0.0,
            }),
            Err(_) => None,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= 1.0 / 5.0 {
            // SLOWED to 5 FPS
            self.animation_timer = 0.0;
            if self.current_frame < 9 {
                self.current_frame += 1;
            }
        }
    }
}

pub struct EnemySpawn {
    position: Vec2,
    animation: SpawnAnimation,
}

impl EnemySpawn {
    pub async fn new(position: Vec2) -> Option<Self> {
        if let Some(animation) = SpawnAnimation::new().await {
            Some(Self {
                position,
                animation,
            })
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation.update(dt);
    }

    pub fn draw(&self) {
        let box_size = 72.0; // Match spawn sprite size

        // Draw sprite at EXACT position first
        let frame_x = self.animation.current_frame as f32 * box_size;
        draw_texture_ex(
            &self.animation.texture,
            self.position.x, // Position is ALREADY the box coordinates
            self.position.y, // Position is ALREADY the box coordinates
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(frame_x, 0.0, box_size, box_size)),
                dest_size: Some(Vec2::new(box_size, box_size)),
                ..Default::default()
            },
        );

        // Draw spawn box overlay
        draw_rectangle(
            self.position.x,
            self.position.y,
            box_size,
            box_size,
            Color::new(0.0, 0.0, 0.0, 0.5), // More visible black background
        );
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            box_size,
            box_size,
            2.0,
            PURPLE
        );

        // Draw "SPAWN" text at top
        draw_text(
            "SPAWN",
            self.position.x + 8.0,  // Offset from left edge
            self.position.y - 5.0,  // Position above box
            20.0,                   // Font size
            PURPLE                  // Match box color
        );
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn is_complete(&self) -> bool {
        self.animation.current_frame == 9
    }
}