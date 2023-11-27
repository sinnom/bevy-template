use bevy::prelude::*;

mod menu;

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GameStates {
    #[default]
    MainMenu,
    InGame,
    /// AKA Pause menu or escape menu
    InGameMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameStates>()
        .run();
}
