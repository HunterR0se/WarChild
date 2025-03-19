use macroquad::prelude::*;
use crate::effects::{Animation, AnimationState, Projectile, ProjectileOwner};
use crate::core::power::Power;

pub struct PlayerCombat {
    current_attack: u32,
    last_attack_time: f32,
    is_shooting: bool,
}

impl PlayerCombat {
    pub fn new() -> Self {
        Self {
            current_attack: 1,
            last_attack_time: 0.0,  
            is_shooting: false,
        }
    }

    pub fn reset(&mut self) {
        self.current_attack = 1;
        self.last_attack_time = 0.0;
        self.is_shooting = false;
    }

    #[allow(dead_code)]
    pub fn get_is_shooting(&self) -> bool {
        self.is_shooting
    }

    #[allow(dead_code)]
    pub fn set_shooting(&mut self, shooting: bool) {
        self.is_shooting = shooting;
    }

    #[allow(dead_code)]
    pub fn try_shoot(&mut self, power: &mut Power, player_id: usize, pos: Vec2, facing_left: bool, animation: &Animation) -> Option<Projectile> {
        // Only create projectile at the end of shooting animation
        if !self.is_shooting {
            // Initial shoot request - check power but don't consume yet
            if power.current_power >= 3.0 {
                self.is_shooting = true;
                None // Don't create projectile yet
            } else {
                None
            }
        } else if animation.is_in_state(&AnimationState::Shoot) && animation.is_finished() {
            // Animation finished - consume power and create projectile
            self.is_shooting = false; // Reset shooting state
            
            // Now consume the power when actually creating the arrow
            power.use_power(3.0);
            
            // Create projectile slightly in front of player
            let arrow_offset = if facing_left { -20.0 } else { 20.0 };
            let arrow_pos = Vec2::new(pos.x + arrow_offset, pos.y + 32.0); // Adjust Y to be at "chest" height
            
            // Create projectile
            Some(Projectile::new(
                arrow_pos,
                facing_left,
                ProjectileOwner::Player,
                player_id,
                8.0  // Base damage of 8 points
            ))
        } else {
            None
        }
    }

    pub fn try_attack(
        &mut self,
        power: &mut Power,
        power_cost: f32,
        current_time: f32,
    ) -> Option<AnimationState> {
        // Calculate power cost based on attack number
        let attack_power_cost = power_cost * self.current_attack as f32;

        if power.current_power >= attack_power_cost {
            // We have enough power - determine attack state
            let attack_state = match self.current_attack {
                1 => AnimationState::Attacking1,
                2 => AnimationState::Attacking2,
                3 => AnimationState::Attacking3,
                4 => AnimationState::Attacking4,
                _ => AnimationState::Attacking1,
            };

            // Consume power 
            power.use_power(attack_power_cost);

            // Update last attack time and set attack number
            self.last_attack_time = current_time;
            // After Attack4, next will be Attack1
            self.current_attack = if self.current_attack >= 4 {
                1  // Reset to first attack
            } else {
                self.current_attack + 1  // Progress to next
            };

            Some(attack_state)
        } else {
            None // Not enough power
        }
    }

    pub fn try_roll(
        &mut self,
        power: &mut Power,
        current_time: f32,
    ) -> Option<AnimationState> {
        const ROLL_POWER_COST: f32 = 0.5;

        // Check if we have enough power to roll
        if power.current_power >= ROLL_POWER_COST {
            // We have enough power - consume it
            power.use_power(ROLL_POWER_COST);
            self.last_attack_time = current_time; // Use last_attack to prevent immediate actions after roll

            // Return rolling state
            Some(AnimationState::Rolling)
        } else {
            None // Not enough power
        }
    }

    pub fn is_attacking(&self, animation: &Animation) -> bool {
        animation.is_in_state(&AnimationState::Attacking1) ||
        animation.is_in_state(&AnimationState::Attacking2) ||
        animation.is_in_state(&AnimationState::Attacking3) ||
        animation.is_in_state(&AnimationState::Attacking4)
    }

    pub fn is_rolling(&self, animation: &Animation) -> bool {
        animation.is_in_state(&AnimationState::Rolling)
    }
}