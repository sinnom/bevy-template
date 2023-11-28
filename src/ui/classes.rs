use bevy::prelude::*;
use bevy_ui_dsl::class_helpers::*;
use bevy_ui_dsl::*;

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = pc(100.);
    b.style.height = pc(100.)
}

pub fn c_half(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.width = pc(50);
    s.height = pc(100);
    s.flex_direction = COLUMN;
    s.justify_content = JUSTIFY_CENTER;
    s.align_items = ALIGN_CENTER;
    s.padding = all(px(10));
}

pub fn c_green(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 212, 148).into();
}

pub fn c_blue(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 164, 212).into();
}

pub fn c_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_button_left(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(66, 135, 245).into();
    // b.image = assets.load("button.png").into();
}

pub fn c_button_right(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(57, 179, 118).into();
    // b.image = assets.load("button.png").into();
}

pub fn c_grid(b: &mut NodeBundle) {
    b.style.width = Val::Px(200.);
    b.style.height = Val::Px(200.);
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_inv_slot(assets: &AssetServer, b: &mut ImageBundle) {
    b.style.width = Val::Px(32.);
    b.style.height = Val::Px(32.);
    // b.image = assets.load("item_slot.png").into();
}

pub fn c_pixel(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("Comfortaa-Bold.ttf").into();
    s.font_size = 8.;
    s.color = Color::WHITE.into();
}
