mod load;
mod world;
mod utils;
mod ui;

use bevy::prelude::*;
use crate::load::LoadPlugin;
use crate::ui::menu::MenuPlugin;
use crate::world::WorldPlugin;

pub struct GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    LoadTexture,
    LoadConfig,
    Spawn,
    Playing,
    Menu,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins(
            (DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "坦克大战 bevy".to_string(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }
            ), LoadPlugin, WorldPlugin, MenuPlugin)
        );
    }
}