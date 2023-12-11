use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;
use crate::GameState;
use crate::world::Moving;
use bevy_common_assets::json::JsonAssetPlugin;
use crate::utils::Boundary;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::LoadTexture).continue_to_state(GameState::LoadConfig),
        )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(GameState::LoadTexture, "load.assets.ron")
            .add_collection_to_loading_state::<_, GameTexture>(GameState::LoadTexture)
            .add_plugins(JsonAssetPlugin::<GameConfig>::new(&["json"]))
            .insert_resource(Msaa::Off)
            .add_systems(Startup, load_game_config)
            .add_systems(Update, insert_resource_game_config.run_if(in_state(GameState::LoadConfig)))
        ;
    }
}

fn load_game_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let json = GameConfigHandle(asset_server.load("resource.json"));
    commands.insert_resource(json);
}

//将可能还在加载的game_config加入到resource池子中
fn insert_resource_game_config(mut commands: Commands, game_config: Res<GameConfigHandle>, mut game_configs: ResMut<Assets<GameConfig>>, mut state: ResMut<NextState<GameState>>) {
    if let Some(game_config) = game_configs.remove(game_config.0.id()) {
        commands.insert_resource(game_config);
        state.set(GameState::Menu)
    }
}

#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GameMap {
    pub size: Vec2,
    pub data: Vec<Vec<LandType>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LandType {
    Tree,
    Ice,
    Brick,
    Iron,
    Water,
    Grass,
    None,
}

#[derive(Deserialize, Asset, TypePath, Resource)]
pub struct GameConfig {
    pub world: WorldConfig,
    pub player1: PlayerConfig,
    pub player2: PlayerConfig,
}

#[derive(Deserialize)]
pub struct WorldConfig {
    pub size: Vec3,
    pub step: f32,
    pub boundary: Boundary,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    pub moving: Moving,
    pub position: Vec3,
}

#[derive(AssetCollection, Resource)]
pub struct GameTexture {
    #[asset(key = "player1_s")]
    pub p1_1: Handle<TextureAtlas>,
    #[asset(key = "player1_m")]
    pub p1_2: Handle<TextureAtlas>,
    #[asset(key = "player1_l")]
    pub p1_3: Handle<TextureAtlas>,
    #[asset(key = "player2_s")]
    pub p2_1: Handle<TextureAtlas>,
    #[asset(key = "player2_m")]
    pub p2_2: Handle<TextureAtlas>,
    #[asset(key = "player2_l")]
    pub p2_3: Handle<TextureAtlas>,
    #[asset(key = "world")]
    pub world: Handle<TextureAtlas>,
    #[asset(key = "logo")]
    pub logo: Handle<TextureAtlas>,
    #[asset(key = "font")]
    pub font: Handle<Font>,
}

