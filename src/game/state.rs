use crate::effects::Projectile;
use crate::enemies::types::*;
use crate::physics::{CollisionType, World};
use macroquad::prelude::Vec2;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use warchild::objects::{Collectible, artifact::Artifact, gem::Gem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Loading,
    Menu,
    Playing,
    Paused,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

pub struct GameStateManager {
    current_state: GameState,
    pub spawn_timer: f32,
    spawn_created: bool,
    first_spawn_done: bool,
    enemy_count: usize, // Track how many enemies are alive
    pub next_spawn_delay: f32,
    last_spawn_time: f32,                   // Track time since last spawn
    spawned_types: Vec<(EnemyType, usize)>, // Track spawned types and their order
    pub projectiles: Vec<Projectile>,       // Active projectiles
    pub gems: Vec<Gem>,                     // Active gems in the game
    pub artifacts: Vec<Artifact>,           // Active artifacts in the game
    initial_gem_spawned: bool,              // Track if we've spawned the initial gem
}

impl GameStateManager {
    // Define the enemy types list once, to be used by both new() and reset_all()
    const ENEMY_TYPES: [(EnemyType, usize); 60] = [
        // Tier 1 (8-20 HP)
        (EnemyType::Man(ManVariant::Warrior), 1), // 8 HP
        (EnemyType::Skeleton(SkeletonVariant::Warrior), 2), // 12 HP
        (EnemyType::Zombie(ZombieVariant::Shambler), 3), // 15 HP
        (EnemyType::Werewolf(WerewolfVariant::Basic), 4), // 15 HP
        (EnemyType::Witch(WitchVariant::Apprentice), 5), // 18 HP
        (EnemyType::Ghost(GhostVariant::Basic), 6), // 19 HP
        (EnemyType::Demon(DemonVariant::Lesser), 7), // 20 HP
        // Tier 2 (22-45 HP)
        (EnemyType::Dwarf(DwarfVariant::Warrior), 8), // 22 HP
        (EnemyType::Priest(PriestVariant::Acolyte), 9), // 25 HP
        (EnemyType::Goblin(GoblinVariant::Scout), 10), // 25 HP
        (EnemyType::Orc(OrcVariant::Grunt), 11),      // 28 HP
        (EnemyType::Gorgon(GorgonVariant::Lesser), 12), // 30 HP
        (EnemyType::Hellhound(HellhoundVariant::Pup), 13), // 32 HP
        (EnemyType::Pyromancer(PyromancerVariant::Novice), 14), // 33 HP
        (EnemyType::Demon(DemonVariant::Common), 15), // 35 HP
        (EnemyType::Mutant(MutantVariant::Feral), 16), // 35 HP
        (EnemyType::Werewolf(WerewolfVariant::Alpha), 17), // 38 HP
        (EnemyType::Tengu(TenguVariant::Scout), 18),  // 38 HP
        (EnemyType::Ghost(GhostVariant::Haunted), 19), // 35 HP
        (EnemyType::Golem(GolemVariant::Stone), 20),  // 40 HP
        (EnemyType::Skeleton(SkeletonVariant::Archer), 21), // 42 HP
        (EnemyType::Samurai(SamuraiVariant::Warrior), 22), // 42 HP
        (EnemyType::Zombie(ZombieVariant::Walker), 23), // 45 HP
        (EnemyType::Dwarf(DwarfVariant::Berserker), 24), // 45 HP
        // Tier 3 (48-75 HP)
        (EnemyType::Witch(WitchVariant::Sorceress), 25), // 48 HP
        (EnemyType::Minotaur(MinotaurVariant::Young), 26), // 50 HP
        (EnemyType::Demon(DemonVariant::Greater), 27),   // 52 HP
        (EnemyType::Priest(PriestVariant::Cleric), 28),  // 54 HP
        (EnemyType::Orc(OrcVariant::Warrior), 29),       // 55 HP
        (EnemyType::Goblin(GoblinVariant::Warrior), 30), // 56 HP
        (EnemyType::Gorgon(GorgonVariant::Greater), 31), // 58 HP
        (EnemyType::Man(ManVariant::Master), 32),        // 60 HP
        (EnemyType::Pyromancer(PyromancerVariant::Adept), 33), // 61 HP
        (EnemyType::Mutant(MutantVariant::Evolved), 34), // 62 HP
        (EnemyType::Hellhound(HellhoundVariant::Hunter), 35), // 65 HP
        (EnemyType::Samurai(SamuraiVariant::Archer), 36), // 68 HP
        (EnemyType::Demon(DemonVariant::Elite), 37),     // 70 HP
        (EnemyType::Golem(GolemVariant::Iron), 38),      // 70 HP
        (EnemyType::Tengu(TenguVariant::Warrior), 39),   // 72 HP
        (EnemyType::Werewolf(WerewolfVariant::Elder), 40), // 75 HP
        (EnemyType::Zombie(ZombieVariant::Runner), 41),  // 75 HP
        (EnemyType::Ghost(GhostVariant::Wraith), 42),    // 75 HP
        // Tier 4 (80-100 HP)
        (EnemyType::Demon(DemonVariant::Master), 43), // 80 HP
        (EnemyType::Skeleton(SkeletonVariant::Elite), 44), // 80 HP
        (EnemyType::Witch(WitchVariant::Archmage), 45), // 85 HP
        (EnemyType::Priest(PriestVariant::Bishop), 46), // 87 HP
        (EnemyType::Dwarf(DwarfVariant::Champion), 47), // 88 HP
        (EnemyType::Orc(OrcVariant::Warlord), 48),    // 89 HP
        (EnemyType::Demon(DemonVariant::Lord), 49),   // 90 HP
        (EnemyType::Pyromancer(PyromancerVariant::Master), 50), // 91 HP
        (EnemyType::Gorgon(GorgonVariant::Queen), 51), // 92 HP
        (EnemyType::Mutant(MutantVariant::Perfect), 52), // 93 HP
        (EnemyType::Minotaur(MinotaurVariant::Elder), 53), // 94 HP
        (EnemyType::Goblin(GoblinVariant::Champion), 54), // 95 HP
        (EnemyType::Zombie(ZombieVariant::Brute), 55), // 95 HP
        (EnemyType::Ghost(GhostVariant::Specter), 56), // 95 HP
        (EnemyType::Golem(GolemVariant::Crystal), 57), // 96 HP
        (EnemyType::Samurai(SamuraiVariant::Master), 58), // 97 HP
        (EnemyType::Demon(DemonVariant::Overlord), 59), // 98 HP
        (EnemyType::Hellhound(HellhoundVariant::Alpha), 60), // 98 HP
    ];

    pub fn new() -> Self {
        Self {
            current_state: GameState::default(),
            spawn_timer: 0.0,
            spawn_created: false,
            first_spawn_done: false,
            enemy_count: 0,
            next_spawn_delay: 2.5, // Initial spawn at 2.5 seconds
            last_spawn_time: 0.0,
            spawned_types: Self::ENEMY_TYPES.to_vec(),
            projectiles: Vec::new(),
            gems: Vec::new(),
            artifacts: Vec::new(),
            initial_gem_spawned: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.current_state == GameState::Playing {
            // Track time for spawning
            if !self.first_spawn_done {
                self.spawn_timer += dt; // Used for initial spawn
            } else {
                self.last_spawn_time += dt; // Used for subsequent spawns
            }

            // Update active gems
            self.gems.retain_mut(|gem| {
                gem.update(dt);
                gem.active
            });

            // Update active artifacts
            self.artifacts.retain_mut(|artifact| {
                artifact.update(dt);
                artifact.is_active()
            });
        }
    }

    pub fn should_spawn(&mut self) -> bool {
        if self.current_state != GameState::Playing {
            return false;
        }
        if self.spawned_types.is_empty() {
            return false;
        } // No more enemies to spawn

        // First spawn check - always at 2.5 seconds
        if !self.first_spawn_done {
            return self.spawn_timer >= 2.5;
        }

        // For subsequent spawns, check if enough time has passed since last spawn
        if self.last_spawn_time >= self.next_spawn_delay {
            return true;
        }

        false
    }

    pub fn get_next_enemy_type(&mut self) -> Option<EnemyType> {
        if self.spawned_types.is_empty() {
            None
        } else {
            // Take the first enemy type (they're already in HP order)
            Some(self.spawned_types.remove(0).0)
        }
    }

    pub fn mark_spawned(&mut self) {
        if !self.first_spawn_done {
            self.first_spawn_done = true;
            // Set up next spawn delay after first spawn
            self.next_spawn_delay = thread_rng().gen_range(12.0..15.0);
        } else {
            // Set next spawn delay for subsequent spawns
            self.next_spawn_delay = thread_rng().gen_range(12.0..15.0);
        }

        self.spawn_timer = 0.0; // Reset spawn timer
        self.last_spawn_time = 0.0; // Reset time since last spawn
        self.spawn_created = true;
    }

    pub fn mark_enemy_spawned(&mut self) {
        self.enemy_count += 1;
    }

    pub fn mark_enemy_dead(&mut self) {
        self.enemy_count -= 1;
    }

    #[allow(dead_code)]
    pub fn reset_spawn(&mut self) {
        self.spawn_timer = 0.0;
        self.spawn_created = false;
    }

    pub fn reset_all(&mut self) {
        self.spawn_timer = 0.0;
        self.spawn_created = false;
        self.first_spawn_done = false;
        self.enemy_count = 0;
        self.next_spawn_delay = 2.5; // Reset to 2.5 seconds initial spawn
        self.last_spawn_time = 0.0;
        // Reset enemy types list - same as new()
        self.spawned_types = Self::ENEMY_TYPES.to_vec();
        // Clear any active projectiles
        self.projectiles.clear();
        // Clear gems and artifacts
        self.gems.clear();
        self.artifacts.clear();
        self.initial_gem_spawned = false;
    }

    // Helper function to get random platform position
    fn get_random_platform_pos(&self, world: &World) -> Option<Vec2> {
        // Get all platforms and their walkable areas
        let mut walkable_areas: Vec<(Vec2, Vec2)> = Vec::new();
        for platform in &world.platforms {
            for area in &platform.collision_areas {
                if matches!(area.area_type, CollisionType::Walkable) {
                    // Convert to world space and store - use walkable top for gems
                    let world_pos = platform.pos + area.bounds.0;
                    let world_size = area.bounds.1 - area.bounds.0;
                    walkable_areas.push((world_pos, world_size));
                }
            }
        }

        // Choose random platform area
        let mut rng = thread_rng();
        if let Some((pos, size)) = walkable_areas.choose(&mut rng) {
            // Choose random position within walkable area, adjust for gem size
            let x = rng.gen_range(pos.x..pos.x + size.x - 16.0); // 16 is gem width
            Some(Vec2::new(x, pos.y - 72.0)) // Place 72px above platform walkable top
        } else {
            None
        }
    }

    pub async fn spawn_initial_gem(&mut self, world: &World) -> bool {
        if self.initial_gem_spawned {
            return false;
        }

        // Get random platform position
        if let Some(pos) = self.get_random_platform_pos(world) {
            // Load a random gem (1-6)
            let mut rng = thread_rng();
            let gem_type = rng.gen_range(1..=6);
            let texture_path = format!("assets/Objects/Gems/{}.png", gem_type);

            if let Some(mut gem) = Gem::new(&texture_path, gem_type).await {
                gem.position = pos;
                self.gems.push(gem);
                self.initial_gem_spawned = true;
                return true;
            }
        }

        false
    }

    pub fn mark_gem_collected(&mut self, index: usize) -> Option<(u8, u8)> {
        if let Some(gem) = self.gems.get_mut(index) {
            gem.collect();
            Some(gem.get_rewards())
        } else {
            None
        }
    }

    pub fn get_active_gems(&self) -> &Vec<Gem> {
        &self.gems
    }

    pub fn get_active_artifacts(&self) -> &Vec<Artifact> {
        &self.artifacts
    }

    pub fn mark_artifact_collected(&mut self, index: usize) -> Option<(u8, u8)> {
        if let Some(artifact) = self.artifacts.get_mut(index) {
            artifact.collect();
            Some(artifact.get_rewards())
        } else {
            None
        }
    }

    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn update_projectiles(&mut self, dt: f32) {
        self.projectiles.retain_mut(|proj| {
            proj.update(dt);
            !proj.is_done()
        });
    }

    #[allow(dead_code)]
    pub fn clear_projectiles(&mut self) {
        self.projectiles.clear();
    }

    pub fn handle_enemy_death(&mut self, enemy_pos: Vec2, enemy_size: Vec2) {
        // Roll for drops
        let mut rng = thread_rng();
        let drop_roll = rng.gen_range(0.0..1.0);
        let drop_pos = Vec2::new(
            enemy_pos.x + enemy_size.x + 16.0, // 16px right of enemy
            enemy_pos.y, // Same Y as enemy's final position
        );

        if drop_roll < 0.25 { // 25% chance for gem (up from 20%)
            // Use existing gem spawn logic
            let gem_type = rng.gen_range(1..=6);
            let texture_path = format!("assets/Objects/Gems/{}.png", gem_type);

            futures::executor::block_on(async {
                if let Some(mut gem) = Gem::new(&texture_path, gem_type).await {
                    gem.position = drop_pos;
                    self.gems.push(gem);
                }
            });
        } else if drop_roll < 0.40 { // Additional 15% chance for artifact (up from 10%)
            // Load random artifact (1-20)
            let artifact_type = rng.gen_range(1..=20);
            let texture_path = format!("assets/Objects/Artifacts/{}.png", artifact_type);
            futures::executor::block_on(async {
                if let Some(mut artifact) = Artifact::new(&texture_path).await {
                    artifact.position = drop_pos;
                    self.artifacts.push(artifact);
                }
            });
        }
    }

    pub fn get_state(&self) -> GameState {
        self.current_state
    }

    pub fn set_state(&mut self, new_state: GameState) {
        if self.current_state != new_state {
            self.current_state = new_state;
            if new_state == GameState::Menu {
                self.reset_all();
            }
        }
    }

    pub fn is_state(&self, state: GameState) -> bool {
        self.current_state == state
    }
}
