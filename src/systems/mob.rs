use amethyst::{
    core::timing::Time,
    core::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use states::Mob;

pub struct MobSystem;

impl<'s> System<'s> for MobSystem {
    type SystemData = (
        ReadStorage<'s, Mob>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mobs, mut locals, time): Self::SystemData) {
        for (mob, local) in (&mobs, &mut locals).join() {
            local.translation[0] = local.translation[0] + mob.velocity[0] * time.delta_seconds();
            local.translation[1] = local.translation[1] + mob.velocity[1] * time.delta_seconds();
        }
    }
}