use amethyst::core::Transform;
use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use states::{Player};
use crate::config::{LevelConfig, PlayerConfig};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        Read<'s, LevelConfig>,
        Read<'s, PlayerConfig>,
    );

    fn run(&mut self, (mut transforms, players, input, time, level_config, player_config): Self::SystemData) {
        for (_player, transform) in (&players, &mut transforms).join() {
            let movement_x = input.axis_value("horizontal");
            let movement_y = input.axis_value("vertical");
            if let Some(mv_amount) = movement_y {
                let scaled_amount = player_config.speed * mv_amount as f32 * time.delta_seconds();
                transform.translation[1] = (transform.translation[1] + scaled_amount)
                    .min(level_config.height - player_config.height * 0.5)
                    .max(player_config.height * 0.5);
            }
            if let Some(mv_amount) = movement_x {
                let scaled_amount = player_config.speed * mv_amount as f32 * time.delta_seconds();
                transform.translation[0] = (transform.translation[0] + scaled_amount)
                    .min(level_config.width - player_config.width * 0.5)
                    .max(player_config.width * 0.5);
            }
        }
    }
}





