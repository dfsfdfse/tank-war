use bevy::app::App;
use tank_war::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
