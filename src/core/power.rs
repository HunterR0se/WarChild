use macroquad::prelude::*;

#[derive(Debug)]
pub struct Power {
    pub current_power: f32,
    pub max_power: f32,
    // Texture fields for power bar (using LoadingBar_2 series)
    bar_background: Option<Texture2D>,
    bar_fill_blue: Option<Texture2D>,
}

impl Default for Power {
    fn default() -> Self {
        Self {
            current_power: 100.0,
            max_power: 100.0,
            bar_background: None,
            bar_fill_blue: None,
        }
    }
}

impl Power {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load_textures(&mut self) {
        self.bar_background = Some(load_texture("assets/Gui/Bars/LoadingBar_2_Background.png").await.unwrap());
        self.bar_fill_blue = Some(load_texture("assets/Gui/Bars/LoadingBar_2_Fill_Blue.png").await.unwrap());
    }

    pub fn use_power(&mut self, amount: f32) -> bool {
        if self.current_power >= amount {
            self.current_power = (self.current_power - amount).max(0.0);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn add_power(&mut self, amount: f32) {
        self.current_power = (self.current_power + amount).min(self.max_power);
    }

pub fn draw_power_bar(&self, position: Vec2) {
        const BAR_WIDTH: f32 = 200.0;
        const BAR_HEIGHT: f32 = 20.0;
        const FILL_INSET: f32 = 2.0;  // Inset the fill by 2 pixels on each side
        const FILL_Y_OFFSET: f32 = 1.0;  // Move fill up by 1 pixel

        // Draw background bar first
        if let Some(ref bg_texture) = self.bar_background {
            draw_texture_ex(
                bg_texture,
                position.x,
                position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
                    ..Default::default()
                }
            );
        }

        // Draw blue fill bar
        if let Some(ref fill_texture) = self.bar_fill_blue {
            let power_percent = self.current_power / self.max_power;
            let fill_width = (BAR_WIDTH - FILL_INSET * 2.0) * power_percent;
            
            // Source rectangle should be in pixels of original texture
            let texture_width = fill_texture.width();
            let source_width = texture_width * power_percent;
            
            draw_texture_ex(
                fill_texture,
                position.x + FILL_INSET,  // Offset by inset amount
                position.y + FILL_INSET - FILL_Y_OFFSET,  // Offset by inset minus 1 pixel
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(fill_width, BAR_HEIGHT - FILL_INSET * 2.0)),
                    source: Some(Rect::new(0.0, 0.0, source_width, fill_texture.height())),
                    ..Default::default()
                }
            );
        }
    }
}
