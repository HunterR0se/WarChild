use macroquad::prelude::*;
use crate::effects::Animation;
use crate::assets::sprites::SpriteSheet;
use super::Collectible;

pub struct Artifact {
    pub position: Vec2,
    pub active: bool,
    pub collected: bool,
    animation: Animation,
    collection_timer: f32,  // 0.1s duration
    collection_scale: f32,  // 1.0 -> 1.05 during collection
    power_reward: u8,
    health_reward: u8,
}

impl Artifact {
    pub async fn new(texture_path: &str) -> Option<Self> {
        // Load sprite sheet (4 frames for artifacts)
        let sprite_sheet = SpriteSheet::new(texture_path, 20.0, 20.0, 4).await?;
        
        // Create animation with the sprite sheet
        let mut animation = Animation::new_with_texture(sprite_sheet.get_texture().clone(), 20, 20);
        animation.set_animation_fps(10.0); // 10fps for artifact animation
        
        Some(Self {
            position: Vec2::ZERO,
            animation,
            active: true,
            collected: false,
            collection_timer: 0.0,
            collection_scale: 1.0,
            power_reward: 3,  // Artifacts give less power than gems
            health_reward: 2, // And less health
        })
    }
}

impl Collectible for Artifact {
    fn is_collected(&self) -> bool {
        self.collected
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn collect(&mut self) {
        if !self.collected && self.active {
            self.collected = true;
            self.collection_timer = 0.0;
            self.collection_scale = 1.0;
        }
    }

    fn update(&mut self, dt: f32) -> bool {
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

    fn draw(&self) {
        if !self.active {
            return;
        }

        let scale = if self.collected { self.collection_scale } else { 1.0 };
                
        // Use our standard animation draw method
        self.animation.draw(
            self.position,
            false, // never flip artifacts
            Vec2::new(scale, scale),
            WHITE,
        );

        // Keep debug bounding box
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            20.0,  // Fixed artifact size (updated from 16.0)
            20.0,
            2.0,
            PURPLE, // Different color from gems
        );
    }

    fn bounds(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            20.0,  // Fixed artifact size (updated from 16.0)
            20.0,
        )
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn get_rewards(&self) -> (u8, u8) {
        (self.health_reward, self.power_reward)
    }
}