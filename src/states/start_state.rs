use amethyst::assets::{Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Projection,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::winit::VirtualKeyCode;

use crate::config::LevelConfig;
use crate::states::GameState;
use crate::states::ScoreText;

pub struct StartState;

impl<'a, 'b> SimpleState<'a, 'b> for StartState {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("StartState on_start");
        let world = data.world;
        initialise_camera(world);
        initialise_text(world);
        initialise_scoreboard(world);
        show_text(world);
    }
    
    fn on_pause(&mut self, data: StateData<GameData>) {
        hide_text(data.world);
    }
    
    fn on_resume(&mut self, data: StateData<GameData>) {
        show_text(data.world);
    }
    

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Return){
                Trans::Push(Box::new(GameState))
            } else if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    let (level_height, level_width) = {
        let config = &world.read_resource::<LevelConfig>();
        (config.height, config.width)
    };

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            level_width,
            level_height,
            0.0,
        )))
        .with(transform)
        .build();
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

    let text1 = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "Sacrifices must be made...".to_string(),
            [0.08, 0.08, 0.20, 1.],
            level_height/10.0,
        )).build();
        
    let text_transform = UiTransform::new(
        "Instructions".to_string(), Anchor::Middle,
        0., -level_height/10.0, 1., level_width, level_height/2.0, 0,
    );

    let text2 = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "Press Enter to Start".to_string(),
            [0.08, 0.08, 0.20, 1.],
            level_height/20.0,
        )).build();
        
    world.add_resource(WelcomeText { text1, text2 });
}

fn show_text(world: &mut World) {
    let level_height = {
        let config = &world.read_resource::<LevelConfig>();
        config.height
    };

    let mut ui_transform = world.write_storage::<UiTransform>();
    let text = world.read_resource::<WelcomeText>();
    if let Some(transform) = ui_transform.get_mut(text.text1) {
        transform.local_y = 0.0;
    }
    if let Some(transform) = ui_transform.get_mut(text.text2) {
        transform.local_y = -level_height / 10.0;
    }
}

fn hide_text(world: &mut World) {
    let level_height = {
        let config = &world.read_resource::<LevelConfig>();
        config.height
    };

    let mut ui_transform = world.write_storage::<UiTransform>();
    let text = world.read_resource::<WelcomeText>();
    if let Some(transform) = ui_transform.get_mut(text.text1) {
        transform.local_y = level_height*2.0;
    }
    if let Some(transform) = ui_transform.get_mut(text.text2) {
        transform.local_y = level_height*2.0;
    }
}

struct WelcomeText {
    pub text1: Entity,
    pub text2: Entity,
}


fn initialise_scoreboard(world: &mut World) {
    let (level_height, level_width) = {
        let config = &world.read_resource::<LevelConfig>();
        (config.height, config.width)
    };

    let font = world.read_resource::<Loader>().load(
        "resources/font/kenvector_future_thin.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let transform = UiTransform::new(
        "Score".to_string(), Anchor::TopLeft,
        level_width / 5.0, level_height, 1., level_width, level_height / 5.0, 0,
    );

    let score = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        )).build();
        
    world.add_resource(ScoreText { score });
}
