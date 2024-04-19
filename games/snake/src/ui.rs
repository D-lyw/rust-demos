use bevy::prelude::*;
use bevy::{
    ecs::{component::Component, system::Commands},
    ui::node_bundles::NodeBundle,
};

use crate::GameScore;

#[derive(Component)]
pub struct ResultNode;

pub fn render_result_ui(mut commands: Commands, game_score: Res<GameScore>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                top: Val::Percent(30.0),
                ..default()
            },
            ..default()
        })
        .insert(ResultNode)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        font: default(),
                    },
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                text: Text::from_section(
                    format!("Your Score: {}", game_score.score),
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        font: default(),
                    },
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: Val::Px(30.0),
                    ..default()
                },
                text: Text::from_section(
                    "Press blankspaces to continue",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::ORANGE_RED,
                        font: default(),
                    },
                ),
                ..default()
            });
        });
}
