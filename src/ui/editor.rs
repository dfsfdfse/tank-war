use bevy::prelude::*;
use crate::ui::menu::MenuState;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::MapEdit), editor_setup);
    }
}

fn editor_setup() {}