pub mod food;
pub mod snake;
pub mod ui;
use bevy::prelude::Query;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy::{app::AppExit, prelude::*};
use food::{spawn_foods, Food, FoodSpawnTimer};
use snake::{handle_snake_growing, move_snake, setup_snake, snake_eating, spawn_segment, transform_position, Direction, LastTailPosition, Size, SnakeHead, SnakeSegments};
use ui::{render_result_ui, ResultNode};

const ARENA_WIDTH: usize = 25;
const ARENA_HEIGHT: usize = 25;

#[derive(States, Debug, Hash, Copy, Clone, Default, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Running,
    Plause,
    GameOver,
}

#[derive(Resource, Debug, Hash, Copy, Clone, Default, PartialEq)]
pub struct GameScore {
    score: i32,
}

#[derive(Event)]
pub struct EatFood;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

/// the logic sets
/// 1. snake move, eat and growing  
/// 2. food spawn
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SnakeMoveSet;

#[derive(Resource)]
pub struct BTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Playground".to_string(),
                resolution: WindowResolution::new(800.0, 800.0),
                ..Default::default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::hex("003366").unwrap()))
        .insert_resource(BTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(GameScore { score: 0 })
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .init_state::<GameState>()
        .add_event::<EatFood>()
        .configure_sets(
            Update,
            SnakeMoveSet
                .after(snake_movement_input)
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Running), setup_snake)
        .add_systems(
            Update,
            (
                snake_movement_input,
                (
                    move_snake.after(snake_movement_input),
                    snake_eating,
                    handle_snake_growing,
                    size_scaling,
                    transform_position,
                    spawn_foods,
                )
                    .in_set(SnakeMoveSet),
            ),
        ) // 设置背景颜色为淡蓝色
        .add_systems(OnEnter(GameState::GameOver), game_over_cleanup)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn snake_movement_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut result_node: Query<Entity, With<ResultNode>>,
    mut game_score: ResMut<GameScore>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && *game_state.get() == GameState::GameOver {
        commands
            .entity(result_node.get_single_mut().unwrap())
            .despawn_recursive();
        game_score.score = 0;
        next_game_state.set(GameState::Running);
        return;
    }

    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn size_scaling(
    primary_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = primary_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn game_over_cleanup(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
    foods: Query<Entity, With<Food>>,
    game_score: Res<GameScore>,
) {
    for food_entity in foods.iter() {
        commands.entity(food_entity).despawn_recursive();
    }
    for segment in segments.iter_mut() {
        commands.entity(*segment).despawn_recursive();
    }

    render_result_ui(commands, game_score);
}
