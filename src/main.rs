use crate::ui::menu::MenuPlugin;
use bevy::prelude::*;

mod ui;

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_state::<GameState>()
        .run();
}
