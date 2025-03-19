use crate::effects::{Animation, AnimationState};
use crate::physics::Platform;
use crate::effects::PoisonEffect;
use crate::core::health::Health;
use macroquad::prelude::*;
use std::collections::HashMap;
use super::AnimationLoader;
use super::types::variants::{AttackPattern, EnemyType, EnemyVariant};

pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    #[allow(dead_code)]
    pub last_velocity: Vec2,
    #[allow(dead_code)]
    pub direction_change_cooldown: f32,
    pub world_id: Option<usize>,
    pub animations: HashMap<AnimationState, Animation>,
    pub current_state: AnimationState,
    pub facing_left: bool,
    pub health: Health,
    pub on_ground: bool,
    #[allow(dead_code)]
    pub enemy_type: EnemyType,
    pub variant: EnemyVariant,  // NEW: Store the variant configuration
    pub attack_cooldown: f32,
    pub attack_range: f32,
    pub attack_ready: bool,
    pub hit_dealt: bool,
    pub poison_effect: Option<PoisonEffect>,
    #[allow(dead_code)]
    pub area_attack: bool,
    pub current_attack: u8,
    pub max_attacks: u8
}

impl Enemy {
    // Helper method to check if movement in a direction is safe
    fn check_safe_movement(&self, platforms: &[Platform], moving_left: bool) -> bool {
        let check_distance = 20.0; // Look 20px ahead
        
        // Position to check ahead
        let check_x = if moving_left {
            self.position.x - check_distance  // Left edge
        } else {
            self.position.x + self.size.x + check_distance // Right edge
        };
        
        // First check if we're still on current platform
        let mut has_current_platform = false;
        let mut has_landing_platform = false;
        
        for platform in platforms {
            // Check if we're still on current platform
            if check_x >= platform.pos.x && 
               check_x <= platform.pos.x + platform.size.x &&
               self.position.y + self.size.y >= platform.pos.y &&
               self.position.y + self.size.y <= platform.pos.y + 5.0 {
                has_current_platform = true;
                break;
            }
            
            // Check if there's ANY platform below to land on (no depth limit)
            if check_x >= platform.pos.x &&
               check_x <= platform.pos.x + platform.size.x &&
               platform.pos.y > self.position.y + self.size.y { // Just needs to be below us
                has_landing_platform = true;
                break;
            }
        }
        
        has_current_platform || has_landing_platform // Allow movement if either condition is true
    }

    pub async fn new(enemy_type: EnemyType, spawn_pos: Vec2) -> Option<Self> {
        // Load animations first
        let animations = AnimationLoader::load_animations(&enemy_type).await?;

        // Get the variant configuration
        let variant = match &enemy_type {
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

        // Set up health based on variant's base HP
        let health_amount = variant.base_hp as f32; // Use direct HP values
        let mut health = Health::new();
        health.max_health = health_amount;
        health.current_health = health_amount;
        health.load_textures_with_paths(
            "",
            "assets/Gui/Bars/LoadingBar_4_Fill_Green.png",
            "",
        ).await;

        // Get max attacks based on abilities
        let max_attacks = if variant.abilities.has_special { 2 } else { 1 };

        Some(Self {
            position: spawn_pos,
            size: Vec2::new(32.0, 66.0),
            velocity: Vec2::ZERO,
            last_velocity: Vec2::ZERO,
            direction_change_cooldown: 0.0,
            world_id: None,
            animations,
            current_state: AnimationState::Idle,
            facing_left: false,
            health,
            on_ground: false,
            enemy_type,
            variant,
            attack_cooldown: 0.0,
            attack_range: 60.0,
            attack_ready: true,
            hit_dealt: false,
            poison_effect: None,
            area_attack: false,
            current_attack: 1,
            max_attacks,
        })
    }

    pub fn update(
        &mut self,
        dt: f32,
        player_pos: Vec2,
        platforms: &[Platform],
    ) -> Option<(usize, bool)> {
        // Only stop if player is directly above/below us (within our width)
        let player_in_column = player_pos.x + 32.0 >= self.position.x && 
                              player_pos.x <= self.position.x + self.size.x;
        
        // Skip movement freeze if we're in an attack animation
        if !matches!(self.current_state, 
            AnimationState::Attacking1 | 
            AnimationState::Attacking2 | 
            AnimationState::Attacking3 | 
            AnimationState::Attacking4) {
            if player_in_column && self.on_ground {
                self.velocity = Vec2::ZERO;
                self.current_state = AnimationState::Idle;
                return None;
            }
        }

        // Handle poison effect first
        if let Some(poison) = &mut self.poison_effect {
            poison.timer -= dt;
            poison.next_tick -= dt;

            // Update poison animation if there is one
            if let Some(anim) = &mut poison.animation {
                anim.update(dt);
            }

            // Check if it's time for poison damage
            if poison.next_tick <= 0.0 {
                poison.next_tick = 1.0; // Reset tick timer
                // Return signal to deal poison damage
                return Some((0, false)); // (target_id, is_enemy_attack)
            }

            // Check if timer expired
            if poison.timer <= 0.0 {
                self.poison_effect = None;
            }
        }

        // Update animation first - get reference to current animation
        let current_state = self.current_state.clone();
        let animation_complete = {
            if let Some(anim) = self.animations.get_mut(&current_state) {
                anim.update(dt);
                anim.is_finished()
            } else {
                false
            }
        };

        // If we're dead, stop all updates
        if self.current_state == AnimationState::Dead {
            self.velocity = Vec2::ZERO; // Ensure no movement while dead
            return None; // No updates while dead
        }

        // Calculate distance to player
        let distance_to_player = (player_pos - self.position).length();
        let in_attack_range = distance_to_player <= self.attack_range;

        // Handle attack animations and transitions
        if matches!(self.current_state, 
            AnimationState::Attacking1 | 
            AnimationState::Attacking2 | 
            AnimationState::Attacking3 | 
            AnimationState::Attacking4 | 
            AnimationState::Special)
        {
            // Deal damage on appropriate frame based on enemy type
            if let Some(anim) = self.animations.get(&self.current_state) {
                let damage_frame = 3; // All enemies damage on 3rd frame for now

                if anim.current_frame() == damage_frame && !self.hit_dealt {
                    self.hit_dealt = true;
                    
                    // Check if this is a special attack that should apply poison
                    let should_apply_poison = self.current_state == AnimationState::Special 
                        && self.variant.abilities.has_dot;

                    if should_apply_poison {
                        // Create poison effect
                        let mut poison_effect = PoisonEffect {
                            timer: 5.0,         // 5 seconds of poison
                            damage_per_sec: 1.0,  // 1 damage per second
                            next_tick: 1.0,     // Damage every 1 second
                            animation: None,     // Will set below if available
                            source_pos: self.position, // Current enemy position
                        };

                        // Try to load poison animation if enemy has one
                        if let Some(poison_anim) = self.animations.get(&AnimationState::Poison) {
                            poison_effect.animation = Some(poison_anim.clone());
                        }

                        self.poison_effect = Some(poison_effect);
                    }

                    return Some((0, true)); // Deal damage
                }
            }

            // Handle animation completion
            if animation_complete {
                match self.current_state {
                    AnimationState::Attacking1 |
                    AnimationState::Attacking2 |
                    AnimationState::Attacking3 |
                    AnimationState::Attacking4 |
                    AnimationState::Special => {
                        // Remember which attack we just finished
                        let completed_attack = self.current_attack;
                        
                        // Go to idle and set cooldown
                        self.current_state = AnimationState::Idle;
                        if let Some(idle_anim) = self.animations.get_mut(&AnimationState::Idle) {
                            idle_anim.reset();
                        }

                        // Handle next attack based on pattern
                        match self.variant.abilities.pattern {
                            AttackPattern::Cyclic => {
                                // For cyclic pattern, just use a short cooldown
                                self.attack_cooldown = 0.5;
                            },
                            _ => {
                                // For basic pattern, check if we finished sequence
                                if completed_attack >= self.max_attacks {
                                    self.attack_cooldown = 1.0; // Longer cooldown after full sequence
                                    self.current_attack = 1; // Reset to start
                                } else {
                                    // Set up for next attack after cooldown
                                    self.current_attack = completed_attack + 1;
                                    self.attack_cooldown = 0.5; // Short cooldown between attacks
                                }
                            }
                        }
                        
                        // Always reset attack flags
                        self.attack_ready = false;
                        self.hit_dealt = false;
                    },
                    _ => {}
                }
            }

            self.velocity = Vec2::ZERO; // Stop movement during attack animations
            return None;
        }

        // Update attack cooldown
        if !self.attack_ready {
            self.attack_cooldown -= dt;
            if self.attack_cooldown <= 0.0 {
                self.attack_ready = true;
            }
        }

        // Check attack conditions
        if self.attack_ready && in_attack_range {
            // Check if we can use special attack
            let can_use_special = self.variant.abilities.has_special &&
                (self.current_attack == 1 || !matches!(self.current_state, 
                    AnimationState::Attacking1 | 
                    AnimationState::Attacking2 | 
                    AnimationState::Attacking3 | 
                    AnimationState::Attacking4));

            // Random chance for Special if available and we're able to use it
            if can_use_special && 
               rand::gen_range(0.0, 1.0) < 0.4 && // 40% chance for testing
               self.animations.contains_key(&AnimationState::Special) {
                self.current_state = AnimationState::Special;
                if let Some(special_anim) = self.animations.get_mut(&AnimationState::Special) {
                    special_anim.reset();
                }
            } else {
                // Start the appropriate attack based on current_attack counter and pattern
                let next_state = {
                    // Handle attack pattern
                    if matches!(self.variant.abilities.pattern, AttackPattern::Cyclic) {
                        // Cycle through available attacks
                        self.current_attack = if self.current_attack >= self.max_attacks {
                            1  // Back to first attack
                        } else {
                            self.current_attack + 1  // Next attack in sequence
                        };
                    }
                    // Convert current_attack to animation state
                    match self.current_attack {
                        1 => AnimationState::Attacking1,
                        2 => AnimationState::Attacking2,
                        3 => AnimationState::Attacking3,
                        4 => AnimationState::Attacking4,
                        _ => panic!("Invalid attack number: {}", self.current_attack),
                    }
                };

                // Only start attack if we have this animation
                if self.animations.contains_key(&next_state) {
                    self.current_state = next_state;
                    if let Some(attack_anim) = self.animations.get_mut(&self.current_state) {
                        attack_anim.reset();
                    }
                }
            }
            
            self.hit_dealt = false;
            return None;
        }

        // Movement logic - only move if not in attack range
        let movement_speed = 100.0; // Slower than player
        if distance_to_player > self.attack_range {
            // Move towards player if on ground
            if self.on_ground {
                self.facing_left = self.position.x > player_pos.x;
                // Check if movement is safe before applying velocity
                if self.check_safe_movement(platforms, self.facing_left) {
                    self.velocity.x = if self.facing_left {
                        -movement_speed
                    } else {
                        movement_speed
                    };
                } else {
                    self.velocity.x = 0.0; // Stop at edge if no safe path
                }
            }
        } else {
            self.velocity.x = 0.0; // Stay still when in range
        }

        // Apply gravity if not on ground
        if !self.on_ground {
            self.velocity.y += 1000.0 * dt; // Same gravity as player
        }

        // Calculate next position
        let mut next_pos = self.position + self.velocity * dt;

        // Reset ground state unless we confirm we're on a platform
        self.on_ground = false;

        // Check platform collisions
        for platform in platforms {
            let next_bottom = next_pos.y + self.size.y;
            let next_right = next_pos.x + self.size.x;

            // First priority: Landing on top of platform
            if next_bottom >= platform.pos.y && // Will be at or below platform top
               self.position.y + self.size.y <= platform.pos.y + 5.0 && // Currently at or slightly above platform
               next_right > platform.pos.x &&
               next_pos.x < platform.pos.x + platform.size.x
            {
                // Horizontally overlapping
                next_pos.y = platform.pos.y - self.size.y;
                self.velocity.y = 0.0;
                self.on_ground = true;
                break; // Stop checking once we've landed
            }
            // Side collisions - only check if we're not landing
            else if next_right > platform.pos.x
                && next_pos.x < platform.pos.x + platform.size.x
                && next_pos.y + self.size.y > platform.pos.y
                && next_pos.y < platform.pos.y + platform.size.y
            {
                if self.velocity.x > 0.0 {
                    next_pos.x = platform.pos.x - self.size.x;
                } else if self.velocity.x < 0.0 {
                    next_pos.x = platform.pos.x + platform.size.x;
                }
                self.velocity.x = 0.0;
            }
        }

        // Update position
        self.position = next_pos;

        // Update animation state based on movement
        if self.current_state != AnimationState::Hurt && self.current_state != AnimationState::Dead
        {
            let new_state = if self.velocity.x == 0.0 {
                AnimationState::Idle
            } else {
                AnimationState::Walking
            };

            if self.current_state != new_state {
                self.current_state = new_state;
            }
        }

        None // No damage this frame
    }

    pub fn draw(&self, player_pos: Vec2) {
        // Draw collision box (blue outline)
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            2.0,
            BLUE,
        );

        // Draw hitbox (red outline) - matches collision box for now
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            1.0,
            RED,
        );

        // Draw health bar if not in death animation
        if self.current_state != AnimationState::Dead {
            // Draw health bar above enemy - EXACT same width as enemy
            let health_pos = Vec2::new(
                self.position.x,        // Exactly at enemy's left edge
                self.position.y - 22.0, // 22px above enemy
            );
            self.health.draw_health_bar_with_size(health_pos, Vec2::new(32.0, 4.0));
        }

        // Calculate distance to player for range checks
        let distance_to_player = (player_pos - self.position).length();
        let in_attack_range = distance_to_player <= self.attack_range;

        // Only draw attack range indicator if alive and in range
        if in_attack_range && self.current_state != AnimationState::Dead {
            // Draw attack range indicator (semi-transparent yellow box)
            let range_pos = if self.facing_left {
                Vec2::new(self.position.x - self.attack_range, self.position.y)
            } else {
                Vec2::new(self.position.x + self.size.x, self.position.y)
            };
            draw_rectangle(
                range_pos.x,
                range_pos.y,
                self.attack_range,
                self.size.y,
                Color::new(1.0, 1.0, 0.0, 0.2), // Yellow with 20% opacity
            );
        }

        if let Some(anim) = self.animations.get(&self.current_state) {
            // Use same sprite offset logic as player for consistent visuals
            let sprite_pos = Vec2::new(
                self.position.x - 48.0,           // Center 32px box in 128px frame
                self.position.y - (128.0 - 66.0), // Align feet with collision box
            );

            // Handle death animation fade
            let color = if self.current_state == AnimationState::Dead {
                if anim.current_frame() == 4 {
                    // Only fade the last frame
                    Color::new(1.0, 1.0, 1.0, 0.5) // 50% opacity on last frame
                } else {
                    WHITE // Keep normal color for all other frames
                }
            } else {
                WHITE
            };

            anim.draw(sprite_pos, self.facing_left, Vec2::ONE, color);
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        // Don't take damage if already dead
        if self.current_state == AnimationState::Dead {
            return;
        }

        self.health.take_damage(amount);

        // If health reaches 0, transition to Dead state and reset animation
        if self.health.current_health <= 0.0 {
            self.current_state = AnimationState::Dead;
            if let Some(dead_anim) = self.animations.get_mut(&AnimationState::Dead) {
                dead_anim.reset();
                dead_anim.set_looping(false);
                dead_anim.set_animation_fps(8.0);
            }
            // Clear combat state
            self.attack_ready = false;
            self.attack_cooldown = 0.0;
            self.velocity = Vec2::ZERO;
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    #[allow(dead_code)]
    pub fn is_dead(&self) -> bool {
        if self.health.current_health <= 0.0 {
            if self.current_state == AnimationState::Dead {
                if let Some(anim) = self.animations.get(&AnimationState::Dead) {
                    return anim.is_finished();
                }
            }
            return false; // Still dying
        }
        false // Not dead yet
    }
}