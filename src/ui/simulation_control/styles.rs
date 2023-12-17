use bevy::prelude::*;

pub struct SimulationControlStyle;

impl SimulationControlStyle {
    pub fn style() -> Style {
        Style {
            display: Display::Flex,
            align_self: AlignSelf::FlexStart,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: Val::Auto,
            height: Val::Percent(15.0),
            ..Style::DEFAULT
        }
    }

    pub fn item() -> Style {
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(0.0), Val::Px(0.0)),
            ..Style::DEFAULT
        }
    }

    pub fn image() -> Style {
        Style {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..Style::DEFAULT
        }
    }
}
