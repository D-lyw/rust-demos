// use bevy::{app::{App, AppExit, Update}, ecs::{event::EventWriter, system::Commands}, DefaultPlugins};
use bevy::prelude::Query;
use bevy::render::extract_resource::ExtractResource;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

const ARENA_WIDTH: usize = 25;
const ARENA_HEIGHT: usize = 25;

#[derive(States, Debug, Hash, Copy, Clone, Default, PartialEq, Eq)]
struct GameState {}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Resource)]
struct BTimer(Timer);

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

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
        .add_systems(Startup, (setup_camera, setup_snake))
        
        .insert_resource(ClearColor(Color::rgb(0.2, 0.3, 0.5)))
        .insert_resource(BTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .add_systems(
            Update,
            (
                snake_movement_input.before(move_snake_v2),
                move_snake_v2,
                size_scaling,
                transform_position,
            ),
        ) // 设置背景颜色为淡蓝色
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                // translation: Vec3::new(0.0, 0.0, 0.0),
                // scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead {
            direction: Direction::Up,
        })
        .insert(Position { x: 13.0, y: 13.0 })
        .insert(Size::square(1.0));
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
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

fn move_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_position: Query<&mut Position, With<SnakeHead>>,
) {
    if let Ok(mut pos) = head_position.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            pos.x -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            pos.x += 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            pos.y -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            pos.y += 1.0;
        }
    }
}
fn move_snake_v2(
    mut heads: Query<(&mut Position, &SnakeHead)>,
    time: Res<Time>,
    mut timer: ResMut<BTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    if let Some((mut head_pos, head)) = heads.iter_mut().next() {
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1.0;
            }
            Direction::Right => {
                head_pos.x += 1.0;
            }
            Direction::Up => {
                head_pos.y += 1.0;
            }
            Direction::Down => {
                head_pos.y -= 1.0;
            }
        };
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

fn transform_position(
    window: Query<&Window, With<PrimaryWindow>>,
    mut pos: Query<(&Position, &mut Transform)>,
) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };

    fn calculate_transform(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    for (position, mut transform) in pos.iter_mut() {
        transform.translation = Vec3::new(
            calculate_transform(position.x, window.width(), ARENA_WIDTH as f32),
            calculate_transform(position.y, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
