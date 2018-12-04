#![cfg_attr(feature="nightly", feature(amethyst))]
extern crate amethyst;
#[macro_use]
extern crate serde_derive;

mod states;
mod systems;
mod config;

use amethyst::{
	core::transform::TransformBundle,
    //core::frame_limiter::FrameRateLimitStrategy,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DrawSprite,
        Pipeline, RenderBundle, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};
//use std::time::Duration;

use crate::config::GameConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use crate::states::StartState;

    let config_path = format!(
        "{}/resources/config/config.ron",
        application_root_dir()
    );
    let config = GameConfig::load(&config_path);

    let binding_path = format!(
        "{}/resources/config/bindings.ron",
        application_root_dir()
    );
    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.04, 0.04, 0.10, 1.0], 1.0)
            //.with_pass(DrawFlat::<PosNormTex>::new()),
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),

    );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config.display)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::MobSystem, "mob_system", &[])
        .with(systems::BounceSystem, "collision_system", &["player_system", "mob_system"])
        .with(systems::ScoreSystem, "score_system", &["mob_system"]);

    let mut game = Application::build("./", StartState)?
        //.with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 144)
        .with_resource(config.level)
        .with_resource(config.player)
        .with_resource(config.mob)
        .build(game_data)?;

    game.run();

    Ok(())
}
