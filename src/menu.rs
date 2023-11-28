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

fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.)
}

fn c_half(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.width = Val::Percent(50.);
    s.height = Val::Percent(100.);
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.padding = UiRect::all(Val::Px(10.));
}

fn c_green(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 212, 148).into();
}

fn c_blue(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 164, 212).into();
}

fn c_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style.margin = UiRect::all(Val::Px(10.));
}

fn c_button_left(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(66, 135, 245).into();
    // b.image = assets.load("button.png").into();
}

fn c_button_right(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(57, 179, 118).into();
    // b.image = assets.load("button.png").into();
}

fn c_grid(b: &mut NodeBundle) {
    b.style.width = Val::Px(200.);
    b.style.height = Val::Px(200.);
    b.style.margin = UiRect::all(Val::Px(10.));
}

fn c_inv_slot(assets: &AssetServer, b: &mut ImageBundle) {
    b.style.width = Val::Px(32.);
    b.style.height = Val::Px(32.);
    // b.image = assets.load("item_slot.png").into();
}

fn c_pixel(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("Comfortaa-Bold.ttf").into();
    s.font_size = 8.;
    s.color = Color::WHITE.into();
}
