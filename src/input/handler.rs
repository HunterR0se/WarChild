use macroquad::prelude::*;
use crate::effects::AnimationState;
use crate::physics::World;
use crate::player::{PlayerMovement, PlayerCombat, PlayerAnimation};

pub struct InputHandler {
    movement: PlayerMovement,
    combat: PlayerCombat,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            movement: PlayerMovement::new(),
            combat: PlayerCombat::new(),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.movement.reset();
        self.combat.reset();
    }

    pub fn handle_movement(&mut self, world: &mut World, player_id: usize, animation: &mut PlayerAnimation) -> Vec2 {
        let mut is_running = false;

        // Handle UP press while hanging
        if let Some(actor) = world.get_actor(player_id) {
            if actor.is_hanging && is_key_pressed(KeyCode::Up) {
                // Get current hang point
                if let Some(hang_pos) = actor.hang_point {
                    
                    // Find matching platform edge
                    let mut new_pos = None;
                    for platform in &world.platforms {
                        let left_edge = platform.pos.x;
                        let right_edge = platform.pos.x + platform.size.x;
                        
                        // Check which edge matches our hang point
                        if (hang_pos.x - left_edge).abs() < 1.0 {
                            // We're on LEFT edge, move RIGHT
                            new_pos = Some(Vec2::new(
                                left_edge + 18.0,  // Move RIGHT onto platform (was 48.0)
                                hang_pos.y - 66.0   // Move up full height
                            ));
                            break;
                        } else if (hang_pos.x - right_edge).abs() < 1.0 {
                            // We're on RIGHT edge, move LEFT
                            new_pos = Some(Vec2::new(
                                right_edge - 32.0 - 18.0,  // Move LEFT onto platform (was 48.0)
                                hang_pos.y - 66.0  // Move up full height
                            ));
                            break;
                        }
                    }
                    
                    // Update actor position and states
                    if let Some(new_position) = new_pos {
                        if let Some(actor) = world.get_actor_mut(player_id) {
                            actor.pos = new_position;
                            actor.is_hanging = false;
                            actor.grab_immune = true;  // Stay immune until we release UP
                            actor.hang_point = None;
                            actor.velocity = Vec2::ZERO;
                            actor.on_ground = true;
                            actor.in_scripted_movement = true;  // Enter scripted mode
                        }
                        // Force Idle animation immediately after positioning
                        if let Some(anim) = animation.get_animation() {
                            anim.set_state(AnimationState::Idle);
                            anim.force_frame(0);  // Reset to first idle frame
                        }
                        return Vec2::ZERO;  // No movement this frame
                    }
                }
                return Vec2::ZERO;
            } else if actor.is_hanging {
                // Handle DOWN press while hanging
                if is_key_pressed(KeyCode::Down) {
                    if let Some(actor) = world.get_actor_mut(player_id) {
                        actor.is_hanging = false;
                        actor.hang_point = None;
                        actor.velocity = Vec2::new(0.0, 200.0); // Increased downward velocity
                        actor.grab_immune = true;
                        actor.flash_timer = 0.5; // Use flash timer to track grab immunity duration
                        // Push player away from wall slightly to avoid collision
                        if actor.facing_left {
                            actor.pos.x += 5.0; // Push right if facing left
                        } else {
                            actor.pos.x -= 5.0; // Push left if facing right
                        }
                        // Set falling animation
                        if let Some(anim) = animation.get_animation() {
                            anim.set_state(AnimationState::Jumping);
                            anim.force_frame(5); // Use falling frame from jump animation
                        }
                        return Vec2::ZERO; // No movement this frame
                    }
                }
                
                // Set hanging animation when just hanging
                if let Some(anim) = animation.get_animation() {
                    if !anim.is_in_state(&AnimationState::Hanging) {
                        anim.set_state(AnimationState::Hanging);
                    }
                }
            }
        }

        // Handle post-scripted movement (after pull-up) and immunity states
        if let Some(actor) = world.get_actor_mut(player_id) {
            if actor.in_scripted_movement {
                // Check if UP has been released to exit scripted movement
                if !is_key_down(KeyCode::Up) {
                    actor.in_scripted_movement = false;
                    actor.grab_immune = false;  // Reset grab immunity
                }
            } else if actor.grab_immune && !is_key_down(KeyCode::Up) && !is_key_down(KeyCode::Down) {
                // Reset grab immunity when neither UP nor DOWN are held
                actor.grab_immune = false;
            }
        }

        // Check if we're in running state
        if let Some(anim) = animation.get_animation() {
            is_running = anim.is_in_state(&AnimationState::Running);

            // Handle non-hanging movement animations
            if !anim.is_in_state(&AnimationState::Jumping) &&
               !anim.is_in_state(&AnimationState::Attacking1) &&
               !anim.is_in_state(&AnimationState::Attacking2) &&
               !anim.is_in_state(&AnimationState::Attacking3) &&
               !anim.is_in_state(&AnimationState::Attacking4) &&
               !anim.is_in_state(&AnimationState::Shoot) &&
               !anim.is_in_state(&AnimationState::Rolling) {
                let movement_state = self.movement.get_movement_state();
                if !anim.is_in_state(&movement_state) {
                    anim.set_state(movement_state);
                }
            }
        }

        // Calculate movement based on input - block movement if hanging
        let mut movement = if let Some(actor) = world.get_actor(player_id) {
            if actor.is_hanging {
                Vec2::ZERO  // No horizontal movement while hanging
            } else {
                self.movement.update(
                    is_key_down(KeyCode::Right),
                    is_key_down(KeyCode::Left),
                    get_frame_time(),
                    is_running
                )
            }
        } else {
            Vec2::ZERO
        };

        // Update facing direction - only if not hanging
        if let Some(actor) = world.get_actor_mut(player_id) {
            if !actor.is_hanging {  // Only update facing direction when not hanging
                if is_key_down(KeyCode::Right) {
                    actor.facing_left = false;
                } else if is_key_down(KeyCode::Left) {
                    actor.facing_left = true;
                }
            }
            
            // Apply roll movement if in rolling state
            if let Some(anim) = animation.get_animation() {
                if anim.is_in_state(&AnimationState::Rolling) {
                    // Use normal running movement but keep roll animation
                    let base_movement = self.movement.update(
                        actor.facing_left == false,  // right pressed if not facing left
                        actor.facing_left == true,   // left pressed if facing left
                        get_frame_time(),
                        true  // force running state
                    );
                    movement = base_movement * 1.60;  // Increase roll distance by 60%
                }
            }
        }

        movement
    }

    pub fn handle_combat(&mut self, world: &mut World, player_id: usize, animation: &mut PlayerAnimation) {
        // Get player's on_ground state first
        let on_ground = world.get_actor(player_id)
            .map(|a| a.on_ground)
            .unwrap_or(false);

        // Handle shield input
        if is_key_pressed(KeyCode::Down) && on_ground {
            // Only allow shooting if we have enough power
            if let Some(actor) = world.get_actor_mut(player_id) {
                if let Some(power) = &mut actor.power {
                    if let Some(anim) = animation.get_animation() {
                        if !self.combat.is_attacking(anim) && 
                           power.current_power >= 2.5 && 
                           (anim.is_in_state(&AnimationState::Idle) || 
                            anim.is_in_state(&AnimationState::Walking)) {
                            
                            power.use_power(2.5);  // Deduct 2.5 power points
                            anim.set_state(AnimationState::Shoot);
                        }
                    }
                }
            }
        }

        // Handle roll input
        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && on_ground {
            if let Some(actor) = world.get_actor_mut(player_id) {
                if let Some(power) = &mut actor.power {
                    if let Some(anim) = animation.get_animation() {
                        if !self.combat.is_rolling(anim) && 
                           !self.combat.is_attacking(anim) &&
                           !anim.is_in_state(&AnimationState::Shoot) &&
                           (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right)) {
                            if let Some(roll_state) = self.combat.try_roll(power, get_time() as f32) {
                                actor.is_rolling = true;  // Enable roll collision pass-through
                                anim.force_frame(0);  // Ensure we start at first frame
                                anim.set_state(roll_state);
                            }

                        // Disable rolling when roll animation finishes
                        } else if actor.is_rolling && anim.is_in_state(&AnimationState::Rolling) {
                            if let Some(frame) = anim.get_current_frame() {
                                if frame >= 5 {  // Start transition one frame earlier
                                    // Get current movement speed for transition
                                    let mut current_speed = self.movement.update(
                                        actor.facing_left == false,
                                        actor.facing_left == true,
                                        get_frame_time(),
                                        true
                                    );
                                    
                                    // Gradually decrease roll speed in last 2 frames
                                    let transition_factor = if frame == 5 { 0.5 } else { 0.25 };
                                    current_speed.x *= transition_factor;
                                    
                                    if frame >= 6 {  // Only stop rolling at very end
                                        actor.is_rolling = false;
                                        
                                        // Choose animation based on remaining speed
                                        anim.set_state(if current_speed.x.abs() > 50.0 { 
                                            AnimationState::Running 
                                        } else { 
                                            AnimationState::Idle 
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Handle attack input
        if is_key_pressed(KeyCode::Space) {
            let power_cost = world.get_power_cost(false) as f32;
            // Only allow attack if we have power
            if let Some(actor) = world.get_actor_mut(player_id) {
                if let Some(power) = &mut actor.power {
                    if let Some(anim) = animation.get_animation() {
                        // Only allow new attacks if we're not currently in an attack animation
                        if !self.combat.is_attacking(anim) {
                            match self.combat.try_attack(power, power_cost, get_time() as f32) {
                                Some(attack_state) => {
                                    anim.set_state(attack_state);
                                }
                                None => {
                                    // No power - flash player sprite only
                                    actor.flash_timer = 0.1; // 100ms flash
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn handle_jumping(&mut self, world: &mut World, player_id: usize, animation: &mut PlayerAnimation, movement: &mut Vec2, on_ground: bool) {
        if let Some(anim) = animation.get_animation() {
            // Handle jump initiation
            if is_key_pressed(KeyCode::Up)
                && !world.get_actor(player_id).map_or(false, |a| {
                    let skip = a.is_hanging || a.in_scripted_movement;
                    skip
                })  // Don't jump if hanging or in scripted move
                && (world.can_regular_jump(player_id) || world.can_double_jump(player_id))
            {
                // Update double jump state if this is a double jump
                if world.can_double_jump(player_id) {
                    if let Some(actor) = world.get_actor_mut(player_id) {
                        actor.can_double_jump = false;
                        actor.flash_timer = 0.2; // Start flash effect
                    }
                    anim.force_frame(0); // Reset jump animation for second boost
                }
                anim.set_state(AnimationState::Jumping);
            }

            // Handle jump physics
            if anim.is_in_state(&AnimationState::Jumping) {
                match anim.get_current_frame() {
                    Some(0) | Some(1) => {
                        // Prepare to jump - stay on ground
                        movement.y = 0.0;
                    }
                    Some(2) | Some(3) => {
                        movement.y = -375.0;  // Original full jump height
                        movement.x *= 0.7;    // Dampen horizontal momentum
                    }
                    Some(4) => {
                        movement.y = -190.0;  // Original peak force
                    }
                    Some(5) => {
                        // Immediate drop after peak
                        movement.y = 50.0;
                    }
                    Some(6) | Some(7) => {
                        // Continue falling
                        movement.y = 50.0;
                        if on_ground {
                            anim.set_state(AnimationState::Idle);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn check_state_change(&self) -> Option<GameStateChange> {
        if is_key_pressed(KeyCode::Escape) {
            Some(GameStateChange::Pause)
        } else if is_key_pressed(KeyCode::Q) {
            Some(GameStateChange::QuitToMenu)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub enum GameStateChange {
    Pause,
    QuitToMenu,
}