use macroquad::prelude::*;

mod assets;
mod core;
mod effects;
mod enemies;
mod game;
mod input;
mod physics;
use core::health::Health;
use core::power::Power;
mod player;
use crate::assets::GameBackground;
use crate::effects::{AnimationState, Projectile, ProjectileOwner, ProjectileState};
use crate::enemies::{Enemy, EnemySpawn};
use crate::game::{GameState, StateHandler};
use crate::input::InputHandler;
use crate::physics::{PlatformManager, World};

// Import the AudioSystem and SoundTriggers for sound effects
use warchild::audio::{AudioSystem, SoundTriggers};
use warchild::effects::AnimationState as LibAnimationState;
use warchild::objects::Collectible;

// Helper function to convert between similar AnimationState enums
fn convert_animation_state(state: crate::effects::AnimationState) -> Option<LibAnimationState> {
    match state {
        crate::effects::AnimationState::Idle => Some(LibAnimationState::Idle),
        crate::effects::AnimationState::Walking => Some(LibAnimationState::Walking),
        crate::effects::AnimationState::Running => Some(LibAnimationState::Running),
        crate::effects::AnimationState::Jumping => Some(LibAnimationState::Jumping),
        crate::effects::AnimationState::Attacking1 => Some(LibAnimationState::Attacking1),
        crate::effects::AnimationState::Attacking2 => Some(LibAnimationState::Attacking2),
        crate::effects::AnimationState::Attacking3 => Some(LibAnimationState::Attacking3),
        crate::effects::AnimationState::Attacking4 => Some(LibAnimationState::Attacking4),
        crate::effects::AnimationState::Hanging => Some(LibAnimationState::Hanging),
        crate::effects::AnimationState::PullUp => Some(LibAnimationState::PullUp),
        crate::effects::AnimationState::Shoot => Some(LibAnimationState::Shoot),
        crate::effects::AnimationState::Rolling => Some(LibAnimationState::Rolling),
        crate::effects::AnimationState::Falling => Some(LibAnimationState::Falling),
        crate::effects::AnimationState::Dead => Some(LibAnimationState::Dead),
        _ => None,
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "War Child".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state_handler = StateHandler::new();
    let mut game_background: Option<GameBackground> = None;
    let mut platform_manager: Option<PlatformManager> = None;
    let mut player_animation = player::PlayerAnimation::new().await;
    let mut player_movement = player::PlayerMovement::new();
    let mut player_combat = player::PlayerCombat::new();
    let mut input_handler = InputHandler::new();
    let mut spawn_animation: Option<EnemySpawn> = None;
    let mut active_enemies: Vec<Enemy> = Vec::new();
    let mut world = World::new();
    let mut player_id: Option<usize> = None;
    
    // Initialize AudioSystem and SoundTriggers
    let mut audio_system = AudioSystem::new();
    let mut sound_triggers = SoundTriggers::new();

    loop {
        clear_background(BLACK);

        // Handle state-specific updates and rendering
        match state_handler.get_state() {
            GameState::Loading => {
                if state_handler.handle_loading_state().await {
                    // Preload common sounds while in loading screen
                    audio_system.queue_all_audio();
                    
                    // Process some audio loads each frame during loading
                    let _processed = audio_system.process_queue(10).await;
                    
                    continue;
                }
            }
            GameState::Menu => {
                state_handler.handle_menu_state();
            }
            GameState::Playing => {
                // Update state handler first
                state_handler.update(get_frame_time());

                // Initialize background if needed
                if game_background.is_none() {
                    game_background = GameBackground::new().await;
                }

                // Initialize animation system if needed
                if !player_animation.is_initialized() {
                    player_animation.initialize().await;
                }

                // Initialize platforms if needed
                if platform_manager.is_none() {
                    let mut pm = PlatformManager::new();
                    pm.initialize(&mut world).await;
                    platform_manager = Some(pm);

                    // Start player above the leftmost platform
                    if let Some(platform_pos) = world.get_leftmost_platform_top() {
                        let player_pos = Vec2::new(
                            platform_pos.x + 50.0,  // Slightly right of platform left edge
                            platform_pos.y - 200.0, // Above the platform
                        );
                        player_id = Some(world.add_actor(
                            player_pos,
                            Vec2::new(32.0, 66.0), // Match sprite height
                            Some(0),               // Force player to be ID 0
                        ));

                        // Initialize player health and power
                        if let Some(id) = player_id {
                            if let Some(actor) = world.get_actor_mut(id) {
                                actor.is_player = true; // Mark this actor as the player
                                let mut player_health = Health::new();
                                player_health.max_health = 100.0; // Set player max health to 100
                                player_health.current_health = 100.0; // Start at full health
                                player_health.load_textures().await;
                                actor.health = Some(player_health);

                                let mut player_power = Power::new();
                                player_power.load_textures().await;
                                actor.power = Some(player_power);
                            }
                        }

                        // Now that platforms are initialized, spawn initial gem
                        futures::executor::block_on(
                            state_handler.state_manager.spawn_initial_gem(&world),
                        );
                    }
                }

                // Draw background if loaded
                if let Some(bg) = &mut game_background {
                    bg.draw();
                }

                // Draw platforms
                if let Some(pm) = &mut platform_manager {
                    pm.draw();
                }

                // Draw gems
                state_handler.draw_gems();

                // Draw artifacts
                for artifact in state_handler.state_manager.get_active_artifacts() {
                    artifact.draw();
                }

                // Check for gem collection
                if let Some(id) = player_id {
                    if let Some(actor) = world.get_actor_mut(id) {
                        let player_rect =
                            Rect::new(actor.pos.x, actor.pos.y, actor.size.x, actor.size.y);

                        // Track which gem to collect (if any)
                        let mut gem_to_collect = None;

                        // Check collision with each gem first
                        for (i, gem) in state_handler
                            .state_manager
                            .get_active_gems()
                            .iter()
                            .enumerate()
                        {
                            if !gem.collected && gem.active {
                                if player_rect.overlaps(&gem.bounds()) {
                                    gem_to_collect = Some(i);
                                    break;
                                }
                            }
                        }

                        // Now handle collection if needed
                        if let Some(i) = gem_to_collect {
                            // Start collection animation on gem
                            state_handler.state_manager.mark_gem_collected(i);
                            
                            // Play gem pickup sound
                            sound_triggers.handle_item_pickup(&audio_system, "gem");

                            // Trigger player collection effects - flash white and grow
                            if let Some(anim) = player_animation.get_animation() {
                                anim.flash_white(); // Trigger white flash
                                anim.grow_by_percent(10.0); // Grow by 10%
                            }

                            // Get rewards before collection
                            if let Some(rewards) =
                                state_handler.state_manager.get_active_gems().get(i)
                            {
                                let (health_reward, power_reward) = rewards.get_rewards();

                                // Apply rewards
                                if let (Some(health), Some(power)) =
                                    (&mut actor.health, &mut actor.power)
                                {
                                    health.current_health = (health.current_health
                                        + health_reward as f32)
                                        .min(health.max_health);
                                    power.current_power = (power.current_power
                                        + power_reward as f32)
                                        .min(power.max_power);
                                }
                            }
                        }

                        // Track which artifact to collect (if any)
                        let mut artifact_to_collect = None;

                        // Check collision with each artifact
                        for (i, artifact) in state_handler
                            .state_manager
                            .get_active_artifacts()
                            .iter()
                            .enumerate()
                        {
                            if !artifact.is_collected() && artifact.is_active() {
                                if player_rect.overlaps(&artifact.bounds()) {
                                    artifact_to_collect = Some(i);
                                    break;
                                }
                            }
                        }

                        // Now handle artifact collection if needed
                        if let Some(i) = artifact_to_collect {
                            // Start collection animation on artifact
                            state_handler.state_manager.mark_artifact_collected(i);
                            
                            // Play artifact pickup sound
                            sound_triggers.handle_item_pickup(&audio_system, "artifact");

                            // Trigger player collection effects - flash white and grow
                            if let Some(anim) = player_animation.get_animation() {
                                anim.flash_white(); // Trigger white flash
                                anim.grow_by_percent(10.0); // Grow by 10%
                            }

                            // Get rewards before collection
                            if let Some(rewards) =
                                state_handler.state_manager.get_active_artifacts().get(i)
                            {
                                let (health_reward, power_reward) = rewards.get_rewards();

                                // Apply rewards
                                if let (Some(health), Some(power)) =
                                    (&mut actor.health, &mut actor.power)
                                {
                                    health.current_health = (health.current_health
                                        + health_reward as f32)
                                        .min(health.max_health);
                                    power.current_power = (power.current_power
                                        + power_reward as f32)
                                        .min(power.max_power);
                                }
                            }
                        }
                    }
                }

                // Update and draw projectiles BEFORE poison effects but AFTER background
                state_handler.update_projectiles(get_frame_time());

                // Check projectile collisions with enemies
                let mut collided_projectiles: Vec<(Vec2, usize)> = Vec::new(); // (collision_point, enemy_id)

                // Get reference to projectiles through state_manager
                for projectile in &state_handler.state_manager.projectiles {
                    // Skip inactive projectiles
                    if projectile.get_state() != ProjectileState::Active {
                        continue;
                    }

                    // Check each enemy for collision
                    for enemy in &active_enemies {
                        if let Some(enemy_id) = enemy.world_id {
                            if let Some(actor) = world.get_actor(enemy_id) {
                                if let Some(collision_point) =
                                    projectile.check_collision(actor.pos, actor.size)
                                {
                                    collided_projectiles.push((collision_point, enemy_id));
                                    break; // Only collide with first enemy hit
                                }
                            }
                        }
                    }
                }

                // Handle collisions and damage
                for (collision_point, enemy_id) in collided_projectiles {
                    // Find and update projectile through state_manager
                    for projectile in &mut state_handler.state_manager.projectiles {
                        if projectile.get_state() == ProjectileState::Active {
                            projectile.handle_collision(collision_point);

                            // Deal damage to enemy
                            world.apply_damage(
                                projectile.owner_id,
                                enemy_id,
                                false,
                                &mut active_enemies,
                                &state_handler.state_manager.projectiles,
                            );
                            
                            // Play hit sound
                            sound_triggers.handle_hit(&audio_system, false);
                            break;
                        }
                    }
                }

                state_handler.draw_projectiles();

                // Check if we should create spawn animation
                if state_handler.should_spawn() {
                    if let Some(pm) = &platform_manager {
                        if let Some(pos) = pm.get_upper_right_platform_position() {
                            spawn_animation = EnemySpawn::new(pos).await;
                            match &spawn_animation {
                                Some(_) => state_handler.mark_spawned(),
                                None => {}
                            }
                        }
                    }
                }

                // Update and draw spawn animation if it exists
                if let Some(spawn) = &mut spawn_animation {
                    spawn.update(get_frame_time());
                    spawn.draw();

                    // Create enemy when animation completes
                    if spawn.is_complete() {
                        let spawn_pos = spawn.get_position();
                        spawn_animation = None;

                        // Get next enemy type in HP order
                        if let Some(enemy_type) = state_handler.get_next_enemy_type() {
                            // Create enemy with ordered type
                            match Enemy::new(enemy_type, spawn_pos).await {
                                Some(mut enemy) => {
                                    // Add enemy to the physics world
                                    let enemy_id = world.add_actor(
                                        spawn_pos,
                                        Vec2::new(32.0, 66.0), // Same size as player
                                        None,                  // Use next sequential ID for enemies
                                    );

                                    // Store the ID
                                    enemy.world_id = Some(enemy_id);
                                    active_enemies.push(enemy);
                                    state_handler.mark_enemy_spawned();
                                }
                                None => {}
                            }
                        }
                    }
                }

                // Update physics world
                world.update(get_frame_time());

                // Update and draw enemies
                if let Some(id) = player_id {
                    if let Some(player_pos) = world.get_actor_position(id) {
                        // First update/draw all enemies
                        let mut pending_damage = None;
                        {
                            active_enemies.retain_mut(|enemy| {
                                // Update health state first for enemies too
                                if let Some(enemy_id) = enemy.world_id {
                                    if let Some(actor) = world.get_actor_mut(enemy_id) {
                                        if let Some(health) = &mut actor.health {
                                            health.update(get_frame_time());
                                        }
                                    }
                                }

                                // Regular update for living enemy
                                if enemy.current_state != AnimationState::Dead {
                                    if let Some((target_id, is_enemy_attack)) = enemy.update(
                                        get_frame_time(),
                                        player_pos,
                                        world.platforms.as_slice(),
                                    ) {
                                        if let Some(enemy_id) = enemy.world_id {
                                            // Save the damage request for after the loop
                                            pending_damage =
                                                Some((enemy_id, target_id, is_enemy_attack));
                                        }
                                    }
                                }

                                // Get the actor's current health
                                if let Some(enemy_id) = enemy.world_id {
                                    if let Some(actor) = world.get_actor(enemy_id) {
                                        if let Some(health) = &actor.health {
                                            enemy.health.current_health = health.current_health;
                                        }
                                    }
                                }

                                // Always update death animation if in death state
                                if enemy.current_state == AnimationState::Dead {
                                    if let Some(anim) =
                                        enemy.animations.get_mut(&AnimationState::Dead)
                                    {
                                        anim.update(get_frame_time());
                                    }
                                }

                                // Sync position and health state back to actor
                                if let Some(enemy_id) = enemy.world_id {
                                    if let Some(actor) = world.get_actor_mut(enemy_id) {
                                        actor.pos = enemy.get_position();
                                        if let Some(health) = &mut actor.health {
                                            health.current_health = enemy.health.current_health;
                                            health.is_taking_damage = enemy.health.is_taking_damage;
                                            health.damage_flash_timer =
                                                enemy.health.damage_flash_timer;
                                        }
                                    }
                                }
                                enemy.draw(player_pos);

                                // Check if enemy should be removed
                                if enemy.current_state == AnimationState::Dead {
                                    if let Some(anim) = enemy.animations.get(&AnimationState::Dead)
                                    {
                                        if anim.current_frame() == 4 {
                                            // Last frame - remove this enemy
                                            if let Some(enemy_id) = enemy.world_id {
                                                // Remove from physics world
                                                world.remove_actor(enemy_id);
                                                // Handle drops
                                                state_handler.state_manager.handle_enemy_death(enemy.position, enemy.size);
                                                
                                                // Play death sound
                                                sound_triggers.handle_enemy_death(&audio_system);
                                            }
                                            state_handler.mark_enemy_dead();
                                            return false; // Remove from vec
                                        }
                                    }
                                }
                                true // Keep enemy in vec
                            });
                        }

                        // Handle any pending damage after the loop
                        if let Some((enemy_id, target_id, is_enemy_attack)) = pending_damage {
                            world.apply_damage(
                                enemy_id,
                                target_id,
                                is_enemy_attack,
                                &mut active_enemies,
                                &state_handler.state_manager.projectiles,
                            );
                        }

                        // After any enemy removals, update remaining enemies
                        // DO NOT modify their IDs - just clear dead ones
                        active_enemies.retain(|enemy| {
                            if let Some(enemy_id) = enemy.world_id {
                                if enemy.current_state == AnimationState::Dead {
                                    if let Some(anim) = enemy.animations.get(&AnimationState::Dead)
                                    {
                                        if anim.is_finished() {
                                            // Remove dead enemy silently
                                            world.remove_actor(enemy_id);
                                            // Handle drops
                                            state_handler.state_manager.handle_enemy_death(enemy.position, enemy.size);
                                            
                                            // Play death sound
                                            sound_triggers.handle_enemy_death(&audio_system);

                                            state_handler.mark_enemy_dead();
                                            return false; // Remove from vec
                                        }
                                    }
                                }
                            }
                            true // Keep enemy in vec
                        });
                    }
                }

                if let Some(id) = player_id {
                    if let Some(actor) = world.get_actor_mut(id) {
                        // Update health state first
                        if let Some(health) = &mut actor.health {
                            health.update(get_frame_time());
                        }

                        // Death from falling off screen
                        if actor.pos.y > screen_height() {
                            state_handler.set_state(GameState::GameOver);
                            continue; // Skip rest of frame
                        }
                        // Check if dead (from any cause)
                        if let Some(health) = &actor.health {
                            if health.current_health <= 0.0 {
                                // Skip death animation, go straight to game over

                                active_enemies.clear();
                                spawn_animation = None;
                                state_handler.set_state(GameState::GameOver);
                                continue;
                            }
                        }
                    }
                }

                // Update player movement and actions
                if let Some(id) = player_id {
                    let mut movement =
                        input_handler.handle_movement(&mut world, id, &mut player_animation);

                    // Handle player actions
                    let actor_on_ground = world.get_actor(id).map(|a| a.on_ground).unwrap_or(false);
                    input_handler.handle_jumping(
                        &mut world,
                        id,
                        &mut player_animation,
                        &mut movement,
                        actor_on_ground,
                    );
                    input_handler.handle_combat(&mut world, id, &mut player_animation);

                    // Collect all needed data first
                    let actor_pos = world.get_actor_position(id);
                    let mut actor_facing =
                        world.get_actor(id).map(|a| a.facing_left).unwrap_or(false);
                    let _actor_on_ground = world.get_actor(id).map(|a| a.on_ground).unwrap_or(false);
                    let _actor_size = world.get_actor_size(id).unwrap_or(Vec2::new(32.0, 66.0));

                    if let Some(anim) = player_animation.get_animation() {
                        anim.update(get_frame_time());
                        if anim.is_in_state(&AnimationState::Attacking1)
                            || anim.is_in_state(&AnimationState::Attacking2)
                            || anim.is_in_state(&AnimationState::Attacking3)
                            || anim.is_in_state(&AnimationState::Attacking4)
                        {
                            match anim.get_current_frame() {
                                Some(3) => {
                                    // Only check for hits if we have enough power
                                    if let Some(actor) = world.get_actor(id) {
                                        if let Some(power) = &actor.power {
                                            if power.current_power
                                                >= world.get_power_cost(false) as f32
                                            {
                                                // Only check on frame 3, like enemy
                                                // Check for hits during this frame
                                                if let Some(hit_id) = world
                                                    .check_attack_hit(id, Vec2::new(40.0, 66.0))
                                                {
                                                    // Apply damage on hit
                                                    world.apply_damage(
                                                        id,
                                                        hit_id,
                                                        false,
                                                        &mut active_enemies,
                                                        &state_handler.state_manager.projectiles,
                                                    ); // false = player attack

                                                    // Note: Damage is handled by world.apply_damage() - we don't need to apply it again here
                                                    // Just sync health state for visual updates
                                                    for enemy in &mut active_enemies {
                                                        if let Some(enemy_id) = enemy.world_id {
                                                            if enemy_id == hit_id {
                                                                if let Some(actor) =
                                                                    world.get_actor(hit_id)
                                                                {
                                                                    if let Some(health) =
                                                                        &actor.health
                                                                    {
                                                                        enemy
                                                                            .health
                                                                            .current_health =
                                                                            health.current_health;
                                                                    }
                                                                }
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(5) => {
                                    anim.set_state(AnimationState::Idle);
                                }
                                Some(_) => {}
                                None => {}
                            }
                        } else if anim.is_in_state(&AnimationState::Shoot) {
                            anim.update(get_frame_time());
                            let max_frame = anim.get_frame_count() - 1;

                            // Spawn projectile at end of animation
                            if anim.current_frame() == max_frame {
                                if let Some(actor) = world.get_actor(id) {
                                    let spawn_pos = Vec2::new(
                                        actor.pos.x + if actor.facing_left { -20.0 } else { 20.0 },
                                        actor.pos.y - 48.0, // Moved 16 pixels higher (from -32.0 to -48.0)
                                    );

                                    // Scale arrow damage with enemy max health
                                    let scaled_damage = if let Some(hit_id) = world.check_attack_hit(id, Vec2::new(40.0, 66.0)) {
                                        // Get target's health to scale damage
                                        if let Some(target) = world.get_actor(hit_id) {
                                            if let Some(health) = &target.health {
                                                // Scale damage between 25-50% of enemy max health
                                                let min_damage = health.max_health * 0.25;
                                                let max_damage = health.max_health * 0.50;
                                                rand::gen_range(min_damage, max_damage)
                                            } else {
                                                50.0 // Fallback if no health component
                                            }
                                        } else {
                                            50.0 // Fallback if no target
                                        }
                                    } else {
                                        50.0 // Fallback if no hit detected
                                    };

                                    let mut projectile = Projectile::new(
                                        spawn_pos,
                                        actor.facing_left,
                                        ProjectileOwner::Player,
                                        0, // Player is always ID 0
                                        scaled_damage, // Use scaled damage
                                    );
                                    projectile.initialize_animation().await;
                                    state_handler.add_projectile(projectile);

                                    // Now transition to Idle
                                    anim.set_state(AnimationState::Idle);
                                }
                            }
                        } else if anim.is_in_state(&AnimationState::PullUp) {
                            if let Some(actor) = world.get_actor_mut(id) {
                                if let Some(target_pos) = actor.target_pos {
                                    match anim.get_current_frame() {
                                        Some(0) => {
                                            // Stay at start
                                            actor.velocity = Vec2::ZERO; // Freeze in place
                                        }
                                        Some(1) => {
                                            // Move exactly 33% to target
                                            actor.velocity = Vec2::ZERO;
                                            // Calculate exact 33% position
                                            let new_pos = Vec2::new(
                                                target_pos.x * 0.33 + actor.pos.x * 0.67, // 33% of the way
                                                target_pos.y * 0.33 + actor.pos.y * 0.67,
                                            );
                                            actor.pos = new_pos;
                                        }
                                        Some(2) => {
                                            // Move exactly 66% to target
                                            actor.velocity = Vec2::ZERO;
                                            // Calculate exact 66% position
                                            let new_pos = Vec2::new(
                                                target_pos.x * 0.66 + actor.pos.x * 0.34, // 66% of the way
                                                target_pos.y * 0.66 + actor.pos.y * 0.34,
                                            );
                                            actor.pos = new_pos;
                                        }
                                        Some(3) => {
                                            // Set final position + cleanup
                                            actor.pos = target_pos; // Set exact final position
                                            actor.velocity = Vec2::ZERO;
                                            actor.in_scripted_movement = false;
                                            actor.is_hanging = false;
                                            actor.grab_immune = false;
                                            actor.hang_point = None;
                                            actor.target_pos = None;
                                            actor.on_ground = true;
                                            anim.set_state(AnimationState::Idle);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            movement = Vec2::ZERO;
                        } else {
                            // Get initial state
                            let is_hanging;
                            let facing_left;

                            if let Some(actor) = world.get_actor(id) {
                                is_hanging = actor.is_hanging;
                                facing_left = actor.facing_left;
                            } else {
                                is_hanging = false;
                                facing_left = false;
                            }

                            if is_hanging {
                                // Switch to hanging animation if we're not in it
                                if !anim.is_in_state(&AnimationState::Hanging) {
                                    anim.set_state(AnimationState::Hanging);
                                }

                                // Only handle up/down input when hanging
                                if is_key_pressed(KeyCode::Up) {
                                    // STEP 1: Get hang point and check edge type first
                                    let mut target_position = None;
                                    let current_hang_point =
                                        if let Some(actor) = world.get_actor(id) {
                                            actor.hang_point
                                        } else {
                                            None
                                        };

                                    if let Some(hang_pos) = current_hang_point {
                                        // Check platform edges to determine if we're on right edge
                                        let mut on_right_edge = false;
                                        for platform in &world.platforms {
                                            let right_edge = platform.pos.x + platform.size.x;
                                            if (hang_pos.x - right_edge).abs() < 1.0 {
                                                on_right_edge = true;
                                                break;
                                            }
                                        }

                                        // Calculate target position based on edge type
                                        if let Some(actor) = world.get_actor(id) {
                                            target_position = Some(Vec2::new(
                                                if on_right_edge {
                                                    hang_pos.x - actor.size.x - 48.0
                                                // Move LEFT from right edge
                                                } else {
                                                    hang_pos.x + 48.0 // Move RIGHT from left edge
                                                },
                                                hang_pos.y - 66.0, // Move up full height
                                            ));
                                        }
                                    }

                                    // STEP 2: Update actor with target position
                                    if let Some(target_pos) = target_position {
                                        if let Some(actor) = world.get_actor_mut(id) {
                                            actor.in_scripted_movement = true;
                                            actor.velocity = Vec2::ZERO;
                                            actor.target_pos = Some(target_pos);
                                        }

                                        // Start animation at frame 0
                                        anim.set_state(AnimationState::PullUp);
                                        anim.force_frame(0);
                                    }
                                } else if is_key_pressed(KeyCode::Down) {
                                    if let Some(actor) = world.get_actor_mut(id) {
                                        actor.is_hanging = false;
                                        actor.hang_point = None;
                                        actor.velocity = Vec2::new(0.0, 100.0);
                                    }
                                    anim.set_state(AnimationState::Falling);
                                }

                                // Store facing for rendering
                                actor_facing = facing_left;
                            }
                        }

                        // Continue with movement update - handle any poison effect
                        // Apply movement if we're not dead
                        if !anim.is_in_state(&AnimationState::Dead) {
                            // Movement continues normally whether poisoned or not
                            world.move_actor(id, movement);
                        }

                        // Finally do rendering with the collected data
                        if let Some(pos) = actor_pos {
                            let sprite_pos = Vec2::new(
                                pos.x - 48.0,           // Keep horizontal centering
                                pos.y - (128.0 - 66.0), // Align sprite feet with collision box bottom
                            );

                            // Update effects first
                            anim.update_effects(get_frame_time());

                            // Determine render color and scale
                            let color = if let Some(actor) = world.get_actor(id) {
                                if actor.flash_timer > 0.0 {
                                    // Check if this was triggered by a double jump
                                    if anim.is_in_state(&AnimationState::Jumping) {
                                        WHITE // Double jump flash
                                    } else {
                                        RED // Taking damage or no power
                                    }
                                } else if anim.is_in_state(&AnimationState::Dead) {
                                    // Apply fade effect on last frame of death animation
                                    if let Some(frame) = anim.get_current_frame() {
                                        if frame >= 4 {
                                            // Fade out during final frames
                                            Color::new(1.0, 1.0, 1.0, 0.5) // 50% opacity
                                        } else {
                                            WHITE
                                        }
                                    } else {
                                        WHITE
                                    }
                                } else {
                                    // Use flash effect color if active, otherwise WHITE
                                    anim.get_flash_color().unwrap_or(WHITE)
                                }
                            } else {
                                WHITE
                            };

                            // Get current scale from animation system
                            let scale = anim.get_current_scale();

                            // Draw at collision box position with current scale and color
                            anim.draw(sprite_pos, actor_facing, scale, color);
                        }
                    }
                }

                // Draw health and power bars if player exists
                if let Some(id) = player_id {
                    if let Some(actor) = world.get_actor(id) {
                        // Draw health and power bars
                        if let Some(health) = &actor.health {
                            health.draw_health_bar(Vec2::new(20.0, 20.0));
                        }
                        if let Some(power) = &actor.power {
                            power.draw_power_bar(Vec2::new(20.0, 45.0));
                        }
                        
                        // Update sound triggers based on player state
                        sound_triggers.update(
                            &audio_system,
                            // Convert the binary AnimationState to the lib AnimationState
                            convert_animation_state(player_animation.get_current_state()),
                            actor.on_ground,
                            get_frame_time()
                        );
                    }
                }

                // Update and draw projectiles
                state_handler.update_projectiles(get_frame_time());

                state_handler.draw_projectiles();

                // Draw poison effect on top of everything
                if let Some(id) = player_id {
                    if let Some(actor) = world.get_actor_mut(id) {
                        // Handle poison damage and visuals
                        let mut poison_expired = false;

                        if let Some(poison) = &mut actor.poison_effect {
                            poison.timer -= get_frame_time();
                            poison.next_tick -= get_frame_time();

                            // Check for poison damage tick
                            if poison.next_tick <= 0.0 {
                                if let Some(health) = &mut actor.health {
                                    let scaled_damage = rand::gen_range(1.0, 2.0); // Original 1-2 damage per second
                                    health.take_damage(scaled_damage);
                                }
                                poison.next_tick = 1.0; // Reset tick timer
                            }

                            // Check if poison should expire
                            if poison.timer <= 0.0 {
                                poison_expired = true;
                            } else {
                                // Only draw if not expired
                                if let Some(anim) = &mut poison.animation {
                                    anim.update(get_frame_time());
                                    let effect_pos =
                                        Vec2::new(actor.pos.x - 48.0, actor.pos.y - (128.0 - 66.0));
                                    anim.draw(
                                        effect_pos,
                                        false,
                                        Vec2::ONE,
                                        Color::new(1.0, 1.0, 1.0, 0.7),
                                    );
                                }
                            }
                        }

                        // Clean up expired poison effect
                        if poison_expired {
                            actor.poison_effect = None;
                        }
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    state_handler.set_state(GameState::Paused);
                    
                    // Play pause sound
                    sound_triggers.handle_ui_event(&audio_system, "pause");
                } else if is_key_pressed(KeyCode::Q) {
                    game_background = None; // Clear background when returning to menu
                    platform_manager = None; // Clear platforms when returning to menu
                    player_animation.reset(); // Clear animation when returning to menu
                    player_movement.reset(); // Reset movement state
                    player_combat.reset(); // Reset combat state
                    player_id = None; // Clear player reference
                    active_enemies.clear(); // Clear enemies
                    spawn_animation = None; // Clear spawn animation
                    state_handler.reset_all(); // Reset spawn timers and flags
                    world = World::new(); // Reset physics world
                    state_handler.set_state(GameState::Menu);
                }
            }
            GameState::Paused => {
                state_handler.handle_paused_state();
                if state_handler.is_state(GameState::Menu) {
                    // Reset everything when quitting to menu
                    game_background = None;
                    platform_manager = None;
                    player_animation.reset();
                    player_movement.reset();
                    player_combat.reset();
                    player_id = None;
                    active_enemies.clear();
                    spawn_animation = None;
                    state_handler.reset_all();
                    world = World::new();
                }
            }
            GameState::GameOver => {
                // Immediately clean up any running systems to prevent resource consumption
                if !active_enemies.is_empty() {
                    active_enemies.clear();
                }
                if spawn_animation.is_some() {
                    spawn_animation = None;
                }

                state_handler.handle_game_over_state();
                if state_handler.is_state(GameState::Menu) {
                    // Full reset of all game systems
                    game_background = None;
                    platform_manager = None;
                    player_animation.reset();
                    player_movement.reset();
                    player_combat.reset();
                    player_id = None;
                    active_enemies.clear();
                    spawn_animation = None;
                    state_handler.reset_all();
                    // Create fresh physics world
                    world = World::new();
                }
            }
        }

        // Debug info - only show in non-menu states
        if !state_handler.is_state(GameState::Menu) {
            let debug_text = format!(
                "State: {:?} | Screen: {}x{}",
                state_handler.get_state(),
                screen_width() as i32,
                screen_height() as i32
            );
            draw_text(&debug_text, 20.0, screen_height() - 20.0, 20.0, GRAY);
        }

        next_frame().await;
    }
}
