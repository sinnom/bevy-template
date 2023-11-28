use bevy::prelude::*;

mod menu;

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .run();
}
