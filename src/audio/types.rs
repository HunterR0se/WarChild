//! Type definitions for the game's audio system

/// Main categories of sound effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundCategory {
    /// Character-specific sounds (death screams)
    Character,
    /// Combat sounds (weapons, impacts)
    Combat,
    /// Special effects (explosions, misc)
    Effect,
    /// Environmental sounds (alarms, doors)
    Environment,
    /// Movement sounds (footsteps, jumps)
    Movement,
    /// Item-related sounds (pickups, powerups)
    Item,
    /// User interface sounds (menu, alerts)
    UI,
}

/// Types of pause sounds (in/out variations)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseType {
    /// Pause activation sound
    In,
    /// Pause deactivation sound
    Out,
}