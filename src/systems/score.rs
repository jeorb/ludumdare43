use amethyst::{
    core::Transform,
    ecs::prelude::{
        //Entities,
        Join, Read, ReadExpect, System, Write, WriteStorage},
    ui::UiText,
};

use crate::{
    states::Mob,
    //states::Player,
    states::ScoreBoard, states::ScoreText, config::LevelConfig};

pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        //Entities<'s>,
        //WriteStorage<'s, Player>,
        WriteStorage<'s, Mob>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, LevelConfig>,
    );

    fn run(&mut self, (
        //entities,
        //mut players,
        mut mobs,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
        level_config
    ): Self::SystemData) {
        for (mob, transform) in (&mut mobs, &mut locals).join() {
            let mob_x = transform.translation[0];

            let did_hit = if mob_x <= 0.0 || mob_x >= level_config.width {
                let points = if mob.touched { 1 } else {-1 };
                scores.score = (scores.score + points).min(999);

                // Scale difficulty
                if points > 0 || (mob.velocity[0] > 20.0 || mob.velocity[0] < -20.0) {
                    if mob.velocity[0] > 0.0 {
                        mob.velocity[0] = mob.velocity[0] + (points as f32 * 10.0);
                    } else {
                        mob.velocity[0] = mob.velocity[0] - (points as f32 * 10.0);
                    }
                    if mob.velocity[1] > 0.0 {
                        mob.velocity[1] = mob.velocity[1] + (points as f32 * 5.0);
                    } else {
                        mob.velocity[1] = mob.velocity[1] - (points as f32 * 5.0);
                    }
                }

                if let Some(text) = ui_text.get_mut(score_text.score) {
                    text.text = scores.score.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                mob.touched = false;
                mob.velocity[0] = -mob.velocity[0];
                transform.translation[0] = level_config.width / 2.0;
            }
        }

        // TODO: Update player sprite
        /*
        if scores.score < 0 {
            for (player, entity) in (&mut players, &*entities).join() {
                entity.SOMETHING>;
            }
        }
        */
        

    }
}

