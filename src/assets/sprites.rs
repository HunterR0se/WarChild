use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture2D,
    frame_width: f32,
    frame_height: f32,
    frames: usize,
    current_frame: usize,
    animation_timer: f32,
    frame_time: f32,  // Time per frame in seconds
    loop_animation: bool,
}

impl SpriteSheet {
    #[allow(dead_code)]
    pub async fn new(path: &str, frame_width: f32, frame_height: f32, frames: usize) -> Option<Self> {
        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);  // Set nearest filtering for pixel art
                
                // Calculate frames if 0 was passed (auto-detect from texture width)
                let actual_frames = if frames == 0 {
                    (texture.width() / frame_width as f32) as usize
                } else {
                    frames
                };
                
                Some(Self {
                    texture,
                    frame_width,
                    frame_height,
                    frames: actual_frames,
                    current_frame: 0,
                    animation_timer: 0.0,
                    frame_time: 0.1, // Default 10 FPS animation
                    loop_animation: true,
                })
            },
            Err(_) => None,
        }
    }

    // New synchronous constructor for when we already have a texture
    pub fn from_texture(texture: Texture2D, frame_width: f32, frame_height: f32) -> Self {
        texture.set_filter(FilterMode::Nearest);  // Set nearest filtering for pixel art
        
        // Auto-detect frames from texture width
        let texture_width = texture.width();
        let frames = (texture_width / frame_width as f32) as usize;
                
        Self {
            texture,
            frame_width,
            frame_height,
            frames,
            current_frame: 0,
            animation_timer: 0.0,
            frame_time: 0.1, // Default 10 FPS animation
            loop_animation: true,
        }
    }

    pub fn set_animation_fps(&mut self, fps: f32) {
        self.frame_time = 1.0 / fps;
    }

    pub fn set_looping(&mut self, should_loop: bool) {
        self.loop_animation = should_loop;
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_timer += dt;
        if self.animation_timer >= self.frame_time {
            self.animation_timer = 0.0;  // Reset timer
            if self.current_frame < self.frames - 1 {
                self.current_frame += 1;  // Advance to next frame
            } else if self.loop_animation {
                self.current_frame = 0;  // Loop back to start
            } else {
                // Stay on last frame
            }
        }
    }

    pub fn force_frame(&mut self, frame: usize) {
        self.current_frame = frame;  // Allow setting any frame
        self.animation_timer = 0.0;
    }

    #[allow(dead_code)]
    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }

    pub fn draw(&self, position: Vec2, flip_x: bool, scale: Vec2, color: Color) {
        // Calculate source rectangle for current frame
        let src_x = (self.current_frame as f32) * self.frame_width;
        let src_rect = Rect::new(
            src_x,
            0.0,
            self.frame_width,
            self.frame_height
        );

        // Draw the current frame
        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.frame_width * scale.x, self.frame_height * scale.y)),
                source: Some(src_rect),
                rotation: 0.0,
                flip_x,
                flip_y: false,
                pivot: None,
            },
        );
    }

    pub fn get_frame_count(&self) -> usize {
        self.frames
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    #[allow(dead_code)]
    pub fn is_on_last_frame(&self) -> bool {
        self.current_frame == self.frames - 1
    }

    pub fn advance_frame(&mut self) -> bool {
        // Returns true if we advanced, false if we're at the end
        if self.current_frame + 1 < self.frames {
            self.current_frame += 1;
            true
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        // Only consider finished if we're exactly on the final frame (frames - 1)
        !self.loop_animation && self.current_frame == self.frames - 1
    }
}