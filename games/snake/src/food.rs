use bevy::ecs::component::Component;
use bevy::prelude::*;
use rand::random;

use crate::{snake::SnakeSegment, Position, Size, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

#[derive(Component)]
pub struct Food;

pub fn spawn_foods(
    mut commands: Commands,
    time: Res<Time>,
    mut food_spawn_timer: ResMut<FoodSpawnTimer>,
    mut foods_snake_set: ParamSet<(
        Query<&Position, With<Food>>,
        Query<&Position, With<SnakeSegment>>,
    )>,
) {
    if !food_spawn_timer.0.tick(time.delta()).finished() {
        return;
    }

    // generate food
    // max 5 eatable foods per second
    if foods_snake_set.p0().iter().len() < 5 {
        let mut new_food_position = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32).ceil(),
            y: (random::<f32>() * ARENA_HEIGHT as f32).ceil(),
        };
        // new food position could not be in snake body positions or existing foods position
        loop {
            if foods_snake_set
                .p0()
                .iter()
                .any(|food_pos| food_pos == &new_food_position)
                || foods_snake_set
                    .p1()
                    .iter()
                    .any(|food_pos| food_pos == &new_food_position)
            {
                new_food_position = Position {
                    x: (random::<f32>() * ARENA_WIDTH as f32).ceil(),
                    y: (random::<f32>() * ARENA_HEIGHT as f32).ceil(),
                };
            } else {
                break;
            }
        }

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("00ab6f").unwrap(),
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(new_food_position)
            .insert(Size::square(1.0));
    }
}
