use macroquad::prelude::*;

/// Common trait for collectible items like Gems and Artifacts
pub trait Collectible {
    fn is_collected(&self) -> bool;
    fn is_active(&self) -> bool;
    fn collect(&mut self);
    fn update(&mut self, dt: f32) -> bool;
    fn draw(&self);
    fn bounds(&self) -> Rect;
    fn position(&self) -> Vec2;
    fn get_rewards(&self) -> (u8, u8);  // Returns (health, power)
}