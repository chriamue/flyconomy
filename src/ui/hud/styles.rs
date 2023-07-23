use bevy::prelude::*;

pub const HUD_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Auto, Val::Percent(15.0)),
    margin: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
    ..Style::DEFAULT
};

pub const ITEM_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
    margin: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
    ..Style::DEFAULT
};

pub const TEXT_STYLE: Style = Style {
    size: Size::new(Val::Auto, Val::Auto),
    min_size: Size::new(Val::Px(40.0), Val::Auto),
    padding: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
    ..Style::DEFAULT
};

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    }
}
