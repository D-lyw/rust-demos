use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    food::Food, BTimer, EatFood, GameScore, GameState, Position, ARENA_HEIGHT, ARENA_WIDTH,
};

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct SnakeSegment;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub fn setup_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("#ff6666").unwrap(),
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 13.0, y: 13.0 })
            .insert(Size::square(1.0))
            .id(),
        spawn_segment(commands, Position { x: 13.0, y: 12.0 }),
    ]);
}

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::hex("ff9966").unwrap(),
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.92))
        .id()
}

pub fn move_snake(
    time: Res<Time>,
    mut timer: ResMut<BTimer>,
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|segment| *positions.get_mut(*segment).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();

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

        // out of game bounds check
        if head_pos.x < 0.0
            || head_pos.x >= ARENA_WIDTH as f32
            || head_pos.y < 0.
            || head_pos.y >= ARENA_HEIGHT as f32
            || segment_positions.contains(&head_pos)
        {
            next_game_state.set(GameState::GameOver);
            return;
        }

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

pub fn transform_position(
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

pub fn snake_eating(
    mut commands: Commands,
    mut food_events: EventWriter<EatFood>,
    foods_position: Query<(Entity, &Position), With<Food>>,
    header: Query<&Position, With<SnakeHead>>,
    mut game_score: ResMut<GameScore>,
) {
    let head_pos = header.get_single().unwrap();
    for (food_entity, food_pos) in foods_position.iter() {
        if head_pos == food_pos {
            commands.entity(food_entity).despawn_recursive();
            food_events.send(EatFood);
            game_score.score += 1;
        }
    }
}

pub fn handle_snake_growing(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut snake_eating_events: EventReader<EatFood>,
) {
    if snake_eating_events.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}
