use macroquad::prelude::*;

pub struct GameBackground {
    texture: Texture2D,
    position: Vec2,
}

impl GameBackground {
    pub async fn new() -> Option<Self> {
        match load_texture("assets/Backgrounds/Background_00.png").await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                Some(Self {
                    texture,
                    position: Vec2::new(0.0, 0.0),
                })
            }
            Err(err) => {
                eprintln!("Error: Failed to load gameplay background: {}", err);
                None
            }
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, player_velocity: f32, delta_time: f32) {
        if player_velocity != 0.0 {
            self.position.x -= player_velocity * 0.05 * delta_time;

            // Wrap the background position to prevent gaps
            let scale_x = screen_width() / self.texture.width();
            let scaled_width = self.texture.width() * scale_x;

            // If we've scrolled too far left or right, wrap around
            if self.position.x <= -scaled_width {
                self.position.x += scaled_width;
            } else if self.position.x >= scaled_width {
                self.position.x -= scaled_width;
            }
        }
    }

    pub fn draw(&self) {
        // Calculate scaling to ensure background covers full screen with some overflow
        let scale_x = (screen_width() / self.texture.width()) * 1.2; // Add 20% extra width
        let scale_y = screen_height() / self.texture.height();
        let scale = scale_x.max(scale_y);

        let scaled_width = self.texture.width() * scale;
        let scaled_height = self.texture.height() * scale;

        // Draw main background
        draw_texture_ex(
            &self.texture,
            self.position.x,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_width, scaled_height)),
                ..Default::default()
            },
        );

        // Draw a second copy to ensure continuous scrolling
        draw_texture_ex(
            &self.texture,
            self.position.x + scaled_width,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_width, scaled_height)),
                ..Default::default()
            },
        );
    }
}
