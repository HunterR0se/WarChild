use macroquad::prelude::*;
use crate::effects::Animation;

#[derive(Debug, Clone)]
pub struct PoisonEffect {
    pub timer: f32,           // How long poison lasts
    #[allow(dead_code)]
    pub damage_per_sec: f32,  // Damage per second
    pub next_tick: f32,       // When to apply next damage tick
    pub animation: Option<Animation>,  // Poison overlay animation
    #[allow(dead_code)]
    pub source_pos: Vec2,     // Position of enemy that applied the poison
}