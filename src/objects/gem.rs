use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};
use crate::effects::Animation;
use crate::assets::sprites::SpriteSheet;

pub struct Gem {
    pub position: Vec2,
    pub active: bool,
    pub collected: bool,
    animation: Animation,
    #[allow(dead_code)]
    gem_type: u8,        // 1-6
    health_reward: u8,   // 1-5 health
    power_reward: u8,    // 5 power
    collection_timer: f32,  // 0.1s duration
    collection_scale: f32,  // 1.0 -> 1.05 during collection
}

impl Gem {
    pub async fn new(texture_path: &str, gem_type: u8) -> Option<Self> {
        // Load sprite sheet first (7 frames, 16x16 each)
        let sprite_sheet = SpriteSheet::new(texture_path, 16.0, 16.0, 7).await?;
        
        // Create animation with the sprite sheet
        let mut animation = Animation::new_with_texture(sprite_sheet.get_texture().clone(), 16, 16);
        animation.set_animation_fps(10.0); // 10fps for gem animation
        
        // Random health reward between 1-5
        let mut rng = thread_rng();
        let health_reward = rng.gen_range(1..=5);

        Some(Self {
            position: Vec2::ZERO,
            animation,
            active: true,
            collected: false,
            gem_type,
            health_reward,
            power_reward: 5,  // Fixed power reward
            collection_timer: 0.0,
            collection_scale: 1.0,
        })
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        let scale = if self.collected { self.collection_scale } else { 1.0 };
                
        // Use our standard animation draw method
        self.animation.draw(
            self.position,
            false, // never flip gems
            Vec2::new(scale, scale),
            WHITE,
        );

        // Keep debug bounding box
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            16.0,  // Fixed gem size
            16.0,
            2.0,
            BLUE,
        );
    }

    pub fn update(&mut self, dt: f32) -> bool {
        // Update animation
        self.animation.update(dt);

        if self.collected {
            self.collection_timer += dt;
            
            // Increase scale during first half of collection
            if self.collection_timer < 0.05 {
                self.collection_scale = 1.0 + (self.collection_timer / 0.05) * 0.05;
            } else {
                self.collection_scale = 1.05 - ((self.collection_timer - 0.05) / 0.05) * 0.05;
            }

            // Check if collection animation is complete
            if self.collection_timer >= 0.1 {
                self.active = false;
                return true;
            }
        }
        false
    }

    pub fn collect(&mut self) {
        if !self.collected && self.active {
            self.collected = true;
            self.collection_timer = 0.0;
            self.collection_scale = 1.0;
        }
    }

    // Helper to get collision rect
    pub fn bounds(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            16.0,  // Fixed gem size
            16.0,
        )
    }

    pub fn get_rewards(&self) -> (u8, u8) {
        (self.health_reward, self.power_reward)
    }
}