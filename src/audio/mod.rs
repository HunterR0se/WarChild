use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use std::collections::HashMap;
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Make the types module public
pub mod types;
use types::SoundCategory;

// Make the triggers module public
pub mod triggers;
pub use triggers::SoundTriggers;

// Track the loading state of audio resources
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingState {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

// Structure to track loading requests
struct LoadRequest {
    category: SoundCategory,
    name: String,
    path: String,
    is_variation: bool,
    base_name: Option<String>,
    #[allow(dead_code)]
    variation_index: Option<u8>,
}

// Struct to manage a pool of similar sound variations
#[derive(Debug, Clone)]
struct SoundPool {
    variations: Vec<Sound>,
    #[allow(dead_code)]
    base_name: String,
}

impl SoundPool {
    fn new(base_name: &str) -> Self {
        Self {
            variations: Vec::new(),
            base_name: base_name.to_string(),
        }
    }
    
    fn add_variation(&mut self, sound: Sound) {
        self.variations.push(sound);
    }
    
    fn get_random_sound(&self) -> Option<&Sound> {
        if self.variations.is_empty() {
            None
        } else {
            self.variations.choose(&mut thread_rng())
        }
    }
    
    #[allow(dead_code)]
    fn variation_count(&self) -> usize {
        self.variations.len()
    }
}

pub struct AudioSystem {
    // Organize individual sounds by category and name
    sounds: HashMap<SoundCategory, HashMap<String, Sound>>,
    // Organize sound pools (variations) by category and base name
    sound_pools: HashMap<SoundCategory, HashMap<String, SoundPool>>,
    // Per-category volume controls
    category_volumes: HashMap<SoundCategory, f32>,
    // Master volume control
    master_volume: f32,
    // Per-category mute controls
    category_muted: HashMap<SoundCategory, bool>,
    // Master mute control
    master_muted: bool,
    // Track loading state for each sound
    loading_states: HashMap<String, LoadingState>,
    // Queue of pending load requests
    load_queue: Vec<LoadRequest>,
    // Total sounds to load
    total_to_load: usize,
    // Successfully loaded sounds
    total_loaded: usize,
    // Loading failed
    total_failed: usize,
}

impl AudioSystem {
    pub fn new() -> Self {
        // Create empty HashMaps for all categories
        let mut sounds = HashMap::new();
        let mut sound_pools = HashMap::new();
        let mut category_volumes = HashMap::new();
        let mut category_muted = HashMap::new();
        
        // Initialize all sound categories
        for category in [
            SoundCategory::Character,
            SoundCategory::Combat,
            SoundCategory::Effect,
            SoundCategory::Environment,
            SoundCategory::Movement,
            SoundCategory::Item,
            SoundCategory::UI,
        ] {
            sounds.insert(category, HashMap::new());
            sound_pools.insert(category, HashMap::new());
            category_volumes.insert(category, 1.0); // Default full volume
            category_muted.insert(category, false); // Default unmuted
        }

        Self {
            sounds,
            sound_pools,
            category_volumes,
            master_volume: 1.0,
            category_muted,
            master_muted: false,
            loading_states: HashMap::new(),
            load_queue: Vec::new(),
            total_to_load: 0,
            total_loaded: 0,
            total_failed: 0,
        }
    }
    
    // Add a sound to the loading queue
    pub fn queue_sound(&mut self, category: SoundCategory, name: &str, path: &str) {
        let path_key = path.to_string();
        
        // Don't queue if already loaded or in queue
        if self.loading_states.contains_key(&path_key) {
            return;
        }
        
        // Mark as not loaded yet
        self.loading_states.insert(path_key.clone(), LoadingState::NotLoaded);
        
        // Add to queue
        self.load_queue.push(LoadRequest {
            category,
            name: name.to_string(),
            path: path.to_string(),
            is_variation: false,
            base_name: None,
            variation_index: None,
        });
        
        // Update total
        self.total_to_load += 1;
    }
    
    // Add a sound variation to the loading queue
    pub fn queue_sound_variation(&mut self, category: SoundCategory, base_name: &str, variation: u8, path: &str) {
        let path_key = path.to_string();
        
        // Don't queue if already loaded or in queue
        if self.loading_states.contains_key(&path_key) {
            return;
        }
        
        // Mark as not loaded yet
        self.loading_states.insert(path_key.clone(), LoadingState::NotLoaded);
        
        // Add to queue
        self.load_queue.push(LoadRequest {
            category,
            name: format!("{}_{:02}", base_name, variation),
            path: path.to_string(),
            is_variation: true,
            base_name: Some(base_name.to_string()),
            variation_index: Some(variation),
        });
        
        // Update total
        self.total_to_load += 1;
    }
    
    // Queue a batch of sound variations
    pub fn queue_variations(&mut self, category: SoundCategory, base_name: &str, count: u8) {
        let category_folder = Self::get_category_folder_name(category);
        
        for i in 1..=count {
            let variation_path = format!("assets/audio/{}/{}_{:02}.wav", category_folder, base_name, i);
            self.queue_sound_variation(category, base_name, i, &variation_path);
        }
    }
    
    // Process a batch of queued sound loads asynchronously
    pub async fn process_queue(&mut self, batch_size: usize) -> usize {
        let batch_size = batch_size.min(self.load_queue.len());
        if batch_size == 0 {
            return 0;
        }
        
        // Take a batch of requests from the queue
        let requests: Vec<LoadRequest> = self.load_queue.drain(0..batch_size).collect();
        
        // Update loading states for all items in this batch
        for request in &requests {
            self.loading_states.insert(request.path.clone(), LoadingState::Loading);
        }
        
        // Process each sound sequentially (avoids borrowing issues)
        let mut processed_count = 0;
        for request in &requests {
            let path = &request.path;
            
            // Load the sound
            if let Ok(sound) = load_sound(path).await {
                // Sound loaded successfully
                processed_count += 1;
                self.total_loaded += 1;
                self.loading_states.insert(request.path.clone(), LoadingState::Loaded);
                
                if request.is_variation {
                    // Add to sound pool
                    if let Some(base_name) = &request.base_name {
                        // Make sure pool exists
                        if let Some(category_pools) = self.sound_pools.get_mut(&request.category) {
                            if !category_pools.contains_key(base_name) {
                                category_pools.insert(base_name.clone(), SoundPool::new(base_name));
                            }
                            
                            // Add to pool
                            if let Some(pool) = category_pools.get_mut(base_name) {
                                pool.add_variation(sound);
                            }
                        }
                    }
                } else {
                    // Add as regular sound
                    if let Some(category_sounds) = self.sounds.get_mut(&request.category) {
                        category_sounds.insert(request.name.clone(), sound);
                    }
                }
            } else {
                // Sound failed to load
                self.total_failed += 1;
                self.loading_states.insert(request.path.clone(), LoadingState::Failed);
            }
        }
        
        processed_count
    }
    
    // Process the entire queue
    pub async fn process_all_queued(&mut self) -> usize {
        let mut total_processed = 0;
        
        while !self.load_queue.is_empty() {
            let processed = self.process_queue(50).await;
            total_processed += processed;
        }
        
        total_processed
    }
    
    // Get loading progress information
    pub fn get_loading_progress(&self) -> (usize, usize, usize) {
        (self.total_loaded, self.total_failed, self.total_to_load)
    }
    
    // Calculate loading percentage (0-100)
    pub fn loading_percentage(&self) -> f32 {
        if self.total_to_load == 0 {
            return 100.0; // Nothing to load means we're done
        }
        
        let completed = self.total_loaded + self.total_failed;
        (completed as f32 / self.total_to_load as f32) * 100.0
    }
    
    // Check if all queued sounds have been processed
    pub fn is_loading_complete(&self) -> bool {
        self.load_queue.is_empty() && (self.total_loaded + self.total_failed >= self.total_to_load)
    }
    
    // Reset loading progress and queue
    pub fn reset_loading(&mut self) {
        self.load_queue.clear();
        self.loading_states.clear();
        self.total_to_load = 0;
        self.total_loaded = 0;
        self.total_failed = 0;
    }
    
    // Queue all common sound categories
    pub fn queue_common_sounds(&mut self) {
        // UI sounds
        for pattern in &["click", "hover", "select", "back", "pause", "unpause"] {
            let category_folder = Self::get_category_folder_name(SoundCategory::UI);
            let path = format!("assets/audio/{}/{}.wav", category_folder, pattern);
            self.queue_sound(SoundCategory::UI, pattern, &path);
        }
        
        // Combat sounds
        for pattern in &["sword_1", "sword_2", "hit_1", "hit_2", "block", "arrow"] {
            let category_folder = Self::get_category_folder_name(SoundCategory::Combat);
            let path = format!("assets/audio/{}/{}.wav", category_folder, pattern);
            self.queue_sound(SoundCategory::Combat, pattern, &path);
        }
        
        // Movement sounds
        for pattern in &["jump", "land", "walk_1", "walk_2", "run_1", "run_2", "roll"] {
            let category_folder = Self::get_category_folder_name(SoundCategory::Movement);
            let path = format!("assets/audio/{}/{}.wav", category_folder, pattern);
            self.queue_sound(SoundCategory::Movement, pattern, &path);
        }
        
        // Item sounds
        for pattern in &["gem_pickup", "artifact_pickup", "powerup"] {
            let category_folder = Self::get_category_folder_name(SoundCategory::Item);
            let path = format!("assets/audio/{}/{}.wav", category_folder, pattern);
            self.queue_sound(SoundCategory::Item, pattern, &path);
        }
    }
    
    // Queue all common variation sets
    pub fn queue_common_variations(&mut self) {
        // Footstep variations
        self.queue_variations(SoundCategory::Movement, "footstep", 8);
        
        // Combat hit variations
        self.queue_variations(SoundCategory::Combat, "hit", 6);
        
        // Weapon swing variations
        self.queue_variations(SoundCategory::Combat, "swing", 4);
        
        // Enemy death variations
        self.queue_variations(SoundCategory::Character, "death", 10);
    }
    
    // Get loading state of a sound
    pub fn get_sound_state(&self, path: &str) -> LoadingState {
        *self.loading_states.get(path).unwrap_or(&LoadingState::NotLoaded)
    }
    
    // Check if a specific sound is loaded
    pub fn is_sound_loaded(&self, path: &str) -> bool {
        self.get_sound_state(path) == LoadingState::Loaded
    }
    
    // Check if a sound variation set has all items loaded
    pub fn is_variation_set_loaded(&self, category: SoundCategory, base_name: &str) -> bool {
        if let Some(pools) = self.sound_pools.get(&category) {
            if let Some(pool) = pools.get(base_name) {
                return pool.variations.len() > 0;
            }
        }
        false
    }
    
    // Queue all audio files
    pub fn queue_all_audio(&mut self) {
        self.queue_common_sounds();
        self.queue_common_variations();
    }
    
    // Update load progress each frame (call from game loop)
    pub async fn update_loading(&mut self, max_batch_size: usize) -> bool {
        if self.load_queue.is_empty() {
            return true; // Loading complete
        }
        
        let processed = self.process_queue(max_batch_size).await;
        processed > 0
    }

    pub async fn load_sound(&mut self, category: SoundCategory, name: &str, path: &str) {
        if let Ok(sound) = load_sound(path).await {
            if let Some(category_sounds) = self.sounds.get_mut(&category) {
                category_sounds.insert(name.to_string(), sound);
            }
        }
    }
    
    // Get the category folder name as a string
    fn get_category_folder_name(category: SoundCategory) -> &'static str {
        match category {
            SoundCategory::Character => "characters",
            SoundCategory::Combat => "combat",
            SoundCategory::Effect => "effects",
            SoundCategory::Environment => "environment",
            SoundCategory::Movement => "movement",
            SoundCategory::Item => "items",
            SoundCategory::UI => "ui",
        }
    }
    
    // Extract sound name from file path (without extension)
    fn extract_sound_name(file_path: &str) -> String {
        Path::new(file_path)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
    
    // Load a sound variation and add it to the appropriate pool
    pub async fn load_sound_variation(&mut self, category: SoundCategory, base_name: &str, _variation: u8, path: &str) {
        if let Ok(sound) = load_sound(path).await {
            // Get the category's sound pools
            if let Some(category_pools) = self.sound_pools.get_mut(&category) {
                // Check if a pool already exists for this base name
                if !category_pools.contains_key(base_name) {
                    category_pools.insert(base_name.to_string(), SoundPool::new(base_name));
                }
                
                // Add this variation to the pool
                if let Some(pool) = category_pools.get_mut(base_name) {
                    pool.add_variation(sound);
                }
            }
        }
    }
    
    // Preload a set of numbered variations (01, 02, etc.) for a sound
    pub async fn preload_variations(&mut self, category: SoundCategory, base_name: &str, count: u8) -> usize {
        let category_folder = Self::get_category_folder_name(category);
        let mut loaded_count = 0;
        
        // Create a new sound pool for this base name if it doesn't exist
        if let Some(category_pools) = self.sound_pools.get_mut(&category) {
            if !category_pools.contains_key(base_name) {
                category_pools.insert(base_name.to_string(), SoundPool::new(base_name));
            }
        }
        
        // Load each variation (01, 02, etc.)
        for i in 1..=count {
            let variation_path = format!("assets/audio/{}/{}_{:02}.wav", category_folder, base_name, i);
            
            if let Ok(sound) = load_sound(&variation_path).await {
                // Add to the sound pool
                if let Some(category_pools) = self.sound_pools.get_mut(&category) {
                    if let Some(pool) = category_pools.get_mut(base_name) {
                        pool.add_variation(sound);
                        loaded_count += 1;
                    }
                }
            }
        }
        
        loaded_count
    }
    
    // Preload common variation sets
    pub async fn preload_common_variations(&mut self) -> usize {
        let mut total_loaded = 0;
        
        // Footstep variations
        total_loaded += self.preload_variations(SoundCategory::Movement, "footstep", 8).await;
        
        // Combat hit variations
        total_loaded += self.preload_variations(SoundCategory::Combat, "hit", 6).await;
        
        // Weapon swing variations
        total_loaded += self.preload_variations(SoundCategory::Combat, "swing", 4).await;
        
        // Enemy death variations
        total_loaded += self.preload_variations(SoundCategory::Character, "death", 10).await;
        
        total_loaded
    }
    
    // Play a random variation from a sound pool
    pub fn play_variation(&self, category: SoundCategory, base_name: &str) {
        // Check master mute first
        if self.master_muted {
            return;
        }
        
        // Check category mute
        if *self.category_muted.get(&category).unwrap_or(&false) {
            return;
        }
        
        // Get category volume, default to 1.0 if not found
        let category_volume = *self.category_volumes.get(&category).unwrap_or(&1.0);
        
        // Calculate final volume
        let final_volume = self.master_volume * category_volume;
        
        // Find and play a random sound from the pool
        if let Some(category_pools) = self.sound_pools.get(&category) {
            if let Some(pool) = category_pools.get(base_name) {
                if let Some(sound) = pool.get_random_sound() {
                    play_sound(
                        sound,
                        PlaySoundParams {
                            volume: final_volume,
                            looped: false,
                        },
                    );
                }
            }
        }
    }
    
    // Preload sounds in a category with explicit file pattern
    pub async fn preload_category(&mut self, category: SoundCategory, file_patterns: &[&str]) -> usize {
        let category_folder = Self::get_category_folder_name(category);
        let base_path = format!("assets/audio/{}", category_folder);
        
        let mut loaded_count = 0;
        
        // Load each pattern sequentially (avoids borrowing issues)
        for pattern in file_patterns {
            // Construct full path for this pattern
            let full_path = format!("{}/{}.wav", base_path, pattern);
            
            // Extract sound name from the pattern
            let sound_name = Self::extract_sound_name(pattern);
            
            // Load the sound
            if let Ok(sound) = load_sound(&full_path).await {
                if let Some(category_sounds) = self.sounds.get_mut(&category) {
                    category_sounds.insert(sound_name, sound);
                    loaded_count += 1;
                }
            }
        }
        
        loaded_count
    }
    
    // Preload common sound categories with known patterns
    pub async fn preload_common_sounds(&mut self) -> usize {
        let mut total_loaded = 0;
        
        // UI sounds
        total_loaded += self.preload_category(
            SoundCategory::UI, 
            &["click", "hover", "select", "back", "pause", "unpause"]
        ).await;
        
        // Combat sounds
        total_loaded += self.preload_category(
            SoundCategory::Combat,
            &["sword_1", "sword_2", "hit_1", "hit_2", "block", "arrow"]
        ).await;
        
        // Movement sounds
        total_loaded += self.preload_category(
            SoundCategory::Movement,
            &["jump", "land", "walk_1", "walk_2", "run_1", "run_2", "roll"]
        ).await;
        
        // Item sounds
        total_loaded += self.preload_category(
            SoundCategory::Item,
            &["gem_pickup", "artifact_pickup", "powerup"]
        ).await;
        
        total_loaded
    }

    pub fn play(&self, category: SoundCategory, name: &str) {
        // Check master mute first
        if self.master_muted {
            return;
        }
        
        // Check category mute
        if *self.category_muted.get(&category).unwrap_or(&false) {
            return;
        }
        
        // Get category volume, default to 1.0 if not found
        let category_volume = *self.category_volumes.get(&category).unwrap_or(&1.0);
        
        // Calculate final volume
        let final_volume = self.master_volume * category_volume;
        
        // Find and play the sound
        if let Some(category_sounds) = self.sounds.get(&category) {
            if let Some(sound) = category_sounds.get(name) {
                play_sound(
                    sound,
                    PlaySoundParams {
                        volume: final_volume,
                        looped: false,
                    },
                );
            }
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_category_volume(&mut self, category: SoundCategory, volume: f32) {
        self.category_volumes.insert(category, volume.clamp(0.0, 1.0));
    }

    pub fn toggle_master_mute(&mut self) {
        self.master_muted = !self.master_muted;
    }

    pub fn toggle_category_mute(&mut self, category: SoundCategory) {
        if let Some(muted) = self.category_muted.get_mut(&category) {
            *muted = !*muted;
        }
    }
}