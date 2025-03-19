use crate::core::damage::DamageSystem;
use crate::core::health::Health;
use crate::effects::{AnimationState, PoisonEffect, Projectile, ProjectileState};
use crate::enemies::Enemy;
use macroquad::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Actor {
    pub pos: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub on_ground: bool,
    ground_buffer: f32,        // Time since last being on ground
    pub can_double_jump: bool, // Track if double jump is available
    pub flash_timer: f32,      // For visual effects
    pub facing_left: bool,
    pub health: Option<Health>, // Using our new Health component
    pub power: Option<crate::core::power::Power>, // Using our new Power component
    pub is_player: bool,        // To distinguish player from enemies
    pub poison_effect: Option<PoisonEffect>, // Track poison status effect
    pub is_rolling: bool,       // Track if actor is rolling
    pub last_attack_time: f32,  // Time since last attack for chain resetting
    pub current_attack: u32,    // Current attack in chain (1-4)
    pub is_hanging: bool,       // Track if actor is hanging from edge
    pub hang_point: Option<Vec2>, // Position of edge being hung from
    pub target_pos: Option<Vec2>, // Target position for scripted movement
    pub frame_moved: bool,      // Track if we've moved this frame
    pub in_scripted_movement: bool, // Disable physics during scripted actions
    pub grab_immune: bool,      // Prevent grabbing during certain actions like pull-up
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum CollisionType {
    Walkable,   // Platform top (GREEN)
    Vertical,   // Platform sides (BLUE)
    Connection, // Platform edges (RED)
}

#[derive(Debug)]
pub struct CollisionArea {
    pub bounds: (Vec2, Vec2), // (top-left, bottom-right)
    pub area_type: CollisionType,
}

#[derive(Debug)]
pub struct Platform {
    pub pos: Vec2,
    pub size: Vec2,
    pub is_deadly: bool,
    pub collision_areas: Vec<CollisionArea>,
}

pub struct World {
    pub actors: Vec<(usize, Actor)>, // (id, actor) pairs
    pub platforms: Vec<Platform>,
    gravity: f32,
    damage_system: DamageSystem,
    next_actor_id: usize, // Track next available ID
}

impl World {
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            platforms: Vec::new(),
            gravity: 1000.0,
            damage_system: DamageSystem::new(),
            next_actor_id: 1, // Start from 1, 0 reserved for player
        }
    }

    pub fn add_actor(&mut self, pos: Vec2, size: Vec2, force_id: Option<usize>) -> usize {
        let id = force_id.unwrap_or_else(|| {
            let next_id = self.next_actor_id;
            self.next_actor_id += 1;
            next_id
        });

        let actor = Actor {
            pos,
            size,
            velocity: Vec2::ZERO,
            on_ground: false,
            ground_buffer: 0.0,
            can_double_jump: true,
            flash_timer: 0.0,
            facing_left: false,
            health: None,
            power: None,
            is_player: false,
            poison_effect: None,         // Start with no poison effect
            is_rolling: false,           // Start not rolling
            last_attack_time: 0.0,       // Initialize attack timer
            current_attack: 1,           // Start with Attack_1
            is_hanging: false,           // Start not hanging
            hang_point: None,            // No hang point initially
            target_pos: None,            // No target position initially
            frame_moved: false,          // Start not moved
            in_scripted_movement: false, // Start with physics enabled
            grab_immune: false,          // Start not immune to grabbing
        };
        self.actors.push((id, actor));
        id
    }

    pub fn get_actor(&self, actor_id: usize) -> Option<&Actor> {
        self.actors
            .iter()
            .find(|(id, _)| *id == actor_id)
            .map(|(_, actor)| actor)
    }

    pub fn get_actor_mut(&mut self, actor_id: usize) -> Option<&mut Actor> {
        self.actors
            .iter_mut()
            .find(|(id, _)| *id == actor_id)
            .map(|(_, actor)| actor)
    }

    pub fn add_platform(&mut self, pos: Vec2, size: Vec2) {
        let platform = Platform {
            pos,
            size,
            is_deadly: false,
            collision_areas: vec![
                // Walk area (GREEN)
                CollisionArea {
                    bounds: (Vec2::ZERO, Vec2::new(size.x, size.y)),
                    area_type: CollisionType::Walkable,
                },
                // Left edge (RED) - Wider area
                CollisionArea {
                    bounds: (Vec2::ZERO, Vec2::new(8.0, size.y)),
                    area_type: CollisionType::Connection,
                },
                // Right edge (RED) - Wider area
                CollisionArea {
                    bounds: (Vec2::new(size.x - 8.0, 0.0), Vec2::new(size.x, size.y)),
                    area_type: CollisionType::Connection,
                },
            ],
        };
        self.platforms.push(platform);
    }

    #[allow(dead_code)]
    pub fn can_attack(&self, actor_id: usize, _is_special: bool) -> bool {
        if let Some((_, actor)) = self.actors.iter().find(|(id, _)| *id == actor_id) {
            if let Some(power) = &actor.power {
                power.current_power > 0.0
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_power_cost(&self, is_special: bool) -> f32 {
        self.damage_system.get_power_cost(is_special)
    }

    pub fn update(&mut self, dt: f32) {
        // First calculate next positions and update velocities
        let mut next_positions: Vec<(usize, Vec2)> = Vec::new();

        // First pass: Just store positions for scripted actors and physics for others
        for (id, actor) in &mut self.actors {
            // Skip physics for scripted movement only
            if actor.in_scripted_movement {
                next_positions.push((*id, actor.pos));
                continue; // Skip ALL remaining physics/collision
            }

            // For non-scripted actors, do normal physics
            if !actor.on_ground {
                actor.velocity.y += self.gravity * dt;
            }

            // Draw player bounding box if this is the player
            if actor.is_player {
                draw_rectangle_lines(
                    actor.pos.x,
                    actor.pos.y,
                    actor.size.x,
                    actor.size.y,
                    2.0,
                    GREEN,
                );
            }

            // Calculate next position
            let next_pos = actor.pos + actor.velocity * dt;

            // Update ground buffer
            if actor.on_ground {
                actor.ground_buffer = 0.1; // Reset buffer when on ground
            } else {
                actor.ground_buffer = (actor.ground_buffer - dt).max(0.0);
            }

            // Reset ground state unless we confirm we're on a platform
            actor.on_ground = false;

            // Store next position for collision checking
            next_positions.push((*id, next_pos));
        }

        // Collect all actor data for the second pass
        let actor_data: Vec<_> = self
            .actors
            .iter()
            .map(|(id, actor)| (*id, actor.pos, actor.size, actor.is_rolling))
            .collect();

        // Second pass: handle collisions and update positions
        for (current_id, current_next_pos) in &next_positions {
            if let Some((_, current_actor)) =
                self.actors.iter_mut().find(|(id, _)| id == current_id)
            {
                let mut final_pos = *current_next_pos;

                // Check collisions with other actors using collected data
                for (other_id, other_pos, other_size, _other_is_rolling) in &actor_data {
                    if *other_id != *current_id {
                        let next_actor_right = final_pos.x + current_actor.size.x;
                        let next_actor_bottom = final_pos.y + current_actor.size.y;
                        let other_right = other_pos.x + other_size.x;
                        let other_bottom = other_pos.y + other_size.y;

                        // Skip collision if either actor is rolling
                        if current_actor.is_rolling {
                            continue;
                        }
                        // Get other actor's data from collected data earlier
                        for (id, _, _, is_rolling) in &actor_data {
                            if *id == *other_id && *is_rolling {
                                continue;
                            }
                        }

                        // Only block horizontal movement - allow jumping over
                        if next_actor_bottom > other_pos.y && final_pos.y < other_bottom {
                            // Vertical overlap
                            const COLLISION_BUFFER: f32 = 5.0; // Small buffer to prevent sticking

                            // Moving right and would hit other actor
                            if current_actor.velocity.x > 0.0
                                && next_actor_right > other_pos.x
                                && final_pos.x < other_right
                            {
                                final_pos.x = other_pos.x - current_actor.size.x - COLLISION_BUFFER;
                                current_actor.velocity.x = 0.0;
                            }
                            // Moving left and would hit other actor
                            else if current_actor.velocity.x < 0.0
                                && final_pos.x < other_right
                                && next_actor_right > other_pos.x
                            {
                                final_pos.x = other_right + COLLISION_BUFFER;
                                current_actor.velocity.x = 0.0;
                            }
                        }
                    }
                }

                // Check platform collisions
                for platform in &self.platforms {
                    let actor_bottom = final_pos.y + current_actor.size.y;
                    let actor_right = final_pos.x + current_actor.size.x;

                    // If not already hanging, check for edge grab
                    if current_actor.is_player
                        && !current_actor.is_hanging
                        && !current_actor.in_scripted_movement
                    {
                        let current_pos = current_actor.pos;
                        let current_size = current_actor.size;
                        let mut found_grab = false;

                        // Check current platform's connection points only
                        for area in &platform.collision_areas {
                            if matches!(area.area_type, CollisionType::Connection) {
                                // Get world space connection point
                                let _conn_point = platform.pos + area.bounds.0;
                                let platform_top = platform.pos.y;

                                // Create grab zone - smaller, more precise detection
                                let grab_zone = Rect::new(
                                    if area.bounds.0.x == 0.0 {
                                        platform.pos.x - 10.0 // Left edge grab zone
                                    } else {
                                        platform.pos.x + platform.size.x // Right edge grab zone
                                    },
                                    platform.pos.y, // Start at platform top
                                    10.0,           // Grab zone width
                                    20.0,           // Only check top portion
                                );

                                // Show grab zone
                                draw_rectangle_lines(
                                    grab_zone.x,
                                    grab_zone.y,
                                    grab_zone.w,
                                    grab_zone.h,
                                    2.0,
                                    PURPLE,
                                );

                                // Get check point based on facing direction
                                let side_point = if current_actor.facing_left {
                                    current_pos.x // Left side of actor
                                } else {
                                    current_pos.x + current_size.x // Right side of actor
                                };

                                // Show check point
                                draw_circle(side_point, current_pos.y, 3.0, RED);

                                // Simple box collision check
                                if side_point >= grab_zone.x &&
                                   side_point <= grab_zone.x + grab_zone.w &&
                                   current_pos.y >= grab_zone.y &&
                                   current_pos.y <= grab_zone.y + grab_zone.h &&
                                   !current_actor.grab_immune && // Only grab if not immune
                                   current_actor.flash_timer <= 0.0
                                {
                                    // Also check flash timer

                                    current_actor.is_hanging = true;

                                    // Store the correct edge position
                                    let edge_x = if area.bounds.0.x == 0.0 {
                                        platform.pos.x // Left edge of platform
                                    } else {
                                        platform.pos.x + platform.size.x // Right edge of platform
                                    };

                                    // Initial position calculation with visual adjustments
                                    current_actor.pos.y = platform_top + 12.0; // Move down 12 pixels
                                    current_actor.pos.x = if current_actor.facing_left {
                                        edge_x - 3.0 // Move 3 pixels into platform when grabbing left edge
                                    } else {
                                        edge_x - current_actor.size.x + 3.0 // Move 3 pixels into platform when grabbing right edge
                                    };

                                    current_actor.hang_point =
                                        Some(Vec2::new(edge_x, platform_top));
                                    current_actor.velocity = Vec2::ZERO;
                                    found_grab = true;
                                    break;
                                }
                            }
                        }
                        if found_grab {
                            continue; // Skip other collision checks
                        }
                    }

                    // First priority: Landing on top of platform
                    if actor_bottom >= platform.pos.y && // Will be at or below platform top
                       current_actor.pos.y + current_actor.size.y <= platform.pos.y + 1.0 && // Currently at or slightly above platform
                       actor_right > platform.pos.x &&
                       final_pos.x < platform.pos.x + platform.size.x
                    {
                        // Horizontally overlapping

                        // Check for deadly platform collision
                        if platform.is_deadly && current_actor.is_player {
                            if let Some(health) = &mut current_actor.health {
                                health.current_health = 0.0; // Kill player instantly
                            }
                        }

                        // Land on platform
                        final_pos.y = platform.pos.y - current_actor.size.y;
                        current_actor.velocity.y = 0.0;
                        current_actor.on_ground = true;
                        current_actor.can_double_jump = true; // Reset double jump availability
                                                              // Only reset grab immunity if not falling (downward velocity)
                    }
                    // Check hitting platform from below
                    else if current_actor.velocity.y < 0.0 && // Moving upward
                         final_pos.y < platform.pos.y + platform.size.y && // Will be inside platform
                         current_actor.pos.y >= platform.pos.y + platform.size.y && // Currently below platform
                         actor_right > platform.pos.x &&
                         final_pos.x < platform.pos.x + platform.size.x
                    // Horizontally overlapping
                    {
                        // Stop upward momentum and push down
                        final_pos.y = platform.pos.y + platform.size.y;
                        current_actor.velocity.y = 0.0;
                    }
                    // Side collisions
                    else if actor_right > platform.pos.x
                        && final_pos.x < platform.pos.x + platform.size.x
                        && final_pos.y + current_actor.size.y > platform.pos.y
                        && final_pos.y < platform.pos.y + platform.size.y
                    {
                        // Check for deadly platform collision
                        if platform.is_deadly && current_actor.is_player {
                            if let Some(health) = &mut current_actor.health {
                                health.current_health = 0.0; // Kill player instantly
                            }
                        }

                        if current_actor.velocity.x > 0.0 {
                            final_pos.x = platform.pos.x - current_actor.size.x;
                        } else if current_actor.velocity.x < 0.0 {
                            final_pos.x = platform.pos.x + platform.size.x;
                        }
                        current_actor.velocity.x = 0.0;
                    }
                }

                // Update position
                current_actor.pos = final_pos;

                // Update flash timer
                if current_actor.flash_timer > 0.0 {
                    current_actor.flash_timer = (current_actor.flash_timer - dt).max(0.0);
                }

                // Check if fallen below screen
                if current_actor.is_player && current_actor.pos.y > screen_height() {
                    if let Some(health) = &mut current_actor.health {
                        health.current_health = 0.0; // Kill player
                    }
                }
            }
        }
    }

    pub fn move_actor(&mut self, actor_id: usize, movement: Vec2) {
        if let Some((_, actor)) = self.actors.iter_mut().find(|(id, _)| *id == actor_id) {
            // COMPLETELY skip ANY movement during scripted sequences
            if actor.in_scripted_movement {
                return; // Do not apply ANY movement at all
            }
            // For player with health, only allow movement if alive
            if actor.is_player {
                if let Some(health) = &actor.health {
                    if health.current_health <= 0.0 {
                        actor.velocity = Vec2::ZERO;
                        return;
                    }
                }
            }

            // Handle hanging state
            if actor.is_hanging {
                if movement.y < 0.0 {
                    // Up pressed - initiate pull-up
                    actor.is_hanging = false;
                    actor.hang_point = None;
                    actor.velocity = Vec2::ZERO;
                    actor.grab_immune = true; // Prevent immediate jump after pull-up
                    actor.velocity = Vec2::ZERO;
                } else if movement.y > 0.0 {
                    // Down pressed - drop from platform
                    actor.is_hanging = false;
                    actor.hang_point = None;
                    actor.velocity = Vec2::new(0.0, 100.0); // Drop straight down
                } else {
                    // No vertical input - maintain hang
                    if let Some(hang_point) = actor.hang_point {
                        // Lock position at hang point with visual adjustments
                        actor.velocity = Vec2::ZERO;
                        actor.pos.y = hang_point.y + 12.0; // Move down 12 pixels
                        actor.pos.x = if actor.facing_left {
                            hang_point.x - 3.0 // Move 3 pixels into platform when on left edge
                        } else {
                            hang_point.x - actor.size.x + 3.0 // Move 3 pixels into platform when on right edge
                        };
                        return;
                    }
                }
            } else {
                // Apply movement for non-hanging state
                actor.velocity.x = movement.x;

                // Handle jumping - only allow if requested and able
                if movement.y < 0.0 && !actor.grab_immune {
                    // Don't allow jump during grab immunity
                    actor.velocity.y = movement.y;
                    actor.on_ground = false;
                }

                // Keep vertical velocity when poisoned to maintain gravity
                if actor.poison_effect.is_some() {
                    if movement.y == 0.0 {
                        actor.velocity.y = actor.velocity.y.min(500.0); // Cap downward speed
                    }
                }
            }
        }
    }

    pub fn can_regular_jump(&self, actor_id: usize) -> bool {
        if let Some((_, actor)) = self.actors.iter().find(|(id, _)| *id == actor_id) {
            actor.on_ground || actor.ground_buffer > 0.0
        } else {
            false
        }
    }

    pub fn can_double_jump(&self, actor_id: usize) -> bool {
        if let Some((_, actor)) = self.actors.iter().find(|(id, _)| *id == actor_id) {
            !actor.on_ground && actor.can_double_jump
        } else {
            false
        }
    }

    pub fn remove_actor(&mut self, actor_id: usize) {
        // Removing actor from world - just filter out the ID
        self.actors.retain(|(id, _)| *id != actor_id);
    }

    pub fn get_actor_position(&self, actor_id: usize) -> Option<Vec2> {
        self.actors
            .iter()
            .find(|(id, _)| *id == actor_id)
            .map(|(_, actor)| actor.pos)
    }

    pub fn get_actor_size(&self, actor_id: usize) -> Option<Vec2> {
        self.actors
            .iter()
            .find(|(id, _)| *id == actor_id)
            .map(|(_, actor)| actor.size)
    }

    pub fn get_leftmost_platform_top(&self) -> Option<Vec2> {
        self.platforms
            .iter()
            .min_by(|a, b| a.pos.x.partial_cmp(&b.pos.x).unwrap())
            .map(|platform| platform.pos)
    }

    pub fn apply_damage(
        &mut self,
        attacker_id: usize,
        target_id: usize,
        is_enemy_attack: bool,
        active_enemies: &mut Vec<Enemy>,
        projectiles: &Vec<Projectile>,
    ) {
        // Calculate damage amount first
        let applied_damage = if is_enemy_attack {
            if let Some((_, attacker)) = self.actors.iter().find(|(id, _)| *id == attacker_id) {
                if let Some(health) = &attacker.health {
                    let max_damage = health.max_health / 2.0; // Half of max health
                    let min_damage = health.max_health / 4.0; // Quarter of max health
                    rand::gen_range(min_damage, max_damage)
                } else {
                    1.0 // Fallback if no health component
                }
            } else {
                1.0 // Fallback if no attacker
            }
        } else {
            // Check if this is a projectile based on attacker's actor status
            if !self.actors.iter().any(|(id, _)| *id == attacker_id) {
                // For projectiles, find the ACTIVE projectile that matches this attacker_id
                if let Some(projectile) = projectiles
                    .iter()
                    .find(|p| p.owner_id == attacker_id && p.get_state() == ProjectileState::Active)
                {
                    println!(
                        "Found matching projectile with damage: {}",
                        projectile.damage
                    );
                    projectile.damage
                } else {
                    // This shouldn't happen - we should always find the projectile
                    println!(
                        "WARNING: Could not find projectile for attacker {}",
                        attacker_id
                    );
                    rand::gen_range(5.0, 10.0) * 20.0
                }
            } else {
                // Normal player attacks or poison damage
                if let Some(enemy) = active_enemies
                    .iter()
                    .find(|e| e.world_id == Some(attacker_id))
                {
                    self.damage_system
                        .calculate_attack_damage(&enemy.current_state)
                } else {
                    self.damage_system
                        .calculate_attack_damage(&AnimationState::Attacking1) // Default to basic attack
                }
            }
        };

        // Apply damage and check poison
        if let Some((_, target)) = self.actors.iter_mut().find(|(id, _)| *id == target_id) {
            if target.is_player {
                // For player target, apply damage directly
                if let Some(health) = &mut target.health {
                    health.current_health = (health.current_health - applied_damage).max(0.0);
                    // Don't flash player health bar, only enemies
                }
            } else {
                // For enemy target, update both enemy and actor health once
                for enemy in active_enemies.iter_mut() {
                    if let Some(enemy_id) = enemy.world_id {
                        if enemy_id == target_id {
                            enemy.take_damage(applied_damage);
                            
                            // Then just sync the health value
                            if let Some(health) = &mut target.health {
                                health.current_health = enemy.health.current_health;
                                health.is_taking_damage = true;
                                health.damage_flash_timer = 0.1;
                            }
                            break;
                        }
                    }
                }
            }
            
            // Visual effect for all targets
            target.flash_timer = 0.1;
        }

        // Apply poison effect if this is an enemy attack
        if is_enemy_attack {
            if let Some((_, target)) = self.actors.iter_mut().find(|(id, _)| *id == target_id) {
                if target.is_player {
                    // Check if this is a poison effect from an enemy's Special attack
                    if let Some(enemy) = active_enemies
                        .iter()
                        .find(|e| e.world_id == Some(attacker_id))
                    {
                        if enemy.poison_effect.is_some() {
                            // Transfer poison effect to target
                            if let Some(poison) = &enemy.poison_effect {
                                target.poison_effect = Some(poison.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_attack_hit(&self, attacker_id: usize, attack_bounds: Vec2) -> Option<usize> {
        // Get attacker info
        let (_, attacker) = self.actors.iter().find(|(id, _)| *id == attacker_id)?;
        let facing_left = attacker.facing_left;

        // Calculate attack area based on attacker position and facing direction
        let attack_x = if facing_left {
            attacker.pos.x - attack_bounds.x // Left side of attacker
        } else {
            attacker.pos.x + attacker.size.x // Right side of attacker
        };

        let attack_area = Rect::new(attack_x, attacker.pos.y, attack_bounds.x, attack_bounds.y);

        // Check each other actor for intersection
        for (id, target) in &self.actors {
            if *id == attacker_id {
                continue;
            }

            let target_rect = Rect::new(target.pos.x, target.pos.y, target.size.x, target.size.y);

            if attack_area.overlaps(&target_rect) {
                return Some(*id);
            }
        }

        None
    }
}
