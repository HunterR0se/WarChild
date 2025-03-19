use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum AttackPattern {
    Basic,   // Uses Attack_1
    Cyclic,  // Cycles through all Attack_X
    Special, // Uses Special attack
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct EnemyAbilities {
    pub has_special: bool,  // Has Special.png + ability
    pub has_dot: bool,      // Has Poison.png + effect
    pub has_jump: bool,     // Has Jump.png + ability
    pub has_shield: bool,   // Has Shield.png + ability
    pub has_shoot: bool,    // Has Shoot.png + ability
    pub has_arrow: bool,    // Has Arrow.png projectile
    pub has_charge: bool,   // Has Charge.png
    pub pattern: AttackPattern,  // How it uses attacks
    pub attack_frames: u32, // Number of frames in attack animation
}

impl Default for EnemyAbilities {
    fn default() -> Self {
        Self {
            has_special: false,
            has_dot: false,
            has_jump: false,
            has_shield: false,
            has_shoot: false,
            has_arrow: false,
            has_charge: false,
            pattern: AttackPattern::Basic,
            attack_frames: 8,
        }
    }
}

#[derive(Clone)]
pub struct EnemyVariant {
    pub base_hp: u32,
    pub abilities: EnemyAbilities,
    pub asset_path: &'static str,
}

#[derive(Clone)]
pub enum EnemyType {
    Man(ManVariant),
    Ghost(GhostVariant),
    Skeleton(SkeletonVariant),
    Werewolf(WerewolfVariant),
    Witch(WitchVariant),
    Demon(DemonVariant),
    Goblin(GoblinVariant),
    Hellhound(HellhoundVariant),
    Dwarf(DwarfVariant),
    Golem(GolemVariant),
    Gorgon(GorgonVariant),
    Minotaur(MinotaurVariant),
    Mutant(MutantVariant),
    Orc(OrcVariant),
    Priest(PriestVariant),
    Pyromancer(PyromancerVariant),
    Samurai(SamuraiVariant),
    Tengu(TenguVariant),
    Zombie(ZombieVariant),
}

// Man - Basic melee fighter with jump
#[derive(Clone)]
pub enum ManVariant {
    Warrior,     // 8 HP
    #[allow(dead_code)]
    Elite,       // 28 HP
    Master,      // 60 HP
}

impl ManVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            ManVariant::Warrior => 8,
            ManVariant::Elite => 29,
            ManVariant::Master => 60,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                ManVariant::Warrior => "assets/Enemies/Man_1",
                ManVariant::Elite => "assets/Enemies/Man_2",
                ManVariant::Master => "assets/Enemies/Man_3",
            },
        }
    }
}

// Ghost - Spirit with special abilities and ranged attack
#[derive(Clone)]
pub enum GhostVariant {
    Basic,      // 10 HP
    Haunted,    // 35 HP
    Wraith,     // 70 HP
    Specter,    // 95 HP
}

impl GhostVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            GhostVariant::Basic => 19,
            GhostVariant::Haunted => 36,
            GhostVariant::Wraith => 71,
            GhostVariant::Specter => 97,
        };

        let (has_arrow, has_shoot) = match self {
            GhostVariant::Basic => (true, true),
            _ => (false, false),
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                has_jump: true,
                has_arrow,
                has_shoot,
                pattern: AttackPattern::Special,
                attack_frames: match self {
                    GhostVariant::Basic => 8,
                    GhostVariant::Haunted | GhostVariant::Wraith => 12,
                    GhostVariant::Specter => 16,
                },
                ..Default::default()
            },
            asset_path: match self {
                GhostVariant::Basic => "assets/Enemies/Ghost_1",
                GhostVariant::Haunted => "assets/Enemies/Ghost_2",
                GhostVariant::Wraith => "assets/Enemies/Ghost_3",
                GhostVariant::Specter => "assets/Enemies/Ghost_4",
            },
        }
    }
}

// Skeleton - Shield/ranged specialist
#[derive(Clone)]
pub enum SkeletonVariant {
    Warrior,     // 12 HP - Shield
    Archer,      // 42 HP - Ranged
    Elite,       // 80 HP - Shield
}

impl SkeletonVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            SkeletonVariant::Warrior => 12,
            SkeletonVariant::Archer => 41,
            SkeletonVariant::Elite => 78,
        };

        let abilities = match self {
            SkeletonVariant::Warrior | SkeletonVariant::Elite => EnemyAbilities {
                has_shield: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 16,
                ..Default::default()
            },
            SkeletonVariant::Archer => EnemyAbilities {
                has_shoot: true,
                has_arrow: true,
                has_shield: true,
                pattern: AttackPattern::Special,
                attack_frames: 12,
                ..Default::default()
            },
        };

        EnemyVariant {
            base_hp,
            abilities,
            asset_path: match self {
                SkeletonVariant::Warrior => "assets/Enemies/Skeleton_1",
                SkeletonVariant::Archer => "assets/Enemies/Skeleton_2",
                SkeletonVariant::Elite => "assets/Enemies/Skeleton_3",
            },
        }
    }
}

// Werewolf - Fast melee fighter with jump
#[derive(Clone)]
pub enum WerewolfVariant {
    Basic,      // 15 HP
    Alpha,      // 38 HP
    Elder,      // 75 HP
}

impl WerewolfVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            WerewolfVariant::Basic => 16,
            WerewolfVariant::Alpha => 37,
            WerewolfVariant::Elder => 73,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 16, // Has 4 attack animations
                ..Default::default()
            },
            asset_path: match self {
                WerewolfVariant::Basic => "assets/Enemies/Werewolf_1",
                WerewolfVariant::Alpha => "assets/Enemies/Werewolf_2",
                WerewolfVariant::Elder => "assets/Enemies/Werewolf_3",
            },
        }
    }
}

// Witch - Magic user with DoT and special
#[derive(Clone)]
pub enum WitchVariant {
    Apprentice,   // 18 HP
    Sorceress,    // 48 HP
    Archmage,     // 85 HP
}

impl WitchVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            WitchVariant::Apprentice => 18,
            WitchVariant::Sorceress => 48,
            WitchVariant::Archmage => 85,
        };

        let (has_arrow, has_shoot) = match self {
            WitchVariant::Archmage => (true, true),
            _ => (false, false),
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                has_jump: true,
                has_arrow,
                has_shoot,
                pattern: AttackPattern::Special,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                WitchVariant::Apprentice => "assets/Enemies/Witch_1",
                WitchVariant::Sorceress => "assets/Enemies/Witch_2",
                WitchVariant::Archmage => "assets/Enemies/Witch_3",
            },
        }
    }
}

// Demon - Progressive power levels
#[derive(Clone)]
pub enum DemonVariant {
    Lesser,     // 20 HP
    Common,     // 35 HP
    Greater,    // 52 HP
    Elite,      // 70 HP
    Master,     // 80 HP
    Lord,       // 90 HP
    Overlord,   // 98 HP
}

impl DemonVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            DemonVariant::Lesser => 20,
            DemonVariant::Common => 34,
            DemonVariant::Greater => 52,
            DemonVariant::Elite => 69,
            DemonVariant::Master => 76,
            DemonVariant::Lord => 90,
            DemonVariant::Overlord => 100,
        };

        let (has_charge, has_arrow, has_shoot) = match self {
            DemonVariant::Greater | DemonVariant::Elite => (true, false, false),
            DemonVariant::Overlord => (false, true, true),
            _ => (false, false, false),
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_charge,
                has_arrow,
                has_shoot,
                pattern: AttackPattern::Basic,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                DemonVariant::Lesser => "assets/Enemies/Demon_1",
                DemonVariant::Common => "assets/Enemies/Demon_2",
                DemonVariant::Greater => "assets/Enemies/Demon_3",
                DemonVariant::Elite => "assets/Enemies/Demon_4",
                DemonVariant::Master => "assets/Enemies/Demon_5",
                DemonVariant::Lord => "assets/Enemies/Demon_6",
                DemonVariant::Overlord => "assets/Enemies/Demon_7",
            },
        }
    }
}

// Goblin - Agile fighter with jump
#[derive(Clone)]
pub enum GoblinVariant {
    Scout,      // 25 HP
    Warrior,    // 56 HP
    Champion,   // 95 HP
}

impl GoblinVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            GoblinVariant::Scout => 26,
            GoblinVariant::Warrior => 56,
            GoblinVariant::Champion => 95,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                GoblinVariant::Scout => "assets/Enemies/Goblin_1",
                GoblinVariant::Warrior => "assets/Enemies/Goblin_2",
                GoblinVariant::Champion => "assets/Enemies/Goblin_3",
            },
        }
    }
}

// Hellhound - Fast attacker with jump
#[derive(Clone)]
pub enum HellhoundVariant {
    Pup,        // 32 HP
    Hunter,     // 65 HP
    Alpha,      // 98 HP
}

impl HellhoundVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            HellhoundVariant::Pup => 32,
            HellhoundVariant::Hunter => 65,
            HellhoundVariant::Alpha => 101,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                HellhoundVariant::Pup => "assets/Enemies/Hellhound_1",
                HellhoundVariant::Hunter => "assets/Enemies/Hellhound_2",
                HellhoundVariant::Alpha => "assets/Enemies/Hellhound_3",
            },
        }
    }
}

// Dwarf - Special and poison with jump
#[derive(Clone)]
pub enum DwarfVariant {
    Warrior,    // 22 HP
    Berserker,  // 45 HP
    Champion,   // 88 HP
}

impl DwarfVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            DwarfVariant::Warrior => 22,
            DwarfVariant::Berserker => 46,
            DwarfVariant::Champion => 88,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                has_jump: true,
                pattern: AttackPattern::Special,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                DwarfVariant::Warrior => "assets/Enemies/Dwarf_1",
                DwarfVariant::Berserker => "assets/Enemies/Dwarf_2",
                DwarfVariant::Champion => "assets/Enemies/Dwarf_3",
            },
        }
    }
}

// Golem - Heavy fighter with jump
#[derive(Clone)]
pub enum GolemVariant {
    Stone,      // 40 HP
    Iron,       // 70 HP
    Crystal,    // 96 HP
}

impl GolemVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            GolemVariant::Stone => 40,
            GolemVariant::Iron => 70,
            GolemVariant::Crystal => 98,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 16, // Has 4 attack animations
                ..Default::default()
            },
            asset_path: match self {
                GolemVariant::Stone => "assets/Enemies/Golem_1",
                GolemVariant::Iron => "assets/Enemies/Golem_2",
                GolemVariant::Crystal => "assets/Enemies/Golem_3",
            },
        }
    }
}

// Gorgon - Special and poison abilities
#[derive(Clone)]
pub enum GorgonVariant {
    Lesser,     // 30 HP
    Greater,    // 58 HP
    Queen,      // 92 HP
}

impl GorgonVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            GorgonVariant::Lesser => 30,
            GorgonVariant::Greater => 58,
            GorgonVariant::Queen => 92,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                pattern: AttackPattern::Special,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                GorgonVariant::Lesser => "assets/Enemies/Gorgon_1",
                GorgonVariant::Greater => "assets/Enemies/Gorgon_2",
                GorgonVariant::Queen => "assets/Enemies/Gorgon_3",
            },
        }
    }
}

// Minotaur - Heavy hitter
#[derive(Clone)]
pub enum MinotaurVariant {
    Young,      // 50 HP
    Elder,      // 94 HP
}

impl MinotaurVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            MinotaurVariant::Young => 50,
            MinotaurVariant::Elder => 94,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                pattern: AttackPattern::Basic,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                MinotaurVariant::Young => "assets/Enemies/Minotaur_1",
                MinotaurVariant::Elder => "assets/Enemies/Minotaur_2",
            },
        }
    }
}

// Mutant - Special and poison with jump
#[derive(Clone)]
pub enum MutantVariant {
    Feral,      // 35 HP
    Evolved,    // 62 HP
    Perfect,    // 93 HP
}

impl MutantVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            MutantVariant::Feral => 35,
            MutantVariant::Evolved => 62,
            MutantVariant::Perfect => 93,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                has_jump: true,
                pattern: AttackPattern::Special,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                MutantVariant::Feral => "assets/Enemies/Mutant_1",
                MutantVariant::Evolved => "assets/Enemies/Mutant_2",
                MutantVariant::Perfect => "assets/Enemies/Mutant_3",
            },
        }
    }
}

// Orc - Strong fighter with jump
#[derive(Clone)]
pub enum OrcVariant {
    Grunt,      // 28 HP
    Warrior,    // 55 HP
    Warlord,    // 89 HP
}

impl OrcVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            OrcVariant::Grunt => 27,
            OrcVariant::Warrior => 55,
            OrcVariant::Warlord => 89,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 16, // Has 4 attack animations
                ..Default::default()
            },
            asset_path: match self {
                OrcVariant::Grunt => "assets/Enemies/Orc_1",
                OrcVariant::Warrior => "assets/Enemies/Orc_2",
                OrcVariant::Warlord => "assets/Enemies/Orc_3",
            },
        }
    }
}

// Priest - Magic user with special and poison
#[derive(Clone)]
pub enum PriestVariant {
    Acolyte,    // 25 HP
    Cleric,     // 54 HP
    Bishop,     // 87 HP
}

impl PriestVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            PriestVariant::Acolyte => 24,
            PriestVariant::Cleric => 54,
            PriestVariant::Bishop => 87,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_special: true,
                has_dot: true,
                pattern: AttackPattern::Special,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                PriestVariant::Acolyte => "assets/Enemies/Priests_1",
                PriestVariant::Cleric => "assets/Enemies/Priests_2",
                PriestVariant::Bishop => "assets/Enemies/Priests_3",
            },
        }
    }
}

// Pyromancer - Fire mage with jump
#[derive(Clone)]
pub enum PyromancerVariant {
    Novice,     // 33 HP
    Adept,      // 61 HP
    Master,     // 91 HP
}

impl PyromancerVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            PyromancerVariant::Novice => 33,
            PyromancerVariant::Adept => 61,
            PyromancerVariant::Master => 91,
        };

        let has_charge = matches!(self, PyromancerVariant::Novice);

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                has_charge,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                PyromancerVariant::Novice => "assets/Enemies/Pyromancer_1",
                PyromancerVariant::Adept => "assets/Enemies/Pyromancer_2",
                PyromancerVariant::Master => "assets/Enemies/Pyromancer_3",
            },
        }
    }
}

// Samurai - Skilled fighter with shield/ranged variants
#[derive(Clone)]
pub enum SamuraiVariant {
    Warrior,    // 42 HP - Shield
    Archer,     // 68 HP - Ranged
    Master,     // 97 HP - Shield
}

impl SamuraiVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            SamuraiVariant::Warrior => 43,
            SamuraiVariant::Archer => 68,
            SamuraiVariant::Master => 99,
        };

        let abilities = match self {
            SamuraiVariant::Warrior | SamuraiVariant::Master => EnemyAbilities {
                has_shield: true,
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            SamuraiVariant::Archer => EnemyAbilities {
                has_shoot: true,
                has_arrow: true,
                has_jump: true,
                pattern: AttackPattern::Special,
                attack_frames: 12,
                ..Default::default()
            },
        };

        EnemyVariant {
            base_hp,
            abilities,
            asset_path: match self {
                SamuraiVariant::Warrior => "assets/Enemies/Samurai_1",
                SamuraiVariant::Archer => "assets/Enemies/Samurai_2",
                SamuraiVariant::Master => "assets/Enemies/Samurai_3",
            },
        }
    }
}

// Tengu - Swift fighter with jump
#[derive(Clone)]
pub enum TenguVariant {
    Scout,      // 38 HP
    Warrior,    // 72 HP
}

impl TenguVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            TenguVariant::Scout => 39,
            TenguVariant::Warrior => 72,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                has_jump: true,
                pattern: AttackPattern::Cyclic,
                attack_frames: 12, // Has 3 attack animations
                ..Default::default()
            },
            asset_path: match self {
                TenguVariant::Scout => "assets/Enemies/Tengu_1",
                TenguVariant::Warrior => "assets/Enemies/Tengu_2",
            },
        }
    }
}

// Zombie - Basic undead
#[derive(Clone)]
pub enum ZombieVariant {
    Shambler,    // 15 HP
    Walker,      // 45 HP
    Runner,      // 75 HP
    Brute,       // 95 HP
}

impl ZombieVariant {
    pub fn get_variant(&self) -> EnemyVariant {
        let base_hp = match self {
            ZombieVariant::Shambler => 14,
            ZombieVariant::Walker => 44,
            ZombieVariant::Runner => 74,
            ZombieVariant::Brute => 96,
        };

        EnemyVariant {
            base_hp,
            abilities: EnemyAbilities {
                pattern: AttackPattern::Basic,
                attack_frames: 8,
                ..Default::default()
            },
            asset_path: match self {
                ZombieVariant::Shambler => "assets/Enemies/Zombie_1",
                ZombieVariant::Walker => "assets/Enemies/Zombie_2",
                ZombieVariant::Runner => "assets/Enemies/Zombie_3",
                ZombieVariant::Brute => "assets/Enemies/Zombie_4",
            },
        }
    }
}

impl fmt::Display for EnemyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnemyType::Man(_) => write!(f, "Man"),
            EnemyType::Ghost(_) => write!(f, "Ghost"),
            EnemyType::Skeleton(_) => write!(f, "Skeleton"),
            EnemyType::Werewolf(_) => write!(f, "Werewolf"),
            EnemyType::Witch(_) => write!(f, "Witch"),
            EnemyType::Demon(_) => write!(f, "Demon"),
            EnemyType::Goblin(_) => write!(f, "Goblin"),
            EnemyType::Hellhound(_) => write!(f, "Hellhound"),
            EnemyType::Dwarf(_) => write!(f, "Dwarf"),
            EnemyType::Golem(_) => write!(f, "Golem"),
            EnemyType::Gorgon(_) => write!(f, "Gorgon"),
            EnemyType::Minotaur(_) => write!(f, "Minotaur"),
            EnemyType::Mutant(_) => write!(f, "Mutant"),
            EnemyType::Orc(_) => write!(f, "Orc"),
            EnemyType::Priest(_) => write!(f, "Priest"),
            EnemyType::Pyromancer(_) => write!(f, "Pyromancer"),
            EnemyType::Samurai(_) => write!(f, "Samurai"),
            EnemyType::Tengu(_) => write!(f, "Tengu"),
            EnemyType::Zombie(_) => write!(f, "Zombie"),
        }
    }
}