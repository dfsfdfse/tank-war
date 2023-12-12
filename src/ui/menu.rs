use bevy::prelude::*;
use lazy_static::lazy_static;
use crate::GameState;
use crate::load::{GameConfig, GameTexture};
use crate::utils::{Vec2Ext, Vec3Ext};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>().add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(Update, button_system.run_if(in_state(MenuState::Main)))
            .add_systems(OnExit(MenuState::Main), menu_cleanup);
    }
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum MenuState {
    Main,
    Player1,
    Player2,
    MapEdit,
    #[default]
    Disabled,
}

#[derive(Component)]
enum MenuAction {
    Main,
    Player1,
    Player2,
    MapEdit,
}

const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

pub fn full_screen_node_setup() -> NodeBundle {
    NodeBundle {
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
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, game_config: Res<GameConfig>, game_texture: Res<GameTexture>, assets: Res<Assets<TextureAtlas>>) {
    commands.spawn(full_screen_node_setup()).insert(MenuAction::Main).with_children(|parent| {
        if let Some(logo) = assets.get(&game_texture.logo) {
            parent.spawn(AtlasImageBundle {
                texture_atlas: game_texture.logo.clone(),
                transform: Transform::from_scale(logo.size.to_vec3().scale_x(&(game_config.world.size * (game_config.world.step * 0.8)))),
                ..Default::default()
            });
        }
    }).with_children(|parent| {
        button_setup(parent, MenuAction::Player1, "single");
    }).with_children(|parent| {
        button_setup(parent, MenuAction::Player2, "double");
    }).with_children(|parent| {
        button_setup(parent, MenuAction::MapEdit, "editor");
    });
}

fn button_setup(parent: &mut ChildBuilder, action: MenuAction, text: &str) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(140.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(20.)),
            ..Default::default()
        },
        background_color: NORMAL_BUTTON.into(),
        ..Default::default()
    }).insert(action).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..Default::default()
            }));
    });
}

fn button_system(mut query: Query<(&Interaction, &mut BackgroundColor, &MenuAction), (Changed<Interaction>, With<Button>)>, mut game_state: ResMut<NextState<GameState>>, mut map_state: ResMut<NextState<MenuState>>) {
    for (interaction, mut color, action) in &mut query {
        *color = match *interaction {
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::Pressed => PRESSED_BUTTON.into(),
            _ => NORMAL_BUTTON.into()
        };
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::Player1 => {
                    game_state.set(GameState::Spawn);
                    println!("player1");
                }
                MenuAction::Player2 => {
                    game_state.set(GameState::Spawn);
                    println!("player2");
                }
                MenuAction::MapEdit => {
                    println!("map editor");
                    map_state.set(MenuState::MapEdit);
                }
                _ => {}
            }
        }
    }
}

fn menu_cleanup(mut commands: Commands, menu: Query<Entity, With<MenuAction>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}