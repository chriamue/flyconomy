use bevy::prelude::*;

use super::{
    styles::{get_text_style, HUD_STYLE, IMAGE_STYLE, ITEM_STYLE},
    CashText, ExpensesText, IncomeText, PlanesText, HUD,
};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..Default::default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            spawn_hud_item(parent, "Cash: ", "icons/cash.png", &asset_server, CashText);
            spawn_hud_item(
                parent,
                "Planes: ",
                "icons/airplane.png",
                &asset_server,
                PlanesText,
            );
            spawn_hud_item(
                parent,
                "Income: ",
                "icons/profit.png",
                &asset_server,
                IncomeText,
            );
            spawn_hud_item(
                parent,
                "Expenses: ",
                "icons/expense.png",
                &asset_server,
                ExpensesText,
            );
        })
        .id();

    hud_entity
}

fn spawn_hud_item(
    parent: &mut ChildBuilder,
    text: &str,
    icon_path: &str,
    asset_server: &Res<AssetServer>,
    item_component: impl Component,
) {
    parent
        .spawn(NodeBundle {
            style: ITEM_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            // Text
            parent.spawn((
                TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::from_section(text, get_text_style(asset_server)),
                    ..Default::default()
                },
                item_component,
            ));
            // Icon
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load(icon_path).into(),
                ..Default::default()
            });
        });
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
