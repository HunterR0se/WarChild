use crate::effects::{Animation, AnimationState};
use super::types::EnemyType;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct AnimationLoader;

impl AnimationLoader {
    pub async fn load_animations(enemy_type: &EnemyType) -> Option<HashMap<AnimationState, Animation>> {
        let mut animations = HashMap::new();
        
        // Get the variant configuration to access asset path and abilities
        let variant = match enemy_type {
            EnemyType::Man(v) => v.get_variant(),
            EnemyType::Ghost(v) => v.get_variant(),
            EnemyType::Skeleton(v) => v.get_variant(),
            EnemyType::Werewolf(v) => v.get_variant(),
            EnemyType::Witch(v) => v.get_variant(),
            EnemyType::Demon(v) => v.get_variant(),
            EnemyType::Goblin(v) => v.get_variant(),
            EnemyType::Hellhound(v) => v.get_variant(),
            EnemyType::Dwarf(v) => v.get_variant(),
            EnemyType::Golem(v) => v.get_variant(),
            EnemyType::Gorgon(v) => v.get_variant(),
            EnemyType::Minotaur(v) => v.get_variant(),
            EnemyType::Mutant(v) => v.get_variant(),
            EnemyType::Orc(v) => v.get_variant(),
            EnemyType::Priest(v) => v.get_variant(),
            EnemyType::Pyromancer(v) => v.get_variant(),
            EnemyType::Samurai(v) => v.get_variant(),
            EnemyType::Tengu(v) => v.get_variant(),
            EnemyType::Zombie(v) => v.get_variant(),
        };

        // Base animations that all enemies have
        let base_states = [
            (AnimationState::Idle, "Idle"),
            (AnimationState::Walking, "Walk"),
            (AnimationState::Hurt, "Hurt"),
            (AnimationState::Dead, "Dead"),
            (AnimationState::Attacking1, "Attack_1"),
        ];

        // Load base animations
        for (state, anim_name) in base_states.iter() {
            let path = format!("{}/{}.png", variant.asset_path, anim_name);
            if let Ok(texture) = load_texture(&path).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                
                // Set animation properties based on state
                match state {
                    AnimationState::Dead => {
                        animation.set_looping(false);
                        animation.set_animation_fps(8.0);
                    }
                    AnimationState::Attacking1 => {
                        animation.set_looping(false);
                        animation.set_animation_fps(12.0);
                    }
                    _ => {
                        animation.set_looping(true);
                        animation.set_animation_fps(8.0);
                    }
                }
                
                animations.insert(*state, animation);
            }
        }

        // Optional animations based on abilities
        if variant.abilities.has_special {
            if let Ok(texture) = load_texture(&format!("{}/Special.png", variant.asset_path)).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(false);
                animation.set_animation_fps(12.0);
                animations.insert(AnimationState::Special, animation);
            }
        }

        if variant.abilities.has_dot {
            if let Ok(texture) = load_texture(&format!("{}/Poison.png", variant.asset_path)).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(true);
                animation.set_animation_fps(8.0);
                animations.insert(AnimationState::Poison, animation);
            }
        }

        if variant.abilities.has_shoot {
            if let Ok(texture) = load_texture(&format!("{}/Shoot.png", variant.asset_path)).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(false);
                animation.set_animation_fps(12.0);
                animations.insert(AnimationState::Shoot, animation);
            }
        }

        if variant.abilities.has_jump {
            if let Ok(texture) = load_texture(&format!("{}/Jump.png", variant.asset_path)).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(false);
                animation.set_animation_fps(12.0);
                animations.insert(AnimationState::Jumping, animation);
            }
        }

        if variant.abilities.has_shield {
            if let Ok(texture) = load_texture(&format!("{}/Shield.png", variant.asset_path)).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(false);
                animation.set_animation_fps(12.0);
                animations.insert(AnimationState::Shoot, animation);
            }
        }

        // Additional attack animations if enemy has them
        for i in 2..=4 {
            let path = format!("{}/Attack_{}.png", variant.asset_path, i);
            if let Ok(texture) = load_texture(&path).await {
                let mut animation = Animation::new_with_texture(texture, 128, 128);
                animation.set_looping(false);
                animation.set_animation_fps(12.0);
                animations.insert(match i {
                    2 => AnimationState::Attacking2,
                    3 => AnimationState::Attacking3,
                    4 => AnimationState::Attacking4,
                    _ => unreachable!(),
                }, animation);
            }
        }

        if animations.is_empty() {
            None
        } else {
            Some(animations)
        }
    }
}