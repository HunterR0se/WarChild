// Player module - handles all player-specific functionality
//! This module contains all player-related systems including animation,
//! movement, combat, and state management.

mod animation;
mod movement;
mod combat;

pub use animation::PlayerAnimation;
pub use movement::PlayerMovement;
pub use combat::PlayerCombat;