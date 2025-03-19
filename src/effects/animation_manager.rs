use macroquad::prelude::*;
use super::{Animation, AnimationState};

pub struct AnimationManager {
    current_animation: Option<Animation>,
    current_state: AnimationState,
    #[allow(dead_code)]
    collection_scale: f32,  // 1.0 -> 1.05 during collection
    #[allow(dead_code)]
    collection_timer: f32,  // 0.1s duration
    #[allow(dead_code)]
    collecting: bool,       // Track if collecting a gem
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            current_animation: None,
            current_state: AnimationState::Idle,
            collection_scale: 1.0,
            collection_timer: 0.0,
            collecting: false,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, dt: f32) {
        if let Some(anim) = &mut self.current_animation {
            anim.update(dt);
        }

        // Handle collection scaling
        if self.collecting {
            self.collection_timer += dt;
            
            // First half: scale up over 0.1s
            if self.collection_timer < 0.1 {
                self.collection_scale = 1.0 + (self.collection_timer / 0.1) * 0.15;
            } else {
                // Second half: scale down over 0.1s
                self.collection_scale = 1.15 - ((self.collection_timer - 0.1) / 0.1) * 0.15;
            }

            // End collection after 0.2s total
            if self.collection_timer >= 0.2 {
                self.collecting = false;
                self.collection_scale = 1.0;
            }
        }
    }

    #[allow(dead_code)]
    pub fn trigger_collection(&mut self) {
        self.collecting = true;
        self.collection_timer = 0.0;
        self.collection_scale = 1.0;
    }

    pub fn get_current_state(&self) -> AnimationState {
        self.current_state
    }

    #[allow(dead_code)]
    pub fn set_state(&mut self, state: AnimationState) {
        self.current_state = state;
        if let Some(anim) = &mut self.current_animation {
            anim.set_state(state);
        }
    }

    #[allow(dead_code)]
    pub fn is_in_state(&self, state: &AnimationState) -> bool {
        if let Some(anim) = &self.current_animation {
            anim.is_in_state(state)
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn force_frame(&mut self, frame: usize) {
        if let Some(anim) = &mut self.current_animation {
            anim.force_frame(frame);
        }
    }
    
    #[allow(dead_code)]
    pub fn get_current_frame(&self) -> Option<usize> {
        if let Some(anim) = &self.current_animation {
            anim.get_current_frame()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self, pos: Vec2, facing_left: bool, mut scale: Vec2, color: Color) {
        if let Some(anim) = &self.current_animation {
            // Apply collection scale if collecting
            if self.collecting {
                scale.x *= self.collection_scale;
                scale.y *= self.collection_scale;
            }
            anim.draw(pos, facing_left, scale, color);
        }
    }

    pub fn set_animation(&mut self, animation: Animation) {
        self.current_animation = Some(animation);
    }

    pub fn get_animation(&mut self) -> Option<&mut Animation> {
        self.current_animation.as_mut()
    }
}