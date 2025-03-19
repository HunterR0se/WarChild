use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum CollisionType {
    Walkable,    // Can stand/walk on
    Vertical,    // Can wall-jump/climb  
    Connection   // Connection point with other platforms
}

#[derive(Clone)]
pub struct CollisionArea {
    pub area_type: CollisionType,
    pub bounds: (Vec2, Vec2),  // (top-left, bottom-right)
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum PlatformType {
    LeftWall,     // Anchored to bottom
    LeftSpire,    // Bridge between LeftWall and Center
    Center,       // Anchored to bottom
    Floating,     // Can float anywhere
    RightLedge,   // Anchored to right side
    CenterSpire,  // Creative path platform
    RightFloat,   // Additional floating platform
    RightSideFloat, // Side floating platform
    CenterTower,  // Main tower with vertical climbing
    Middle,       // Basic connecting platform
    LeftFading,   // TO BE DOCUMENTED
    Right,        // TO BE DOCUMENTED
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum DecorationType {
    Tree(TreeType),
    House(HouseType),
    Lamp,  // Single lamp type - no variants needed
    Bobble,
    Branch,
    Elevator,
    Fence,
    Hill,
    Ladder,
    Light,
    Rope,
    Saw,
    Spawn,
    Spikes,
    Stairs,
    Tombstones,
    Web,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum TreeType {
    Normal,
    Dying,
    Joshua,
    Twisted,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum HouseType {
    Big,
    Mid,
    Round,
    Triangle,
}

impl PlatformType {
    fn get_texture_path(&self) -> &str {
        match self {
            PlatformType::LeftWall => "assets/Platform/LeftWall.png",
            PlatformType::LeftSpire => "assets/Platform/LeftSpire.png",
            PlatformType::Center => "assets/Platform/Center.png", 
            PlatformType::Floating => "assets/Platform/Floating.png",
            PlatformType::RightLedge => "assets/Platform/RightLedge.png",
            PlatformType::CenterSpire => "assets/Platform/CenterSpire.png",
            PlatformType::RightFloat => "assets/Platform/RightFloat.png",
            PlatformType::RightSideFloat => "assets/Platform/RightSideFloat.png",
            PlatformType::CenterTower => "assets/Platform/CenterTower.png",
            PlatformType::Middle => "assets/Platform/Middle.png",
            PlatformType::LeftFading => "assets/Platform/LeftFading.png",
            PlatformType::Right => "assets/Platform/Right.png",
        }
    }

    fn should_anchor_bottom(&self) -> bool {
        match self {
            PlatformType::Floating => false,
            PlatformType::RightLedge => false, // RightLedge will have custom positioning
            PlatformType::CenterSpire => false, // CenterSpire has custom positioning
            PlatformType::RightFloat => false,  // RightFloat has custom positioning
            PlatformType::RightSideFloat => false, // RightSideFloat has custom positioning
            PlatformType::LeftSpire => false,   // LeftSpire has custom positioning
            _ => true  // All other platforms should be anchored to bottom
        }
    }

    fn get_collision_areas(&self) -> Vec<CollisionArea> {
        match self {
            PlatformType::LeftWall => vec![
                // Main top platform - match original bounding box top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(319.0, 44.0)),
                },
                // Wall climbing surface - moved left to match wall
                CollisionArea {
                    area_type: CollisionType::Vertical, 
                    bounds: (Vec2::new(0.0, 44.0), Vec2::new(32.0, 705.0)),
                },
                // Right connection points (visual only)
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(319.0, 12.0), Vec2::new(319.0, 44.0)),
                },
            ],
            PlatformType::Center => vec![
                // Main top platform (walkable) - aligned with original red box top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(650.0, 44.0)),
                },
                // Left pillar (vertical climbing) - at visible pillar edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(80.0, 44.0), Vec2::new(112.0, 390.0)),
                },
                // Right pillar (vertical climbing) - at visible pillar edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(538.0, 44.0), Vec2::new(570.0, 390.0)),
                },
                // Left connection points (visual only)
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                // Right connection points (visual only)
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(650.0, 12.0), Vec2::new(650.0, 44.0)),
                },
            ],
            PlatformType::CenterSpire => vec![
                // Right side walkable area - starts earlier, matches visible ledge
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(175.0, 12.0), Vec2::new(415.0, 44.0)),
                },
                // Left side walkable area - aligned with bottom of right side
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 44.0), Vec2::new(192.0, 76.0)), // Starts at y=44 where right side ends
                },
                // Main spire for climbing - wider to match actual spire width
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(175.0, 44.0), Vec2::new(240.0, 445.0)),
                },
                // Connection points at actual ledge heights
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 44.0), Vec2::new(0.0, 76.0)), // Aligned with walkable area
                },
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(415.0, 12.0), Vec2::new(415.0, 44.0)),
                },
            ],
            PlatformType::RightLedge => vec![
                // Main walkable platform
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(588.0, 44.0)),
                },
                // Right wall for climbing
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(556.0, 44.0), Vec2::new(588.0, 228.0)),
                },
                // Left connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
            ],
            PlatformType::Floating => vec![
                // Main platform - full width, standard height from texture top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(348.0, 44.0)),
                },
                // Connection points for possible platform joining
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(348.0, 12.0), Vec2::new(348.0, 44.0)),
                },
            ],
            PlatformType::RightSideFloat | PlatformType::RightFloat => vec![
                // Main platform - full width, standard height from texture top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(260.0, 44.0)),
                },
                // Connection points for possible platform joining
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(260.0, 12.0), Vec2::new(260.0, 44.0)),
                },
            ],
            PlatformType::LeftSpire => vec![
                // Main walkable platform - matches original bounding box top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(231.0, 44.0)),
                },
                // Main spire for climbing - at visible spire edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(179.0, 44.0), Vec2::new(211.0, 254.0)),
                },
                // Left connection points (visual only)
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                // Right connection points (visual only)
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(231.0, 12.0), Vec2::new(231.0, 44.0)),
                },
            ],
            PlatformType::CenterTower => vec![
                // Top platform 607x567 - matches original red box top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(607.0, 44.0)),
                },
                // Main tower for climbing - at visible tower edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(288.0, 44.0), Vec2::new(320.0, 535.0)),
                },
                // Left connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                // Right connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(607.0, 12.0), Vec2::new(607.0, 44.0)),
                },
            ],
            PlatformType::Middle => vec![
                // Main platform 526x124 - matches original red box top
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(526.0, 44.0)),
                },
                // Left connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
                // Right connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(526.0, 12.0), Vec2::new(526.0, 44.0)),
                },
            ],
            // The remaining platforms need collision documentation first
            PlatformType::LeftFading => vec![
                // Main platform: matches standard walkable height
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(304.0, 44.0)),
                },
                // Left wall for climbing - matches visible edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(0.0, 44.0), Vec2::new(32.0, 226.0)),
                },
                // Right connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(304.0, 12.0), Vec2::new(304.0, 44.0)),
                },
            ],
            PlatformType::Right => vec![
                // Main platform: matches standard walkable height
                CollisionArea {
                    area_type: CollisionType::Walkable,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(273.0, 44.0)),
                },
                // Right wall for climbing - matches visible edge
                CollisionArea {
                    area_type: CollisionType::Vertical,
                    bounds: (Vec2::new(241.0, 44.0), Vec2::new(273.0, 166.0)),
                },
                // Left connection point
                CollisionArea {
                    area_type: CollisionType::Connection,
                    bounds: (Vec2::new(0.0, 12.0), Vec2::new(0.0, 44.0)),
                },
            ],
        }
    }
}

impl DecorationType {
    fn get_texture_path(&self) -> &str {
        match self {
            DecorationType::Tree(tree_type) => match tree_type {
                TreeType::Normal => "assets/PlatformAssets/Tree_Normal.png",
                TreeType::Dying => "assets/PlatformAssets/Tree_Dying.png",
                TreeType::Joshua => "assets/PlatformAssets/Tree_Joshua.png",
                TreeType::Twisted => "assets/PlatformAssets/Tree_Twisted.png",
            },
            DecorationType::House(house_type) => match house_type {
                HouseType::Big => "assets/PlatformAssets/House_Big.png",
                HouseType::Mid => "assets/PlatformAssets/House_Mid.png",
                HouseType::Round => "assets/PlatformAssets/House_Round.png",
                HouseType::Triangle => "assets/PlatformAssets/House_Triangle.png",
            },
            DecorationType::Lamp => "assets/PlatformAssets/Lamp.png",
            DecorationType::Bobble => "assets/PlatformAssets/Bobble.png",
            DecorationType::Branch => "assets/PlatformAssets/Branch.png",
            DecorationType::Elevator => "assets/PlatformAssets/Elevator.png",
            DecorationType::Fence => "assets/PlatformAssets/Fence.png",
            DecorationType::Hill => "assets/PlatformAssets/Hill.png",
            DecorationType::Ladder => "assets/PlatformAssets/Ladder.png",
            DecorationType::Light => "assets/PlatformAssets/Light.png",
            DecorationType::Rope => "assets/PlatformAssets/Rope.png",
            DecorationType::Saw => "assets/PlatformAssets/Saw.png",
            DecorationType::Spawn => "assets/PlatformAssets/Spawn.png",
            DecorationType::Spikes => "assets/PlatformAssets/Spikes.png",
            DecorationType::Stairs => "assets/PlatformAssets/Stairs.png",
            DecorationType::Tombstones => "assets/PlatformAssets/Tombstones.png",
            DecorationType::Web => "assets/PlatformAssets/Web.png",
        }
    }

    // Get alt texture path for objects with multiple states
    fn get_alt_texture_path(&self) -> Option<&str> {
        match self {
            DecorationType::Lamp => Some("assets/PlatformAssets/Lamp_Off.png"),
            _ => None,
        }
    }
}

#[derive(Default, Clone)]
struct LampState {
    flicker_timer: f32,
    next_flicker: f32,
    fade_opacity: f32,
    is_lit: bool,
    is_flickering: bool,  // True during flicker animation
    flicker_duration: f32, // How long the current flicker will last
}

impl LampState {
    fn new() -> Self {
        Self {
            flicker_timer: 0.0,
            next_flicker: thread_rng().gen_range(3.0..6.0), // Longer time between flickers
            fade_opacity: 1.0,
            is_lit: true,
            is_flickering: false,
            flicker_duration: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.is_flickering {
            // Handle flicker animation
            self.flicker_timer += dt;
            
            if self.flicker_timer < self.flicker_duration {
                // During flicker, rapidly oscillate opacity
                self.fade_opacity = (self.flicker_timer * 30.0).sin() * 0.5 + 0.5;
            } else {
                // End of flicker
                self.is_flickering = false;
                self.flicker_timer = 0.0;
                self.is_lit = !self.is_lit;  // Switch state at end of flicker
                self.fade_opacity = if self.is_lit { 1.0 } else { 0.0 };
                self.next_flicker = thread_rng().gen_range(3.0..6.0);  // Time until next flicker
            }
        } else {
            // Wait for next flicker
            self.flicker_timer += dt;
            if self.flicker_timer >= self.next_flicker {
                self.is_flickering = true;
                self.flicker_timer = 0.0;
                self.flicker_duration = thread_rng().gen_range(0.2..0.4);  // Random flicker duration
            }
        }
    }
}

pub struct PlatformDecoration {
    #[allow(dead_code)]
    decoration_type: DecorationType,
    position: Vec2,
    texture: Texture2D,
    alt_texture: Option<Texture2D>, // For objects with multiple states (like lamp on/off)
    scale: f32,
    lamp_state: Option<LampState>,
}

impl PlatformDecoration {
    pub async fn new(decoration_type: DecorationType, position: Vec2) -> Option<Self> {
        // Load main texture
        let texture = load_texture(decoration_type.get_texture_path()).await.ok()?;
        
        // Load alt texture if available
        let alt_texture = if let Some(alt_path) = decoration_type.get_alt_texture_path() {
            load_texture(alt_path).await.ok()
        } else {
            None
        };

        Some(Self {
            decoration_type: decoration_type.clone(),
            position,
            texture,
            alt_texture,
            scale: 1.0, // Default scale
            lamp_state: match decoration_type {
                DecorationType::Lamp => Some(LampState::new()),
                _ => None,
            },
        })
    }

    pub fn update(&mut self) {
        // Update lamp state if this is a lamp
        if let Some(lamp_state) = &mut self.lamp_state {
            lamp_state.update(get_frame_time());
        }
    }

    pub fn draw(&self) {
        // Get base texture dimensions for positioning
        let base_width = self.texture.width() * self.scale;
        
        // For lamp, draw both textures with opacity based on state
        if let (Some(lamp_state), Some(alt_tex)) = (&self.lamp_state, &self.alt_texture) {
            // Draw off state (Lamp_Off.png) at the exact position 
            // The left lamp post is our anchor point
            draw_texture_ex(
                alt_tex,
                self.position.x,
                self.position.y,
                Color::new(1.0, 1.0, 1.0, 0.85),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        alt_tex.width() * self.scale,
                        alt_tex.height() * self.scale
                    )),
                    ..Default::default()
                }
            );

            // Draw lit state at the EXACT same position - lamp posts will align
            draw_texture_ex(
                &self.texture,
                self.position.x,  // Same exact position
                self.position.y,
                Color::new(1.0, 1.0, 1.0, 0.85 * lamp_state.fade_opacity),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        self.texture.width() * self.scale,  // Keep original dimensions
                        self.texture.height() * self.scale
                    )),
                    ..Default::default()
                }
            );
        } else {
            // Normal decoration drawing
            draw_texture_ex(
                &self.texture,
                self.position.x,
                self.position.y,
                Color::new(1.0, 1.0, 1.0, 0.85),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(base_width, self.texture.height() * self.scale)),
                    ..Default::default()
                }
            );
        }
    }
}

pub struct Platform {
    texture: Texture2D,
    position: Vec2,
    size: Vec2,
    platform_type: PlatformType,
    decoration: Option<PlatformDecoration>,
    collision_areas: Vec<CollisionArea>,
}

impl Platform {
    pub async fn new(platform_type: PlatformType, mut position: Vec2) -> Option<Self> {
        match load_texture(platform_type.get_texture_path()).await {
            Ok(texture) => {
                let size = Vec2::new(texture.width(), texture.height());
                
                // If platform should be anchored to bottom, adjust Y position
                if platform_type.should_anchor_bottom() {
                    position.y = screen_height() - size.y;
                }
                
                Some(Self {
                    texture,
                    position,
                    size,
                    platform_type: platform_type.clone(),
                    decoration: None,
                    collision_areas: platform_type.get_collision_areas(),
                })
            },
            Err(err) => {
                eprintln!("Error: Failed to load platform texture: {}", err);
                None
            }
        }
    }

    pub async fn add_decoration(&mut self, decoration_type: DecorationType, scale: Option<f32>) -> bool {
        // Calculate visible offset for the type of decoration
        let scale = scale.unwrap_or(1.0);
        let bottom_offset = match &decoration_type {
            DecorationType::Tree(tree_type) => match tree_type {
                TreeType::Joshua => 42.0 * scale, // Joshua tree has more bottom padding
                _ => 22.0 * scale, // Standard trees
            },
            DecorationType::House(_) => 35.0 * scale, // House offset unchanged
            DecorationType::Lamp => 48.0 * scale, // Lamp has large bottom section in sprite
            DecorationType::Bobble => 10.0 * scale,
            DecorationType::Branch => 8.0 * scale,
            DecorationType::Elevator => 0.0,  // Sits directly on platform
            DecorationType::Fence => 5.0 * scale,
            DecorationType::Hill => 0.0,  // Sits directly on platform
            DecorationType::Ladder => 0.0, // Extends down from platform
            DecorationType::Light => 15.0 * scale,
            DecorationType::Rope => 0.0,  // Extends down from platform
            DecorationType::Saw => 5.0 * scale,
            DecorationType::Spawn => 0.0,  // Extends down from platform
            DecorationType::Spikes => 0.0, // Sits directly on platform
            DecorationType::Stairs => 0.0, // Sits directly on platform
            DecorationType::Tombstones => 15.0 * scale,
            DecorationType::Web => 10.0 * scale,
        };
            
        if let Some(decoration) = PlatformDecoration::new(decoration_type.clone(), Vec2::ZERO).await {
            // Position decoration with its visible bottom at platform top
            let pos = Vec2::new(
                // Calculate position based on decoration type
                match &decoration_type {
                    DecorationType::Tree(TreeType::Joshua) => {
                        // Place Joshua Tree on right third
                        self.position.x + (self.size.x * 0.7) - ((decoration.texture.width() * scale) / 2.0)
                    },
                    _ => {
                        // Default left third placement
                        self.position.x + (self.size.x * 0.3) - ((decoration.texture.width() * scale) / 2.0)
                    }
                },
                self.position.y - (decoration.texture.height() * scale) + bottom_offset // Align with offset
            );
            
            self.decoration = Some(PlatformDecoration {
                position: pos,
                scale,
                ..decoration
            });
            true
        } else {
            false
        }
    }

    fn get_collision_box(&self) -> Vec<(Vec2, Vec2)> {
        // Convert local collision areas to world space and filter out Connection areas
        self.collision_areas.iter()
            .filter(|area| matches!(area.area_type, CollisionType::Walkable | CollisionType::Vertical))
            .map(|area| {
                let world_pos = self.position + area.bounds.0;
                let world_end = self.position + area.bounds.1;
                (world_pos, world_end)
            })
            .collect()
    }

    pub fn draw(&mut self) {
        // Draw the platform texture
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            }
        );
        
        // Update and draw decoration if it exists
        if let Some(decoration) = &mut self.decoration {
            decoration.update();
            decoration.draw();
        }
        
        // Always draw spawn box for RightLedge platform
        if self.platform_type == PlatformType::RightLedge {
            let spawn_x = self.position.x + (self.size.x * 0.67);  // Right third
            let spawn_y = self.position.y + 12.0;  // Top of walkable area
            let box_size = 72.0;

            // Draw semi-transparent background
            draw_rectangle(
                spawn_x - 36.0,  // Center box
                spawn_y - box_size,
                box_size,
                box_size,
                Color::new(0.0, 0.0, 0.0, 0.3),
            );

            // Draw box outline
            draw_rectangle_lines(
                spawn_x - 36.0,  // Center box
                spawn_y - box_size,
                box_size,
                box_size,
                2.0,
                PURPLE,
            );

            // Draw "SPAWN" text at top
            draw_text(
                "SPAWN",
                spawn_x - 28.0,  // Center text above box
                spawn_y - box_size - 5.0,
                20.0,
                PURPLE,
            );
        }
        
        // Debug: Draw collision areas
        for area in &self.collision_areas {
            let color = match area.area_type {
                CollisionType::Walkable => GREEN,
                CollisionType::Vertical => BLUE,
                CollisionType::Connection => RED,
            };
            
            // Draw rectangle outline for each collision area
            let pos = self.position + area.bounds.0;
            let size = area.bounds.1 - area.bounds.0;
            draw_rectangle_lines(
                pos.x,
                pos.y,
                size.x,
                size.y,
                2.0,
                color
            );
        }
    }
}

pub struct PlatformManager {
    platforms: Vec<Platform>,
}

impl PlatformManager {
    pub fn new() -> Self {
        Self {
            platforms: Vec::new(),
        }
    }

    pub async fn initialize(&mut self, world: &mut crate::physics::World) {
        // Place LeftWall on the left side
        let left_wall = Platform::new(
            PlatformType::LeftWall,
            Vec2::new(0.0, 0.0)  // Y will be adjusted for bottom anchoring
        ).await;

        if let Some(left_wall) = left_wall {
            // Add lamp decoration to left wall (50% size)
            let mut left_wall = left_wall;
            let _ = left_wall.add_decoration(DecorationType::Lamp, Some(0.5)).await;

            // Add all collision boxes from left wall
            for (pos, end) in left_wall.get_collision_box() {
                let size = end - pos;
                world.add_platform(pos, size);
            }
            self.platforms.push(left_wall);

            // Create scattered platform layout with NO overlaps
            
            // RightLedge (enemy/exit platform) - dramatic high position, slight right adjustment
            if let Some(right_ledge) = Platform::new(
                PlatformType::RightLedge,
                Vec2::new(0.0, screen_height() - 850.0)  // Very high near top
            ).await {
                // Add house decoration to RightLedge (40% size)
                let adjusted_x = screen_width() - right_ledge.size.x + 5.0; // Moved 5px right
                let mut right_ledge = right_ledge;
                right_ledge.position.x = adjusted_x;
                let _ = right_ledge.add_decoration(DecorationType::House(HouseType::Mid), Some(0.4)).await;
                
                // Add collision boxes for right ledge
                for (pos, end) in right_ledge.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(right_ledge);
            }

            // LeftSpire - between LeftWall and Center
            if let Some(left_spire) = Platform::new(
                PlatformType::LeftSpire,
                Vec2::new(100.0, screen_height() - 300.0)  // Moved left another 100px
            ).await {
                // Add collision boxes for left spire
                for (pos, end) in left_spire.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(left_spire);
            }

            // Center platform - lower position
            if let Some(mut center) = Platform::new(
                PlatformType::Center,
                Vec2::new(370.0, screen_height() - 350.0)  // Moved down 30px (was 380)
            ).await {
                // Add normal tree to center platform
                let _ = center.add_decoration(DecorationType::Tree(TreeType::Normal), None).await;
                
                // Add collision boxes for center platform
                for (pos, end) in center.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(center);
            }

            // First upper platform - RightFloat
            if let Some(mut right_float) = Platform::new(
                PlatformType::RightFloat,  // Higher platform is RightFloat
                Vec2::new(
                    720.0,  // Moved left 25px (from 745)
                    screen_height() - 855.0  // Moved up 10px (from 845)
                )
            ).await {
                // Add dying tree decoration
                let _ = right_float.add_decoration(DecorationType::Tree(TreeType::Dying), Some(0.4)).await;
                
                // Add collision boxes for floating platform
                for (pos, end) in right_float.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(right_float);
            }

            // Second platform - regular Floating type
            if let Some(float) = Platform::new(
                PlatformType::Floating,  // Platform near CenterSpire is Floating
                Vec2::new(
                    990.0,  // Right position
                    screen_height() - 690.0  // Lower position
                )
            ).await {
                // Add collision boxes for floating platform
                for (pos, end) in float.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(float);
            }

            // CenterSpire - adjusted right and down
            if let Some(mut spire) = Platform::new(
                PlatformType::CenterSpire,
                Vec2::new(
                    1315.0,  // Moved right 15px (from 1300)
                    screen_height() - 462.0  // Moved down 15px (was 477)
                )
            ).await {
                // Add Joshua tree decoration on the right third
                let _ = spire.add_decoration(DecorationType::Tree(TreeType::Joshua), Some(0.3)).await;
                
                // Add collision boxes for spire platform
                for (pos, end) in spire.get_collision_box() {
                    let size = end - pos;
                    world.add_platform(pos, size);
                }
                self.platforms.push(spire);
            }
        }
    }

    pub fn draw(&mut self) {
        for platform in &mut self.platforms {
            platform.draw();
        }
    }

    pub fn get_upper_right_platform_position(&self) -> Option<Vec2> {
        // Find the RightLedge platform
        for platform in &self.platforms {
            if platform.platform_type == PlatformType::RightLedge {
                // Calculate spawn position:
                let spawn_x = platform.position.x + (platform.size.x * 0.67);  // Right third
                let spawn_y = platform.position.y + 12.0;  // Top of walkable
                return Some(Vec2::new(spawn_x - 36.0, spawn_y - 72.0));  // Return box position
            }
        }
        None
    }
}
