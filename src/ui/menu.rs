use crate::ui::classes::*;
use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MenuSet;

#[derive(Component)]
enum UiId {
    HiyaButton,
    HowdyButton,
}

fn spawn_menu(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {
    // Obligatory camera
    commands.spawn(Camera2dBundle::default());
    scale.0 = 2.0;

    // Spawns ui and gathers entity ids
    root((c_root, c_green), &assets, &mut commands, |p| {
        node(c_menu, p, |p| {
            text_button("play", c_menu_button, c_comfortaa, p);
        });
    });
}
