use bevy::asset::io::memory::Value::Vec;
use bevy::prelude::*;
use crate::load::GameTexture;
use crate::ui::menu::{full_screen_node_setup, MenuState};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LandSelected {
            land: LandAction::TreeSelect,
            size: EditorAction::Double,
        }).add_systems(OnEnter(MenuState::MapEdit), editor_setup)
            .add_systems(Update, (print_mouse_position_system, editor_border_selected_update, editor_left_land_action).run_if(in_state(MenuState::MapEdit)));
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
enum EditorAction {
    Double,
    Single,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
enum LandAction {
    TreeSelect,
    IceSelect,
    IronSelect,
    BrickSelect,
    //WaterSelect,
    //GrassSelect,
    NoneSelect,
}

#[derive(Resource)]
struct LandSelected {
    land: LandAction,
    size: EditorAction,
}

#[derive(Component)]
struct EditorAdd;

const SELECTED_BORDER: Color = Color::RED;
const DEFAULT_BORDER: Color = Color::rgb(0.6, 0.6, 0.6);

fn editor_setup(mut commands: Commands, textures: Res<GameTexture>) {
    commands.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(48.),
            height: Val::Px(48.),
            border: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        border_color: SELECTED_BORDER.into(),
        transform: Transform::from_translation(Vec3::new(0.,0.,3.)),
        ..Default::default()
    }, EditorAdd));
    commands.spawn(full_screen_node_setup()).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(730.),
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
                editor_double_land_setup(p, &textures.tree, LandAction::TreeSelect);
                editor_single_land_setup(p, &textures.tree, LandAction::TreeSelect);
                editor_double_land_setup(p, &textures.ice, LandAction::IceSelect);
                editor_single_land_setup(p, &textures.ice, LandAction::IceSelect);
                editor_double_land_setup(p, &textures.iron, LandAction::IronSelect);
                editor_single_land_setup(p, &textures.iron, LandAction::IronSelect);
                editor_double_land_setup(p, &textures.brick, LandAction::BrickSelect);
                editor_single_land_setup(p, &textures.brick, LandAction::BrickSelect);
            });
            parent.spawn(AtlasImageBundle {
                texture_atlas: textures.world.clone(),
                ..Default::default()
            });
        });
    });
}

fn editor_single_land_setup(parent: &mut ChildBuilder, textures: &Handle<TextureAtlas>, land_action: LandAction) {
    parent.spawn(ButtonBundle {
        style: Style {
            border: UiRect::all(Val::Px(2.)),
            width: Val::Px(28.),
            height: Val::Px(28.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        border_color: DEFAULT_BORDER.into(),
        ..Default::default()
    }).insert(EditorAction::Single).insert(land_action).with_children(|p| {
        p.spawn(AtlasImageBundle {
            texture_atlas: textures.clone(),
            transform: Transform::from_scale(Vec3::ONE),
            ..Default::default()
        });
    });
}

fn editor_double_land_setup(parent: &mut ChildBuilder, textures: &Handle<TextureAtlas>, land_action: LandAction) {
    parent.spawn(ButtonBundle {
        style: Style {
            border: UiRect::all(Val::Px(2.)),
            width: Val::Px(52.),
            height: Val::Px(52.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(2.)),
            ..Default::default()
        },
        border_color: DEFAULT_BORDER.into(),
        ..Default::default()
    }).insert(EditorAction::Double).insert(land_action).with_children(|p| {
        p.spawn(AtlasImageBundle {
            texture_atlas: textures.clone(),
            transform: Transform::from_scale(Vec3::ONE),
            ..Default::default()
        });
        p.spawn(AtlasImageBundle {
            texture_atlas: textures.clone(),
            transform: Transform::from_scale(Vec3::ONE),
            ..Default::default()
        });
        p.spawn(AtlasImageBundle {
            texture_atlas: textures.clone(),
            transform: Transform::from_scale(Vec3::ONE),
            ..Default::default()
        });
        p.spawn(AtlasImageBundle {
            texture_atlas: textures.clone(),
            transform: Transform::from_scale(Vec3::ONE),
            ..Default::default()
        });
    });
}

fn editor_left_land_action(mut query: Query<(&Interaction, &LandAction, &EditorAction), (Changed<Interaction>, With<Button>)>, mut land_selected: ResMut<LandSelected>) {
    for (interaction, land, editor) in &mut query {
        if *interaction == Interaction::Pressed {
            land_selected.land = land.clone();
            land_selected.size = editor.clone();
        }
    }
}

fn editor_border_selected_update(mut query: Query<(&mut BorderColor, &LandAction, &EditorAction), With<Button>>, land_selected: Res<LandSelected>) {
    for (mut border, land, size) in &mut query {
        *border = if land_selected.land == *land && land_selected.size == *size {
            SELECTED_BORDER.into()
        } else { DEFAULT_BORDER.into() };
    }
}

fn print_mouse_position_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Query<&Window>,
    mut editor_add: Query<&mut Transform, With<EditorAdd>>,
) {
    let window = windows.single();
    for event in cursor_moved_events.read() {
        let pos = get_window_position(&event.position, window, &Vec2::new(100., 0.));
        if pos.x < 312. && pos.x > -312. && pos.y < 312. && pos.y > -312. {
            let _ = editor_add.single_mut().with_translation(Vec3::new(pos.x, pos.y, 2.));
            /*(*editor_add.single_mut()).translation = Vec3::new(pos.x, pos.y, 2.);*/
            println!("123")
        }
    }
}

fn get_window_position(pos: &Vec2, window: &Window, offset: &Vec2) -> Vec2 {
    Vec2::new(pos.x - (window.width() + offset.x) / 2., pos.y - (window.height() + offset.y) / 2.)
}