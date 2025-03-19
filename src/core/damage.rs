use rand::Rng;
use crate::effects::AnimationState;

pub struct DamageSystem {
    rng: rand::rngs::ThreadRng,
}

impl DamageSystem {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Calculates damage based on current attack state 
    /// Attack1: 1-3 damage
    /// Attack2: 2-4 damage 
    /// Attack3: 4-6 damage
    /// Attack4: 5-7 damage
    pub fn calculate_attack_damage(&mut self, attack_state: &AnimationState) -> f32 {
        match attack_state {
            AnimationState::Attacking1 => self.rng.gen_range(1.0..=3.0),
            AnimationState::Attacking2 => self.rng.gen_range(2.0..=4.0),
            AnimationState::Attacking3 => self.rng.gen_range(4.0..=6.0),
            AnimationState::Attacking4 => self.rng.gen_range(5.0..=7.0),
            _ => 1.0 // Fallback damage reduced too
        }
    }

    /// Calculates damage dealt by enemy to player (1 to half of enemy's max health)
    #[allow(dead_code)]
    pub fn calculate_enemy_damage(&mut self, enemy_max_health: u32) -> u32 {
        // Enemies now do between 25% to 75% of their max health as damage
        let min_damage = (enemy_max_health as f32 * 0.25) as u32;
        let max_damage = (enemy_max_health as f32 * 0.75) as u32;
        self.rng.gen_range(min_damage..=max_damage)
    }

    /// Calculates special attack damage (2x normal damage)
    #[allow(dead_code)]
    pub fn calculate_special_attack_damage(&mut self, enemy_max_health: u32) -> u32 {
        self.calculate_enemy_damage(enemy_max_health) * 2
    }

    /// Calculates charge attack damage (1.5x normal damage)
    #[allow(dead_code)]
    pub fn calculate_charge_attack_damage(&mut self, enemy_max_health: u32) -> u32 {
        (self.calculate_enemy_damage(enemy_max_health) as f32 * 1.5) as u32
    }

    /// Checks if there's enough power for an attack
    #[allow(dead_code)]
    pub fn can_perform_attack(&self, current_power: u32, power_cost: u32) -> bool {
        current_power >= power_cost
    }

    /// Calculate power cost for different attack types
    pub fn get_power_cost(&self, is_special_attack: bool) -> f32 {  // Changed to f32
        if is_special_attack {
            1.0 // Special abilities cost 1 power point
        } else {
            0.5 // Normal attacks cost 0.5 power points
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_damage_ranges() {
        let mut system = DamageSystem::new();
        for _ in 0..100 {
            let damage1 = system.calculate_attack_damage(&AnimationState::Attacking1);
            assert!(damage1 >= 1.0 && damage1 <= 3.0, "Attack1 damage {} out of range", damage1);

            let damage2 = system.calculate_attack_damage(&AnimationState::Attacking2);
            assert!(damage2 >= 2.0 && damage2 <= 4.0, "Attack2 damage {} out of range", damage2);

            let damage3 = system.calculate_attack_damage(&AnimationState::Attacking3);
            assert!(damage3 >= 4.0 && damage3 <= 6.0, "Attack3 damage {} out of range", damage3);

            let damage4 = system.calculate_attack_damage(&AnimationState::Attacking4);
            assert!(damage4 >= 5.0 && damage4 <= 7.0, "Attack4 damage {} out of range", damage4);
        }
    }

    #[test]
    fn test_enemy_damage_range() {
        let mut system = DamageSystem::new();
        let enemy_max_health = 20;
        for _ in 0..100 {
            let damage = system.calculate_enemy_damage(enemy_max_health);
            assert!(damage >= 1 && damage <= enemy_max_health / 2);
        }
    }

    #[test]
    fn test_special_attack_multiplier() {
        let mut system = DamageSystem::new();
        let enemy_max_health = 20;
        let special_damage = system.calculate_special_attack_damage(enemy_max_health);
        assert!(special_damage >= 2);  // At least 2x minimum damage
    }

    #[test]
    fn test_power_cost() {
        let system = DamageSystem::new();
        assert_eq!(system.get_power_cost(false), 0.5);
        assert_eq!(system.get_power_cost(true), 1.0);
    }

    #[test]
    fn test_can_perform_attack() {
        let system = DamageSystem::new();
        assert!(system.can_perform_attack(100, 1));
        assert!(system.can_perform_attack(2, 2));
        assert!(!system.can_perform_attack(0, 1));
        assert!(!system.can_perform_attack(1, 2));
    }
}