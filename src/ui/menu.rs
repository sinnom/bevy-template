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
    let mut hiya = None;
    let mut howdy = None;
    root(c_root, &assets, &mut commands, |p| {
        // Spawns the root NodeBundle. AssetServer gets propagated.
        node((c_half, c_green), p, |p| {
            // Spawns the left pane as a NodeBundle.
            text("This is the left pane!", c_text, c_pixel, p); // Spawns a TextBundle.
            text("Do you like it?", c_text, c_pixel, p);
            text_button("Hiya", c_button_left, c_pixel, p).set(&mut hiya); // Spawns a ButtonBundle with a TextBundle child in the middle. Convenience widget.

            text("Le grid", c_text, c_pixel, p);
        });
        node((c_half, c_blue), p, |p| {
            text("This is the right pane!", c_text, c_pixel, p);
            text("Indeed, I do!", c_text, c_pixel, p);
            text_button("Howdy", c_button_right, c_pixel, p).set(&mut howdy);
        });
    });

    // Inserts marker components into the gathered entities.
    // Useful when you need to interact with specific entities in the UI.
    commands.entity(hiya.unwrap()).insert(UiId::HiyaButton);
    commands.entity(howdy.unwrap()).insert(UiId::HowdyButton);
}