use bevy::prelude::*;

pub struct HudStyle;

impl HudStyle {
    pub fn style() -> Style {
        Style {
            display: Display::Flex,
            justify_self: JustifySelf::End,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            width: Val::Percent(60.0),
            height: Val::Percent(15.0),
            margin: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
            ..Style::DEFAULT
        }
    }

    pub fn item() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(0.0), Val::Px(0.0)),
            ..Style::DEFAULT
        }
    }

    pub fn image() -> Style {
        Style {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            margin: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
            ..Style::DEFAULT
        }
    }

    pub fn text() -> Style {
        Style {
            width: Val::Auto,
            height: Val::Auto,
            min_width: Val::Px(80.0),
            min_height: Val::Auto,
            padding: UiRect::new(Val::Px(4.0), Val::Px(4.0), Val::Px(4.0), Val::Px(4.0)),
            ..Style::DEFAULT
        }
    }
}

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    }
}
