use crate::assets::GameTextures;
use crate::effects::Projectile;
use crate::enemies::types::EnemyType;
use crate::game::state::{GameState, GameStateManager};
use macroquad::prelude::*;

pub struct StateHandler {
    pub state_manager: GameStateManager,
    game_textures: GameTextures,
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            state_manager: GameStateManager::new(),
            game_textures: GameTextures::new(),
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state_manager.get_state()
    }

    pub fn is_state(&self, state: GameState) -> bool {
        self.state_manager.is_state(state)
    }

    pub async fn handle_loading_state(&mut self) -> bool {
        // Load required assets
        if self.game_textures.load_menu_assets().await {
            // Load UI bar assets
            self.game_textures
                .add_texture(
                    "health_bar_bg",
                    "assets/Gui/Bars/LoadingBar_1_Background.png",
                )
                .await;
            self.game_textures
                .add_texture(
                    "health_bar_green",
                    "assets/Gui/Bars/LoadingBar_1_Fill_Green.png",
                )
                .await;
            self.game_textures
                .add_texture(
                    "health_bar_red",
                    "assets/Gui/Bars/LoadingBar_1_Fill_Red.png",
                )
                .await;
            self.game_textures
                .add_texture(
                    "power_bar_bg",
                    "assets/Gui/Bars/LoadingBar_2_Background.png",
                )
                .await;
            self.game_textures
                .add_texture(
                    "power_bar_blue",
                    "assets/Gui/Bars/LoadingBar_2_Fill_Blue.png",
                )
                .await;
            self.game_textures
                .add_texture("game_over_background", "assets/Background/game_over.png")
                .await;

            self.state_manager.set_state(GameState::Menu);
            return true;
        }

        // Draw loading text
        draw_text(
            "Loading...",
            screen_width() * 0.5 - 50.0,
            screen_height() * 0.5,
            30.0,
            WHITE,
        );

        false
    }

    pub fn handle_menu_state(&mut self) {
        // Draw menu background if loaded
        if let Some(menu_bg) = self.game_textures.get("menu_background") {
            let bg_size = Vec2::new(1200.0, 600.0);
            let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
            let bg_pos = Vec2::new(
                screen_center.x - bg_size.x * 0.5,
                screen_center.y - bg_size.y * 0.5 - 100.0, // Move up by 100 pixels from center
            );
            draw_texture_ex(
                menu_bg,
                bg_pos.x,
                bg_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(bg_size),
                    ..Default::default()
                },
            );
        }

        draw_text(
            "Press ENTER to start",
            screen_width() * 0.5 - 100.0,
            screen_height() * 0.65,
            20.0,
            WHITE,
        );
        draw_text(
            "Press Q to quit",
            screen_width() * 0.5 - 100.0,
            screen_height() * 0.65 + 30.0,
            20.0,
            WHITE,
        );

        // Draw legend box
        let legend_x = screen_width() * 0.5 - 300.0; // 600px wide, centered
        let legend_y = screen_height() * 0.7; // Position below menu options

        // Draw semi-transparent black background with gold border
        draw_rectangle(
            legend_x,
            legend_y,
            600.0,
            200.0,
            Color::new(0.0, 0.0, 0.0, 0.7),
        );
        draw_rectangle_lines(
            legend_x,
            legend_y,
            600.0,
            200.0,
            2.0,
            Color::new(1.0, 0.84, 0.0, 0.3),
        );

        // Draw legend text
        draw_text("Controls", legend_x + 250.0, legend_y + 30.0, 24.0, GOLD);

        let controls = [
            ("Move Left/Right", "LEFT/RIGHT or A/D"),
            ("Jump/Climb", "UP or W"),
            ("Shoot/Drop", "DOWN or S"),
            ("Attack", "SPACE"),
            ("Roll", "CTRL+LEFT/RIGHT"),
            ("Pause", "ESC or P"),
        ];

        for (i, (action, key)) in controls.iter().enumerate() {
            let y_pos = legend_y + 60.0 + (i as f32 * 25.0);
            draw_text(action, legend_x + 30.0, y_pos, 20.0, GOLD);
            // Calculate position to align keys based on longest action text
            let key_x = legend_x + 220.0; // More space between columns
            draw_text(key, key_x, y_pos, 20.0, WHITE);
        }

        if is_key_pressed(KeyCode::Enter) {
            self.state_manager.set_state(GameState::Playing);
            // NOTE: World parameter will come from main's world instance
        } else if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }
    }

    pub fn handle_paused_state(&mut self) {
        draw_text(
            "PAUSED",
            screen_width() * 0.5 - 50.0,
            screen_height() * 0.5,
            30.0,
            WHITE,
        );
        draw_text(
            "ESC - Resume, Q - Quit to Menu",
            screen_width() * 0.5 - 120.0,
            screen_height() * 0.5 + 40.0,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Escape) {
            self.state_manager.set_state(GameState::Playing);
        }
    }

    pub fn handle_game_over_state(&mut self) {
        // Draw game over background if loaded
        if let Some(game_over_bg) = self.game_textures.get("game_over_background") {
            let bg_size = Vec2::new(1200.0, 600.0);
            let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
            let bg_pos = Vec2::new(
                screen_center.x - bg_size.x * 0.5,
                screen_center.y - bg_size.y * 0.5,
            );
            draw_texture_ex(
                game_over_bg,
                bg_pos.x,
                bg_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(bg_size),
                    ..Default::default()
                },
            );
        }

        draw_text(
            "Press ENTER to return to menu",
            screen_width() * 0.5 - 140.0,
            screen_height() * 0.65,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Enter) {
            self.state_manager.set_state(GameState::Menu);
        }
    }

    pub fn reset_all(&mut self) {
        self.state_manager.reset_all();
    }

    pub fn get_next_enemy_type(&mut self) -> Option<EnemyType> {
        self.state_manager.get_next_enemy_type()
    }

    pub fn mark_enemy_spawned(&mut self) {
        self.state_manager.mark_enemy_spawned()
    }

    pub fn mark_enemy_dead(&mut self) {
        self.state_manager.mark_enemy_dead()
    }

    pub fn should_spawn(&mut self) -> bool {
        self.state_manager.should_spawn()
    }

    pub fn mark_spawned(&mut self) {
        self.state_manager.mark_spawned()
    }

    #[allow(dead_code)]
    pub fn get_textures(&self) -> &GameTextures {
        &self.game_textures
    }

    pub fn draw_gems(&self) {
        for gem in self.state_manager.get_active_gems() {
            gem.draw();
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.state_manager.update(dt);
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state_manager.set_state(state);
    }

    // Projectile Management
    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.state_manager.add_projectile(projectile);
    }

    pub fn update_projectiles(&mut self, dt: f32) {
        self.state_manager.update_projectiles(dt);
    }

    pub fn draw_projectiles(&self) {
        for projectile in &self.state_manager.projectiles {
            projectile.draw();
        }
    }
}
