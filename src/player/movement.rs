use macroquad::prelude::*;
use crate::effects::AnimationState;

pub struct PlayerMovement {
    current_speed: f32,
    walk_time: f32,
    const_walk_speed: f32,
    const_run_speed: f32,
    #[allow(dead_code)]
    const_acceleration: f32,
    #[allow(dead_code)]
    const_deceleration: f32,
}

impl PlayerMovement {
    pub fn new() -> Self {
        Self {
            current_speed: 0.0,
            walk_time: 0.0,
            const_walk_speed: 180.0,
            const_run_speed: 280.0,
            const_acceleration: 600.0,
            const_deceleration: 800.0,
        }
    }

    pub fn reset(&mut self) {
        self.current_speed = 0.0;
        self.walk_time = 0.0;
    }

    pub fn update(&mut self, right_pressed: bool, left_pressed: bool, delta_time: f32, is_running: bool) -> Vec2 {
        let mut movement = Vec2::ZERO;

        // Determine movement direction and update walk timer
        let target_direction = if right_pressed {
            if !is_running {
                self.walk_time += delta_time;
            }
            1.0
        } else if left_pressed {
            if !is_running {
                self.walk_time += delta_time;
            }
            -1.0
        } else {
            self.walk_time = 0.0;
            0.0
        };

        // Calculate target speed magnitude based on walk/run state
        let target_speed_mag = if target_direction != 0.0 {
            if is_running || self.walk_time > 0.7 {
                self.const_run_speed  // Either already running or walked long enough
            } else {
                self.const_walk_speed
            }
        } else {
            0.0
        };

        // Apply direction to get actual target speed
        let target_speed = target_speed_mag * target_direction;

        // Smoothly adjust current speed using lerp-like interpolation
        if target_speed * self.current_speed < 0.0 {  // Different signs = changing direction
            // Reset walk timer when changing directions
            self.walk_time = 0.0;
            // Smooth direction change with interpolation
            let smooth_factor = 4.0 * delta_time;  // Reduced for even smoother direction changes
            self.current_speed = self.current_speed * (1.0 - smooth_factor);
            
            // If speed is very small when changing direction, use a more gradual threshold
            if self.current_speed.abs() < 5.0 {  // Lower threshold for quicker direction changes
                self.current_speed = 0.0;
            }
        } else {
            // Regular movement interpolation with smoother factors
            let smooth_factor = if target_speed.abs() > self.current_speed.abs() {
                // Acceleration - slightly faster to compensate for quicker stops
                3.0 * delta_time
            } else {
                // Deceleration - much higher to reduce sliding
                6.0 * delta_time
            };
            
            // Interpolate towards target speed
            self.current_speed = self.current_speed * (1.0 - smooth_factor) + target_speed * smooth_factor;
            
            // More gradual overshooting prevention
            if (target_speed > 0.0 && self.current_speed > target_speed * 1.01) ||
               (target_speed < 0.0 && self.current_speed < target_speed * 1.01) {
                self.current_speed = target_speed;
            }
        }

        // Snap to zero if very small speed and no input - lower threshold for tighter control
        if self.current_speed.abs() < 10.0 && target_speed == 0.0 {
            self.current_speed = 0.0;
        }

        movement.x = self.current_speed;
        movement
    }

    pub fn get_movement_state(&self) -> AnimationState {
        if self.current_speed.abs() > self.const_walk_speed + 20.0 {
            // More gradual transition to running state
            // Using smaller threshold but with existing speed buffer
            AnimationState::Running
        } else if self.current_speed.abs() > 15.0 {
            // Lower threshold for walking to make transitions smoother
            // This creates a more natural progression from idle to walk
            AnimationState::Walking
        } else {
            // Small deadzone for idle to prevent jitter
            AnimationState::Idle
        }
    }
}