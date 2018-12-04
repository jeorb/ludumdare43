use amethyst::{
    core::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::states::{Mob, Player};
use crate::config::LevelConfig;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Mob>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, LevelConfig>,
    );

    fn run(&mut self, (mut mobs, players, transforms, level_config): Self::SystemData) {
        for (mob, transform) in (&mut mobs, &transforms).join() {
            let mob_x = transform.translation[0];
            let mob_y = transform.translation[1];

            if mob_y >= level_config.height - mob.width * 0.5 && mob.velocity[1] > 0.0 {
                mob.velocity[1] = -mob.velocity[1];
            } else if mob_y <= mob.height*0.5 && mob.velocity[1] < 0.0 {
                mob.velocity[1] = -mob.velocity[1];
            }

            for (player, player_transform) in (&players, &transforms).join() {
                let player_x = player_transform.translation[0] - player.width * 0.5;
                let player_y = player_transform.translation[1] - player.height * 0.5;

                if point_in_rect(
                    mob_x,
                    mob_y,
                    player_x - mob.width * 0.5,
                    player_y - mob.height * 0.5,
                    player_x + player.width + mob.width * 0.5,
                    player_y + player.height + mob.height * 0.5,
                ) {
                    if mob_x > player_x && mob.velocity[0] < 0.0 {
                        mob.velocity[0] = -mob.velocity[0];
                        mob.touched = true;
                    } else if mob_x < player_x && mob.velocity[0] > 0.0 {
                        mob.velocity[0] = -mob.velocity[0];
                        mob.touched = true;
                    }
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}