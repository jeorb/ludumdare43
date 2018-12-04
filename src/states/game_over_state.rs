use amethyst::assets::{Loader};
use amethyst::ecs::prelude::{Entity};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::winit::VirtualKeyCode;

use crate::{config::LevelConfig};

pub struct GameOverState;

impl<'a, 'b> SimpleState<'a, 'b> for GameOverState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialise_text(world);
    }
    
    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Return) ||
                is_key_down(&event, VirtualKeyCode::Escape) ||
                is_key_down(&event, VirtualKeyCode::Q)
                {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn initialise_text(world: &mut World) {
    let (level_width, level_height) = {
        let config = &world.read_resource::<LevelConfig>();
        (config.width, config.height)
    };

    let font = world.read_resource::<Loader>().load(
        "resources/font/kenvector_future_thin.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let text_transform = UiTransform::new(
        "Title".to_string(), Anchor::Middle,
        0., 0., 1., level_width*2.0, level_height/2.0, 0,
    );

    let text = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "Game Over".to_string(),
            [0.08, 0.08, 0.20, 1.],
            level_height/10.0,
        )).build();
        
    world.add_resource(GameOverText { text });
}

struct GameOverText {
    pub text: Entity,
}

