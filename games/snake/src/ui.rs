use bevy::prelude::*;
use bevy::{
    ecs::{component::Component, system::Commands},
    ui::node_bundles::NodeBundle,
};

use crate::lifetime::{ScoreText, SpeedText, TimerText};
use crate::GameScore;

#[derive(Component)]
pub struct ResultNode;

pub fn render_layout(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: bevy::prelude::BackgroundColor(Color::WHITE),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                ..default()
                            },
                            text: Text::from_section(
                                "Score: 0",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::ORANGE_RED,
                                    font: default(),
                                },
                            ),
                            ..default()
                        })
                        .insert(ScoreText);

                    parent
                        .spawn(TextBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(0.0),
                                    Val::Px(0.0),
                                ),
                                ..default()
                            },
                            text: Text::from_section(
                                "Running: 0s",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::ORANGE_RED,
                                    font: default(),
                                },
                            ),
                            ..default()
                        })
                        .insert(TimerText);

                    parent.spawn(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::new(
                                Val::Px(20.0),
                                Val::Px(20.0),
                                Val::Px(0.0),
                                Val::Px(0.0),
                            ),
                            ..default()
                        },
                        text: Text::from_section(
                            "Speed: 0",
                            TextStyle {
                                font_size: 24.0,
                                color: Color::ORANGE_RED,
                                font: default(),
                            },
                        ),
                        ..default()
                    }).insert(SpeedText);
                });
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Vw(100.0), // 100vw
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                // background_color: BackgroundColor(Color::hex("003366").unwrap()),
                ..default()
            },));
        });
}

pub fn render_result_ui(mut commands: Commands, game_score: Res<GameScore>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                top: Val::Percent(35.0),
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
