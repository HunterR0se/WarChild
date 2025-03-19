//! Sound trigger system for connecting game events to audio playback

use macroquad::prelude::*;

use super::AudioSystem;
use super::types::SoundCategory;
use crate::effects::AnimationState;

/// Manages sound triggers for various game events
pub struct SoundTriggers {
    // Keep track of the last animation state to detect transitions
    last_animation_state: Option<AnimationState>,
    // Keep track of the last on_ground state to detect landing
    last_on_ground: bool,
    // Time since footstep sound was played
    footstep_timer: f32,
    // Time since attack sound was played
    attack_timer: f32,
    // Minimum time between footsteps for walking
    walk_footstep_interval: f32,
    // Minimum time between footsteps for running
    run_footstep_interval: f32,
    // Minimum time between attack sounds
    attack_interval: f32,
}

impl SoundTriggers {
    /// Create a new sound triggers manager
    pub fn new() -> Self {
        Self {
            last_animation_state: None,
            last_on_ground: true,
            footstep_timer: 0.0,
            attack_timer: 0.0,
            walk_footstep_interval: 0.4, // seconds between walking footsteps
            run_footstep_interval: 0.25, // seconds between running footsteps
            attack_interval: 0.2,        // seconds between attack sounds
        }
    }
    
    /// Update sound triggers based on current game state
    pub fn update(
        &mut self,
        audio: &AudioSystem,
        animation_state: Option<AnimationState>,
        on_ground: bool,
        dt: f32,
    ) {
        // Update timers
        self.footstep_timer += dt;
        self.attack_timer += dt;
        
        // Handle state transitions
        if let Some(current_state) = animation_state {
            // Detect animation state transitions
            if self.last_animation_state != Some(current_state) {
                self.handle_state_transition(audio, self.last_animation_state, current_state);
            }
            
            // Handle ongoing states
            self.handle_continuous_state(audio, current_state, on_ground);
            
            // Handle landing
            if on_ground && !self.last_on_ground {
                self.handle_landing(audio);
            }
            
            // Update last state
            self.last_animation_state = Some(current_state);
        }
        
        // Update last on_ground state
        self.last_on_ground = on_ground;
    }
    
    /// Handle transitions between animation states
    fn handle_state_transition(
        &mut self,
        audio: &AudioSystem,
        from_state: Option<AnimationState>,
        to_state: AnimationState,
    ) {
        match to_state {
            AnimationState::Jumping => {
                if matches!(from_state, Some(AnimationState::Idle | AnimationState::Walking | AnimationState::Running)) {
                    // Play jump sound at start of jump
                    audio.play(SoundCategory::Movement, "jump");
                }
            }
            AnimationState::Attacking1 => {
                self.play_attack_sound(audio, 1);
            }
            AnimationState::Attacking2 => {
                self.play_attack_sound(audio, 2);
            }
            AnimationState::Attacking3 => {
                self.play_attack_sound(audio, 3);
            }
            AnimationState::Attacking4 => {
                self.play_attack_sound(audio, 4);
            }
            AnimationState::Rolling => {
                audio.play(SoundCategory::Movement, "roll");
            }
            AnimationState::Shoot => {
                audio.play(SoundCategory::Combat, "arrow");
            }
            _ => {}
        }
    }
    
    /// Handle continuous animation states that might need repeated sounds
    fn handle_continuous_state(
        &mut self,
        audio: &AudioSystem,
        state: AnimationState,
        on_ground: bool,
    ) {
        // Handle footstep sounds for walking/running
        if on_ground {
            match state {
                AnimationState::Walking => {
                    if self.footstep_timer >= self.walk_footstep_interval {
                        audio.play_variation(SoundCategory::Movement, "footstep");
                        self.footstep_timer = 0.0;
                    }
                }
                AnimationState::Running => {
                    if self.footstep_timer >= self.run_footstep_interval {
                        audio.play_variation(SoundCategory::Movement, "footstep");
                        self.footstep_timer = 0.0;
                    }
                }
                _ => {}
            }
        }
    }
    
    /// Handle landing after a jump
    fn handle_landing(&mut self, audio: &AudioSystem) {
        audio.play(SoundCategory::Movement, "land");
    }
    
    /// Play attack sound with cooldown and variation
    fn play_attack_sound(&mut self, audio: &AudioSystem, attack_num: u8) {
        if self.attack_timer >= self.attack_interval {
            // Random choice between sword and swing sounds
            if rand::gen_range(0, 2) == 0 {
                audio.play_variation(SoundCategory::Combat, "swing");
            } else {
                audio.play(SoundCategory::Combat, format!("sword_{}", attack_num).as_str());
            }
            self.attack_timer = 0.0;
        }
    }
    
    /// Handle combat hit events
    pub fn handle_hit(&self, audio: &AudioSystem, is_critical: bool) {
        if is_critical {
            audio.play(SoundCategory::Combat, "hit_critical");
        } else {
            audio.play_variation(SoundCategory::Combat, "hit");
        }
    }
    
    /// Handle item collection events
    pub fn handle_item_pickup(&self, audio: &AudioSystem, item_type: &str) {
        match item_type {
            "gem" => audio.play(SoundCategory::Item, "gem_pickup"),
            "artifact" => audio.play(SoundCategory::Item, "artifact_pickup"),
            "powerup" => audio.play(SoundCategory::Item, "powerup"),
            _ => {}
        }
    }
    
    /// Handle enemy death events
    pub fn handle_enemy_death(&self, audio: &AudioSystem) {
        audio.play_variation(SoundCategory::Character, "death");
    }
    
    /// Handle UI events
    pub fn handle_ui_event(&self, audio: &AudioSystem, event_type: &str) {
        match event_type {
            "click" => audio.play(SoundCategory::UI, "click"),
            "hover" => audio.play(SoundCategory::UI, "hover"),
            "select" => audio.play(SoundCategory::UI, "select"),
            "back" => audio.play(SoundCategory::UI, "back"),
            "pause" => audio.play(SoundCategory::UI, "pause"),
            "unpause" => audio.play(SoundCategory::UI, "unpause"),
            _ => {}
        }
    }
}