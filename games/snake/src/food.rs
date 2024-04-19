use bevy::ecs::component::Component;
use bevy::prelude::*;
use rand::random;

use crate::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

#[derive(Component)]
pub struct Food;

pub fn spawn_foods(
    mut commands: Commands,
    time: Res<Time>,
    mut food_spawn_timer: ResMut<FoodSpawnTimer>,
) {
    if !food_spawn_timer.0.tick(time.delta()).finished() {
        return;
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
        // TODO: food position could not be in snake body positions
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32).ceil(),
            y: (random::<f32>() * ARENA_HEIGHT as f32).ceil(),
        })
        .insert(Size::square(1.0));
}
