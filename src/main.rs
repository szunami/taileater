use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SnakeMovement;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct TransformLabel;

struct Head;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct GridLocation {
    x: i32,
    y: i32,
}

const GRID_WIDTH: f32 = 16.0;
const GRID_HEIGHT: f32 = 16.0;

fn main() {
    println!("Hello, world!");
    App::build()
        .add_plugins(DefaultPlugins)
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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let head_color = materials.add(Color::rgb(117.0 / 255.0, 167.0 / 255.0, 67.0 / 255.0).into());

    // let body_color =  #a8ca58

    let body = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            material: head_color,
            // transform
            transform: Transform::default(),
            ..Default::default()
        })
        .insert(Head)
        .insert(GridLocation { x: 0, y: 0 });

    // TODO: wire up parts later!
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head: Query<(&Head, &mut GridLocation)>,
) {
    for (_head, mut grid_location) in head.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::A) {
            grid_location.x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            grid_location.x += 1;
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            grid_location.y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            grid_location.y += 1;
        }
    }
}

fn gridlocation_to_transform(mut q: Query<(&GridLocation, &mut Transform)>) {
    // TODO: lerp
    for (gridlocation, mut xform) in q.iter_mut() {
        dbg!(gridlocation);
        xform.translation.x = gridlocation.x as f32 * GRID_WIDTH;
        xform.translation.y = gridlocation.y as f32 * GRID_HEIGHT;
    }
}
