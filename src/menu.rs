use bevy::prelude::*;

use crate::GameState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MenuSet;

fn render_menu(state: Res<State<GameState>>) {}
