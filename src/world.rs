use std::collections::{HashMap, VecDeque};
use bevy::prelude::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use crate::GameState;
use crate::load::{GameConfig, GameTexture};
use crate::utils::Vec3Ext;

pub struct WorldPlugin;

lazy_static! {
    pub static ref GAME_BLOCK: Vec3 = Vec3::new(24., 24., 0.);
    pub static ref GAME_X_STEP: Vec3 = Vec3::new(24., 0., 0.);
    pub static ref GAME_Y_STEP: Vec3 = Vec3::new(0., 24., 0.);
    pub static ref GAME_Z_INDEX: Vec3 = Vec3::new(0., 0., 1.);
    pub static ref GAME_PLAYER1_KEY_CODE: [KeyCode; 4] = [KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D];
    pub static ref GAME_PLAYER2_KEY_CODE: [KeyCode; 4] = [KeyCode::Up, KeyCode::Left, KeyCode::Down, KeyCode::Right];
    pub static ref GAME_DIRECTION_SPEED: HashMap<GameDirection, Vec3> = {
        let mut m = HashMap::new();
        m.insert(GameDirection::Left, Vec3::new(-1., 0., 0.));
        m.insert(GameDirection::Right, Vec3::new(1., 0., 0.));
        m.insert(GameDirection::Up, Vec3::new(0., 1., 0.));
        m.insert(GameDirection::Down, Vec3::new(0., -1., 0.));
        m
    };
    pub static ref GAME_TANK_DIRECTION_INDEX: HashMap<GameDirection, [usize;2]> = {
        let mut m = HashMap::new();
        m.insert(GameDirection::Up, [0, 1]);
        m.insert(GameDirection::Down, [2, 3]);
        m.insert(GameDirection::Left, [4, 5]);
        m.insert(GameDirection::Right, [6, 7]);
        m
    };
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyState>().add_systems(Startup, add_camera2d)
            .add_systems(OnEnter(GameState::Spawn), (spawn_player, spawn_world))
            .add_systems(Update, (moving_action, player1_turns, player2_turns, player1_moving, player2_moving, save_key_bind).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Deserialize, Clone, Component)]
pub struct Moving {
    pub speed: f32,
    pub direction: GameDirection,
}

#[derive(Component)]
struct GamePanel;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Default, Resource)]
struct KeyState {
    input1_queue: VecDeque<KeyCode>,
    input2_queue: VecDeque<KeyCode>,
}

fn add_camera2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

//生成游戏布局
fn spawn_world(mut commands: Commands, game_texture: Res<GameTexture>) {
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: game_texture.world.clone(),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    }).insert(GamePanel);
}

fn spawn_player(mut commands: Commands, game_texture: Res<GameTexture>, game_config: Res<GameConfig>, mut game_state: ResMut<NextState<GameState>>) {
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(GAME_TANK_DIRECTION_INDEX[&game_config.player1.moving.direction][0]),
        texture_atlas: game_texture.p1_1.clone(),
        transform: Transform::from_translation(*GAME_X_STEP * game_config.player1.position.x + *GAME_Y_STEP * game_config.player1.position.y + *GAME_Z_INDEX * game_config.player1.position.z),
        ..Default::default()
    }).insert(Player1).insert(game_config.player1.moving.clone());
    game_state.set(GameState::Playing);
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(GAME_TANK_DIRECTION_INDEX[&game_config.player2.moving.direction][0]),
        texture_atlas: game_texture.p2_1.clone(),
        transform: Transform::from_translation(*GAME_X_STEP * game_config.player2.position.x + *GAME_Y_STEP * game_config.player2.position.y + *GAME_Z_INDEX * game_config.player2.position.z),
        ..Default::default()
    }).insert(Player2).insert(game_config.player2.moving.clone());
}

fn moving_action(game_config: Res<GameConfig>, mut query: Query<(&mut Transform, &Moving), With<Moving>>) {
    for (mut moving_transform, moving) in &mut query {
        moving_transform.translation += moving.speed * GAME_DIRECTION_SPEED[&moving.direction];
        moving_transform.translation.boundary(&game_config.world.boundary);
    }
}

fn player1_turns(mut query: Query<(&mut TextureAtlasSprite, &Moving), With<Player1>>) {
    for (mut sprite, moving) in &mut query {
        let indices = GAME_TANK_DIRECTION_INDEX[&moving.direction];
        if moving.speed > 0. {
            sprite.index = if sprite.index == indices[0] { indices[1] } else { indices[0] };
        }
    }
}

fn player2_turns(mut query: Query<(&mut TextureAtlasSprite, &Moving), With<Player2>>) {
    for (mut sprite, moving) in &mut query {
        let indices = GAME_TANK_DIRECTION_INDEX[&moving.direction];
        if moving.speed > 0. {
            sprite.index = if sprite.index == indices[0] { indices[1] } else { indices[0] };
        }
    }
}

fn save_key_bind(mut keys: ResMut<KeyState>, keyboard_input: Res<Input<KeyCode>>) {
    for key_code in GAME_PLAYER1_KEY_CODE.iter() {
        if keyboard_input.just_pressed(*key_code) {
            keys.input1_queue.push_back(*key_code);
        } else if keyboard_input.just_released(*key_code) {
            keys.input1_queue.retain(|&k| k != *key_code);
        }
    }

    for key_code in GAME_PLAYER2_KEY_CODE.iter() {
        if keyboard_input.just_pressed(*key_code) {
            keys.input2_queue.push_back(*key_code);
        } else if keyboard_input.just_released(*key_code) {
            keys.input2_queue.retain(|&k| k != *key_code);
        }
    }
}

fn player1_moving(keys: ResMut<KeyState>, game_config: Res<GameConfig>, mut query: Query<&mut Moving, With<Player1>>) {
    for mut moving in &mut query {
        if let Some(&key_code) = keys.input1_queue.front() {
            match key_code {
                KeyCode::W => {
                    moving.direction = GameDirection::Up;
                    moving.speed = game_config.player1.moving.speed;
                }
                KeyCode::S => {
                    moving.direction = GameDirection::Down;
                    moving.speed = game_config.player1.moving.speed;
                }
                KeyCode::A => {
                    moving.direction = GameDirection::Left;
                    moving.speed = game_config.player1.moving.speed;
                }
                KeyCode::D => {
                    moving.direction = GameDirection::Right;
                    moving.speed = game_config.player1.moving.speed;
                }
                _ => {}
            }
        } else {
            moving.speed = 0.
        }
    }
}

fn player2_moving(keys: ResMut<KeyState>, game_config: Res<GameConfig>, mut query: Query<&mut Moving, With<Player2>>) {
    for mut moving in &mut query {
        if let Some(&key_code) = keys.input2_queue.front() {
            match key_code {
                KeyCode::Up => {
                    moving.direction = GameDirection::Up;
                    moving.speed = game_config.player2.moving.speed;
                }
                KeyCode::Down => {
                    moving.direction = GameDirection::Down;
                    moving.speed = game_config.player2.moving.speed;
                }
                KeyCode::Left => {
                    moving.direction = GameDirection::Left;
                    moving.speed = game_config.player2.moving.speed;
                }
                KeyCode::Right => {
                    moving.direction = GameDirection::Right;
                    moving.speed = game_config.player2.moving.speed;
                }
                _ => {}
            }
        } else {
            moving.speed = 0.
        }
    }
}



