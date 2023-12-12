use bevy::prelude::*;
use crate::load::GameTexture;
use crate::ui::menu::MenuState;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::MapEdit), editor_setup);
    }
}

#[derive(Component)]
enum EditorAction {
    TreeSelect,
    IceSelect,
    IronSelect,
    BrickSelect,
    //WaterSelect,
    //GrassSelect,
    NoneSelect,
}

const SELECTED_BORDER: Color = Color::rgb(0.8, 0.8, 0.8);
const DEFAULT_BORDER: Color = Color::rgb(0.6, 0.6, 0.6);

fn editor_setup(mut commands: Commands, textures: Res<GameTexture>) {
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
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(700.),
                height: Val::Px(630.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    border: UiRect::new(Val::Px(3.), Val::Px(0.), Val::Px(3.), Val::Px(3.)),
                    width: Val::Px(60.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Start,
                    flex_wrap: FlexWrap::Wrap,
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                border_color: Color::rgb(0.6, 0.6, 0.6).into(),
                ..Default::default()
            }).with_children(|p| {
                editor_block_setup(p, textures.tree.clone(), EditorAction::TreeSelect);
                editor_block_setup(p, textures.ice.clone(), EditorAction::IceSelect);
                editor_block_setup(p, textures.iron.clone(), EditorAction::IronSelect);
                editor_block_setup(p, textures.brick.clone(), EditorAction::BrickSelect);
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: textures.world.clone(),
                ..Default::default()
            });
        });
    });
}

fn editor_block_setup(parent: &mut ChildBuilder, textures: Handle<TextureAtlas>, action: EditorAction) {
    parent.spawn(ButtonBundle {
        style: Style {
            border: UiRect::all(Val::Px(2.)),
            width: Val::Px(52.),
            height: Val::Px(52.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        border_color: DEFAULT_BORDER.into(),
        ..Default::default()
    }).insert(action).with_children(|p| {
        p.spawn(AtlasImageBundle {
            texture_atlas: textures,
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..Default::default()
        });
    });
}