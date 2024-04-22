// use bevy::{app::Plugin, ecs::{component::Component, system::Resource}};
use bevy::prelude::*;
use std::time::{Duration, Instant};

use crate::{BTimer, GameState};

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct GameScore {
    pub score: i32,
    pub start_time: Instant,
    pub timer: Timer,
    pub speed: f32, // default start speed 5 squares/seconde
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct ScoreText; // mark score text node

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct TimerText; // mark timer text node

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct SpeedText; // mark speed text node

pub struct LifetimePlugin;
impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameScore {
            score: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            start_time: Instant::now(),
            speed: 1.0 / 5.0 as f32,
        })
        .add_systems(
            Update,
            (update_score_test, update_running_time, update_speed_text),
        )
        .add_systems(OnEnter(GameState::Running), reset_game_time);
    }
}

fn update_score_test(score: Res<GameScore>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    let mut score_text = score_text.single_mut();
    score_text.sections[0].value = format!("Score: {}", score.score);
}

fn update_running_time(
    time: Res<Time>,
    mut game_running_time: ResMut<GameScore>,
    mut time_text: Query<&mut Text, With<TimerText>>,
    game_state: Res<State<GameState>>,
) {
    if !game_running_time.timer.tick(time.delta()).just_finished() {
        return;
    }
    if *game_state.get() != GameState::Running {
        return;
    }
    let duration_time = game_running_time.start_time.elapsed();
    let mut time_text = time_text.single_mut();
    time_text.sections[0].value = format!(
        "Running: {}s {:3}ms",
        duration_time.as_secs(),
        duration_time.subsec_millis()
    );
}

pub fn update_speed_text(
    game_info: Res<GameScore>,
    mut speed_text: Query<&mut Text, With<SpeedText>>,
) {
    let mut speed_text = speed_text.single_mut();
    speed_text.sections[0].value = format!("Speed: {}", game_info.speed);
}

pub fn reset_game_time(mut game_running_time: ResMut<GameScore>, mut move_speed: ResMut<BTimer>) {
    game_running_time.start_time = Instant::now();
    game_running_time.speed = 5.0;

    move_speed
        .0
        .set_duration(Duration::from_secs_f32(1.0 / game_running_time.speed));
}
