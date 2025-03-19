use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
    pub is_taking_damage: bool,
    pub damage_flash_timer: f32,
    // Texture fields for health bar
    bar_background: Option<Texture2D>,
    bar_fill_green: Option<Texture2D>,
    bar_fill_red: Option<Texture2D>,
    is_enemy: bool, // Track if this is an enemy health bar
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current_health: 100.0,
            max_health: 100.0,
            is_taking_damage: false,
            damage_flash_timer: 0.0,
            bar_background: None,
            bar_fill_green: None,
            bar_fill_red: None,
            is_enemy: false,
        }
    }
}

impl Health {
    pub fn new() -> Self {
        Self::default()
    }

    // Load the health bar textures (for player)
    pub async fn load_textures(&mut self) {
        self.is_enemy = false;
        self.bar_background = Some(
            load_texture("assets/Gui/Bars/LoadingBar_1_Background.png")
                .await
                .unwrap(),
        );
        self.bar_fill_green = Some(
            load_texture("assets/Gui/Bars/LoadingBar_1_Fill_Green.png")
                .await
                .unwrap(),
        );
        self.bar_fill_red = Some(
            load_texture("assets/Gui/Bars/LoadingBar_1_Fill_Red.png")
                .await
                .unwrap(),
        );
    }

    // Load custom health bar textures (for enemies)
    pub async fn load_textures_with_paths(
        &mut self,
        bg_path: &str,
        green_path: &str,
        red_path: &str,
    ) {
        self.is_enemy = true;
        self.bar_background = if !bg_path.is_empty() {
            Some(load_texture(bg_path).await.unwrap())
        } else {
            None
        };
        self.bar_fill_green = if !green_path.is_empty() {
            Some(load_texture(green_path).await.unwrap())
        } else {
            None
        };
        self.bar_fill_red = if !red_path.is_empty() {
            Some(load_texture(red_path).await.unwrap())
        } else {
            None
        };
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.current_health = (self.current_health - amount).max(0.0);
        // Only flash the health bar for enemies, not the player
        if self.is_enemy {
            self.is_taking_damage = true;
            self.damage_flash_timer = 0.1; // 100ms flash duration
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_taking_damage {
            self.damage_flash_timer -= dt;
            if self.damage_flash_timer <= 0.0 {
                self.is_taking_damage = false;
                self.damage_flash_timer = 0.0;
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        self.current_health > 0.0
    }

    #[allow(dead_code)]
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0.0
    }

    // Draw the health bar at the specified position
    pub fn draw_health_bar_with_size(&self, position: Vec2, size: Vec2) {
        let health_percent = self.current_health / self.max_health;

        // Choose color based on health percentage (red when <= 30%)
        let base_color = if health_percent > 0.3 {
            Color::new(0.15, 0.6, 0.15, 1.0) // Medium-dark green
        } else {
            Color::new(0.8, 0.2, 0.2, 1.0) // Medium red
        };

        // If taking damage, flash the bar by alternating alpha
        let color = if self.is_taking_damage {
            let flash_alpha = (self.damage_flash_timer * 10.0).sin() * 0.5 + 0.5;
            Color::new(base_color.r, base_color.g, base_color.b, flash_alpha)
        } else {
            base_color
        };

        // Draw the health bar as a simple rectangle
        let fill_width = size.x * health_percent;
        draw_rectangle(position.x, position.y, fill_width, size.y, color);
    }

    // Draw the health bar at the specified position
    pub fn draw_health_bar(&self, position: Vec2) {
        const BAR_WIDTH: f32 = 200.0;
        const BAR_HEIGHT: f32 = 20.0;
        const FILL_INSET: f32 = 2.0; // Inset the fill by 2 pixels on each side
        const FILL_Y_OFFSET: f32 = 1.0; // Move fill up by 1 pixel

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
                },
            );
        }

        // Choose fill color based on health percentage (red when <= 30%)
        let health_percent = self.current_health / self.max_health;
        let fill_texture = if health_percent > 0.3 {
            self.bar_fill_green.as_ref()
        } else {
            self.bar_fill_red.as_ref()
        };

        // Draw the fill bar
        if let Some(fill_texture) = fill_texture {
            let fill_width = (BAR_WIDTH - FILL_INSET * 2.0) * health_percent;

            // Source rectangle should be in pixels of original texture
            let texture_width = fill_texture.width();
            let source_width = texture_width * health_percent;

            // If taking damage, flash the bar by alternating alpha
            let color = if self.is_taking_damage {
                let flash_alpha = (self.damage_flash_timer * 10.0).sin() * 0.5 + 0.5;
                Color::new(1.0, 1.0, 1.0, flash_alpha)
            } else {
                WHITE
            };

            draw_texture_ex(
                fill_texture,
                position.x + FILL_INSET, // Offset by inset amount
                position.y + FILL_INSET - FILL_Y_OFFSET, // Offset by inset minus 1 pixel
                color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(fill_width, BAR_HEIGHT - FILL_INSET * 2.0)),
                    source: Some(Rect::new(0.0, 0.0, source_width, fill_texture.height())),
                    ..Default::default()
                },
            );
        }
    }
}
