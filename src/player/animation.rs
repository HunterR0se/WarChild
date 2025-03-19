use macroquad::prelude::*;
use crate::effects::{Animation, AnimationState, AnimationManager};

pub struct PlayerAnimation {
    animation_manager: Option<AnimationManager>,
    #[allow(dead_code)]
    flash_timer: f32,
    #[allow(dead_code)]
    is_flashing: bool,
    #[allow(dead_code)]
    original_scale: Vec2,
    #[allow(dead_code)]
    target_scale: Vec2,
    #[allow(dead_code)]
    scale_duration: f32,
    #[allow(dead_code)]
    is_scaling: bool,
    #[allow(dead_code)]
    current_scale: Vec2,
    #[allow(dead_code)]
    flash_color: Color,
}

impl PlayerAnimation {
    pub async fn new() -> Self {
        Self {
            animation_manager: None,
            flash_timer: 0.0,
            is_flashing: false,
            original_scale: Vec2::ONE,
            target_scale: Vec2::ONE,
            scale_duration: 0.0,
            is_scaling: false,
            current_scale: Vec2::ONE,
            flash_color: WHITE,
        }
    }

    pub async fn initialize(&mut self) -> bool {
        if self.animation_manager.is_none() {
            let mut manager = AnimationManager::new();
            let mut anim = Animation::new().await.unwrap();
            
            // Set FPS for each animation state
            anim.set_state_fps(AnimationState::Idle, 6.0);      // Slower for more natural idle
            anim.set_state_fps(AnimationState::Walking, 10.0);  // Slightly slower for smoother walk
            anim.set_state_fps(AnimationState::Running, 12.0);  // Reduced from 15.0 for smoother run
            anim.set_state_fps(AnimationState::Jumping, 5.0);   // Significantly reduced for more natural jump timing
            anim.set_state_fps(AnimationState::Attacking1, 10.0); // Combat speeds unchanged
            anim.set_state_fps(AnimationState::Attacking2, 10.0);
            anim.set_state_fps(AnimationState::Attacking3, 10.0);
            anim.set_state_fps(AnimationState::Attacking4, 10.0);
            anim.set_state_fps(AnimationState::Hanging, 6.0);   // Slower for more stable hanging
            anim.set_state_fps(AnimationState::PullUp, 10.0);   // Slightly reduced for clearer motion
            anim.set_state_fps(AnimationState::Shoot, 8.0);     // Slower for clear shooting motion
            anim.set_state_fps(AnimationState::Rolling, 8.0);   // Slowed down for clearer roll animation
            
            manager.set_animation(anim);
            self.animation_manager = Some(manager);
            true
        } else {
            false
        }
    }

    pub fn get_animation(&mut self) -> Option<&mut Animation> {
        if let Some(manager) = &mut self.animation_manager {
            manager.get_animation()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_manager(&mut self) -> Option<&mut AnimationManager> {
        self.animation_manager.as_mut()
    }

    pub fn is_initialized(&self) -> bool {
        self.animation_manager.is_some()
    }
    
    pub fn reset(&mut self) {
        self.animation_manager = None;
    }

    // New methods for flash and scale effects
    #[allow(dead_code)]
    pub fn flash_white(&mut self) {
        self.is_flashing = true;
        self.flash_timer = 0.0;
        self.flash_color = WHITE;
    }

    #[allow(dead_code)]
    pub fn grow_by_percent(&mut self, percent: f32) {
        self.is_scaling = true;
        self.scale_duration = 0.0;
        self.original_scale = self.current_scale;
        self.target_scale = self.original_scale * (1.0 + percent / 100.0);
    }

    #[allow(dead_code)]
    pub fn update_effects(&mut self, dt: f32) {
        // Update flash effect
        if self.is_flashing {
            self.flash_timer += dt;
            if self.flash_timer >= 0.1 { // Flash duration: 0.1 seconds
                self.is_flashing = false;
                self.flash_timer = 0.0;
            }
        }

        // Update scale effect
        if self.is_scaling {
            self.scale_duration += dt;
            let scale_progress = (self.scale_duration / 0.2).min(1.0); // Scale duration: 0.2 seconds
            
            self.current_scale = Vec2::lerp(
                self.original_scale,
                self.target_scale,
                scale_progress
            );

            if scale_progress >= 1.0 {
                self.is_scaling = false;
                self.scale_duration = 0.0;
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_current_scale(&self) -> Vec2 {
        self.current_scale
    }

    #[allow(dead_code)]
    pub fn get_flash_color(&self) -> Option<Color> {
        if self.is_flashing {
            Some(self.flash_color)
        } else {
            None
        }
    }

    pub fn get_current_state(&self) -> AnimationState {
        if let Some(manager) = &self.animation_manager {
            // The AnimationManager has its own get_current_state method:
            return manager.get_current_state();
        }
        AnimationState::Idle // Default to Idle if no animation is available
    }
}