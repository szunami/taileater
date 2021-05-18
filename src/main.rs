use std::collections::HashSet;

use bevy::prelude::*;
use itertools::Itertools;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SnakeMovementLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct TransformLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct GravityLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct GridLocation {
    x: i32,
    y: i32,
}

impl std::ops::Add for GridLocation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Snake;

struct Ground;

struct SnakeParts(Vec<Entity>);

const GRID_WIDTH: f32 = 16.0;
const GRID_HEIGHT: f32 = 16.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts(vec![]))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(snake_movement.system().label(SnakeMovementLabel))
        .add_system(
            gravity
                .system()
                .label(GravityLabel)
                .after(SnakeMovementLabel),
        )
        .add_system(
            gridlocation_to_transform
                .system()
                .label(TransformLabel)
                .after(GravityLabel),
        )
        // gravity
        // gridlocation to transform
        .run();
}

fn setup(
    mut commands: Commands,
    mut snake_parts: ResMut<SnakeParts>,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let head_color = materials.add(Color::rgb(117.0 / 255.0, 167.0 / 255.0, 67.0 / 255.0).into());
    let ground_color = materials.add(Color::rgb(136.0 / 255.0, 75.0 / 255.0, 43.0 / 255.0).into());

    *snake_parts = SnakeParts(vec![
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .insert(Snake)
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .insert(Snake)
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .insert(Snake)
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .insert(Snake)
            .id(),
    ]);

    for i in -10..10 {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: ground_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: i, y: -1 })
            .insert(Ground)
            .id();
    }

    for i in 13..20 {
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: ground_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: i, y: -1 })
            .insert(Ground)
            .id();
    }
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_parts: Res<SnakeParts>,
    mut grid_locations: Query<&mut GridLocation, With<Snake>>,
) {
    let mut diff = GridLocation { x: 0, y: 0 };
    if keyboard_input.just_pressed(KeyCode::A) {
        diff.x -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        diff.x += 1;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        diff.y -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::W) {
        diff.y += 1;
    }

    if diff == (GridLocation { x: 0, y: 0 }) {
        return;
    }

    // TODO: check for blocked!

    for (prev, curr) in snake_parts.0.iter().zip(snake_parts.0[1..].iter()).rev() {
        if let Ok(prev_grid_location) = grid_locations.get_mut(*prev) {
            let tmp = prev_grid_location.clone();
            if let Ok(mut curr_grid_location) = grid_locations.get_mut(*curr) {
                *curr_grid_location = tmp;
            }
        }
    }

    if let Some(head) = snake_parts.0.first() {
        if let Ok(mut grid_location) = grid_locations.get_mut(*head) {
            *grid_location = grid_location.clone() + diff;
        }
    }
}

fn gravity(
    snake_parts: Query<(&Snake, Entity)>,
    ground: Query<(&Ground, Entity)>,
    mut grid_locations: Query<&mut GridLocation>,
) {
    let ground_set = {
        let mut tmp = HashSet::new();
        for (_ground, e) in ground.iter() {
            if let Ok(ground_grid_location) = grid_locations.get_mut(e) {
                tmp.insert(ground_grid_location.clone());
            }
        }
        tmp
    };

    dbg!(&ground_set);

    let mut snake_fall = i32::MIN;

    for (_snake, e) in snake_parts.iter() {
        let snake_grid_location = grid_locations.get_mut(e).expect("snake grid location!");
        let mut distance = -1;

        while ground_set
            .get(&GridLocation {
                x: snake_grid_location.x,
                y: snake_grid_location.y + distance,
            })
            .is_none()
        {
            distance -= 1;
            if distance < -50 {
                break;
            }
        }
        snake_fall = snake_fall.max(distance + 1);

        dbg!(snake_fall, distance);
    }

    for (_snake, e) in snake_parts.iter() {
        let mut snake_grid_location = grid_locations.get_mut(e).expect("snake grid location!");
        snake_grid_location.y += snake_fall;
    }

    dbg!(snake_fall);
}

const LERP_LAMBDA: f32 = 5.0;

fn gridlocation_to_transform(time: Res<Time>, mut q: Query<(&GridLocation, &mut Transform)>) {
    // TODO: lerp
    for (grid_location, mut xform) in q.iter_mut() {
        let target_x = GRID_WIDTH * grid_location.x as f32;
        xform.translation.x = xform.translation.x * (1.0 - LERP_LAMBDA * time.delta_seconds())
            + target_x * LERP_LAMBDA * time.delta_seconds();
        let target_y = GRID_HEIGHT * grid_location.y as f32;
        xform.translation.y = xform.translation.y * (1.0 - LERP_LAMBDA * time.delta_seconds())
            + target_y * LERP_LAMBDA * time.delta_seconds();
    }
}
