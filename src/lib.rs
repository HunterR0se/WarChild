pub mod core;
pub mod physics;
pub mod effects;
pub mod assets;
pub mod enemies;
pub mod input;
pub mod player;
pub mod objects;
pub mod audio;

pub use enemies::Enemy;
pub use physics::{World, Platform};
pub use player::PlayerAnimation;
pub use audio::AudioSystem;