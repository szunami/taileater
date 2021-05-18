use bevy::prelude::*;
use itertools::Itertools;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SnakeMovement;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct TransformLabel;

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

struct SnakeParts(Vec<Entity>);

const GRID_WIDTH: f32 = 16.0;
const GRID_HEIGHT: f32 = 16.0;

fn main() {
    println!("Hello, world!");
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeParts(vec![]))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_startup_system(setup.system())
        .add_system(snake_movement.system().label(SnakeMovement))
        .add_system(
            gridlocation_to_transform
                .system()
                .label(TransformLabel)
                .after(SnakeMovement),
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

    *snake_parts = SnakeParts(vec![
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .id(),
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                material: head_color.clone_weak(),
                ..Default::default()
            })
            .insert(GridLocation { x: 0, y: 0 })
            .id(),
    ]);

    // TODO: wire up parts later!
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_parts: Res<SnakeParts>,
    mut grid_locations: Query<&mut GridLocation>,
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

fn gridlocation_to_transform(mut q: Query<(&GridLocation, &mut Transform)>) {
    // TODO: lerp
    for (gridlocation, mut xform) in q.iter_mut() {
        xform.translation.x = gridlocation.x as f32 * GRID_WIDTH;
        xform.translation.y = gridlocation.y as f32 * GRID_HEIGHT;
    }
}
