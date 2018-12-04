use amethyst::renderer::DisplayConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for LevelConfig {
    fn default() -> Self {
        LevelConfig {
            height: 100.0,
            width: 100.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            height: 32.0,
            width: 32.0,
            speed: 100.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MobConfig {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
}

impl Default for MobConfig {
    fn default() -> Self {
        MobConfig {
            height: 32.0,
            width: 32.0,
            speed: 75.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub display: DisplayConfig,
    pub level: LevelConfig,
    pub player: PlayerConfig,
    pub mob: MobConfig,
}