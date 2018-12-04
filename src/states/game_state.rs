use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Join};
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{
    MaterialTextureSet, PngFormat, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::ui::{UiTransform};
use amethyst::winit::VirtualKeyCode;

use crate::config::{LevelConfig, PlayerConfig, MobConfig};
use crate::states::GameOverState;

pub struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        //world.register::<Mob>();

        initialise_mob(world, sprite_sheet_handle.clone(), 0.0, 0.0, 0.0);
        initialise_mob(world, sprite_sheet_handle.clone(), 30.0, 50.0, 10.0);
        initialise_players(world, sprite_sheet_handle);
        show_scoreboard(world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let world = data.world;
        hide_scoreboard(world);
        let mut dead_entities: Vec<Entity> = Vec::new();
        {
            let mut mobs = world.write_storage::<Mob>();
            for (entity, _) in (&*world.entities(), &mobs).join() {
                dead_entities.push(entity);
            }
            mobs.clear();
        }
        {
            let mut players = world.write_storage::<Player>();
            for (entity, _) in (&*world.entities(), &players).join() {
                dead_entities.push(entity);
            }
            players.clear();
        }
        for entity in dead_entities {
            world.delete_entity(entity).expect("Couldn't delete entities.");
        }
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Pop
            } else if is_key_down(&event, VirtualKeyCode::Q) {
                Trans::Switch(Box::new(GameOverState))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

pub struct Player {
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new(width: f32, height: f32) -> Player {
        Player {
            width: width,
            height: height,
        }
    }
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

pub struct Mob {
    pub velocity: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub touched: bool,
}

impl Component for Mob {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score: i32,
}

pub struct ScoreText {
    pub score: Entity,
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "resources/texture/player_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    
    let texture_id = 0;
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "resources/texture/player_spritesheet.ron",
        SpriteSheetFormat,
        texture_id,
        (),
        &sprite_sheet_store,
    )
}

fn initialise_players(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let (level_height, level_width) = {
        let config = &world.read_resource::<LevelConfig>();
        (config.height, config.width)
    };
    let (player_width, player_height) = {
        let config = &world.read_resource::<PlayerConfig>();
        (config.width, config.height)
    };

    let y = level_height * 0.5;
    left_transform.translation = Vector3::new(player_width * 0.5, y, 0.0);
    right_transform.translation = Vector3::new(level_width - player_width * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Player::new(player_width, player_height))
        .with(left_transform)
        .build();
}


fn initialise_mob(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    offset_x: f32,
    offset_y: f32,
    offset_v: f32,
) {
    let (level_height, level_width) = {
        let config = &world.read_resource::<LevelConfig>();
        (config.height, config.width)
    };

    let (mob_width, mob_height, mob_speed) = {
        let config = &world.read_resource::<MobConfig>();
        (config.width, config.height, config.speed)
    };

    let mut local_transform = Transform::default();
    local_transform.translation = Vector3::new(
        offset_x + level_width * 0.5,
        offset_y + level_height * 0.5,
        0.0
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 5,
        flip_horizontal: false,
        flip_vertical: false,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Mob {
            width: mob_width,
            height: mob_height,
            velocity: [mob_speed+offset_v, mob_speed-25.0-offset_v],
            touched: false,
        })
        .with(local_transform)
        .build();
}


fn show_scoreboard(world: &mut World) {
    let level_height = {
        let config = &world.read_resource::<LevelConfig>();
        config.height
    };

    let mut ui_transform = world.write_storage::<UiTransform>();
    let score_text = world.read_resource::<ScoreText>();
    if let Some(transform) = ui_transform.get_mut(score_text.score) {
        transform.local_y = -level_height / 10.0;
    }
}

fn hide_scoreboard(world: &mut World) {
    let level_height = {
        let config = &world.read_resource::<LevelConfig>();
        config.height
    };

    let mut ui_transform = world.write_storage::<UiTransform>();
    let score_text = world.read_resource::<ScoreText>();
    if let Some(transform) = ui_transform.get_mut(score_text.score) {
        transform.local_y = level_height;
    }
}
