use bevy::prelude::*;
use crate::GameState;
use crate::load::{GameConfig, GameTexture};
use crate::utils::{Vec2Ext, Vec3Ext};

pub struct MenuPlugin;

#[derive(Component)]
pub struct Menu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(Update, ui_event.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component, Clone)]
struct GameButton {
    text: String,
    normal: Color,
    hovered: Color,
    font: Option<Handle<Font>>,
}

impl Default for GameButton {
    fn default() -> Self {
        GameButton {
            text: "".into(),
            normal: Color::rgb(0.2, 0.2, 0.2),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            font: None,
        }
    }
}

fn spawn_menu(mut commands: Commands, game_config: Res<GameConfig>, game_texture: Res<GameTexture>, assets: Res<Assets<TextureAtlas>>) {
    if let Some(logo) = assets.get(&game_texture.logo) {
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Menu).with_children(|parent| {
            parent.spawn(AtlasImageBundle {
                texture_atlas: game_texture.logo.clone(),
                transform: Transform {
                    scale: logo.size.to_vec3().scale_x(&(game_config.world.size * (game_config.world.step * 0.8))),
                    ..default()
                },
                ..Default::default()
            });
        }).with_children(|parent| {
            spawn_button(parent, GameButton { text: "single player".to_string(), font: Some(game_texture.font.clone()), ..Default::default() });
        }).with_children(|parent| {
            spawn_button(parent, GameButton { text: "double player".to_string(), font: Some(game_texture.font.clone()), ..Default::default() });
        }).with_children(|parent| {
            spawn_button(parent, GameButton { text: "level editor".to_string(), font: Some(game_texture.font.clone()), ..Default::default() });
        });
    };
}

fn player1_click(state: &mut NextState<GameState>, text: String) {
    println!("{}", text);
    state.set(GameState::Spawn);
}

fn player2_click(_: &mut NextState<GameState>, text: String) {
    println!("{}", text);
}

fn map_editor_click(_: &mut NextState<GameState>, text: String) {
    println!("{}", text);
}

fn spawn_button(parent: &mut ChildBuilder, game_button: GameButton) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(140.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(20.)),
            ..Default::default()
        },
        background_color: game_button.normal.into(),
        ..Default::default()
    }).insert(game_button.clone()).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            game_button.text.clone(),
            TextStyle {
                font: game_button.font.unwrap(),
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..Default::default()
            }));
    });
}

fn ui_event(
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &GameButton),
        (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, game_button) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                //game_button.text single player
                if game_button.text == "single player".to_string() {
                    player1_click(&mut state, game_button.text.clone());
                } else if game_button.text == "double player".to_string() {
                    player2_click(&mut state, game_button.text.clone());
                } else if game_button.text == "level editor".to_string() {
                    map_editor_click(&mut state, game_button.text.clone());
                }
            }
            Interaction::Hovered => {
                *color = game_button.hovered.into();
            }
            Interaction::None => {
                *color = game_button.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}