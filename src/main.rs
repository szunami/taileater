use bevy::{
    prelude::*,
    reflect::TypeRegistry,
    render::{camera::Camera, pipeline::IndexFormat},
};
use chrono::Local;
use std::{collections::HashSet, path::Path, time::Duration};
use std::{env, fs::File, io::Write, time::SystemTime};
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct FoodLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SnakeMovementLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct TransformLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct GravityLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct WinLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SetupLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct CleanupLabel;
#[reflect(Component)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Reflect, Default)]
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

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Ground;

struct Food;

struct SnakeParts(Vec<Entity>);

const GRID_WIDTH: f32 = 16.0;
const GRID_HEIGHT: f32 = 16.0;

struct MainCamera;

struct Cursor;

struct MyWorld(World, TypeRegistry);

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.last() == Some(&String::from("-l")) {
        App::build()
            .add_plugins(DefaultPlugins)
            .insert_resource(MyWorld(World::new(), TypeRegistry::default()))
            .register_type::<Ground>()
            .register_type::<GridLocation>()
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_startup_system(
                (|world: &mut World| {
                    let real_type_registry = world.get_resource::<TypeRegistry>().unwrap().clone();
                    let mut my_world = world.get_resource_mut::<MyWorld>().unwrap();
                    my_world.1 = real_type_registry;
                })
                .exclusive_system(),
            )
            .add_startup_system(
                (|mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>| {
                    commands
                        .spawn()
                        .insert_bundle(OrthographicCameraBundle::new_2d())
                        .insert(MainCamera);
                    commands.spawn_bundle(UiCameraBundle::default());

                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                            material: cursor_color(&mut materials),
                            ..Default::default()
                        })
                        .insert(GridLocation { x: 0, y: 0 })
                        .insert(Cursor)
                        .id();
                })
                .system(),
            )
            .add_system(my_cursor_system.system())
            .run();
    } else {
        App::build()
            .add_plugins(DefaultPlugins)
            .register_type::<core::option::Option<IndexFormat>>()
            .register_type::<Ground>()
            .register_type::<GridLocation>()
            .insert_resource(SnakeParts(vec![]))
            .add_startup_system(setup.system().label(SetupLabel))
            .add_system(cleanup.system().label(CleanupLabel).after(SetupLabel))
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_system(
                (|q: Query<(&Ground, &Transform)>| {
                    for (_g, xform) in q.iter() {
                        dbg!(xform);
                    }
                })
                .system(),
            )
            // .add_system(food.system().label(FoodLabel))
            // .add_system(
            //     snake_movement
            //         .system()
            //         .label(SnakeMovementLabel)
            //         .after(FoodLabel),
            // )
            // .add_system(
            //     gravity
            //         .system()
            //         .label(GravityLabel)
            //         .after(SnakeMovementLabel),
            // )
            .add_system(
                gridlocation_to_transform.system().label(TransformLabel), // .after(GravityLabel),
            )
            // .add_system(win.system().label(WinLabel).after(GravityLabel))
            // gravity
            // gridlocation to transform
            .run();
    }
}

fn setup(
    mut commands: Commands,
    mut snake_parts: ResMut<SnakeParts>,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut scene_spawner: ResMut<SceneSpawner>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Scenes are loaded just like any other asset.
    
    let args: Vec<String> = env::args().collect();
    let level = args.last().expect("Provide a filename!");

    let scene_handle: Handle<DynamicScene> = asset_server.load(format!("../{}", level).as_str());

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in
    // the World with new entity ids. This guarantees that it will not overwrite existing
    // entities.
    scene_spawner.spawn_dynamic(scene_handle);

    // This tells the AssetServer to watch for changes to assets.
    // It enables our scenes to automatically reload in game when we modify their files
    // asset_server.watch_for_changes().unwrap();

    //     let head_color = materials.add(Color::rgb(168.0 / 255.0, 202.0 / 255.0, 88.0 / 255.0).into());
    //     let body_color = materials.add(Color::rgb(117.0 / 255.0, 167.0 / 255.0, 67.0 / 255.0).into());
    //     let tail_color = materials.add(Color::rgb(232.0 / 255.0, 193.0 / 255.0, 112.0 / 255.0).into());

    //     let food_color = materials.add(Color::rgb(165.0 / 255.0, 48.0 / 255.0, 48.0 / 255.0).into());

    //     *snake_parts = SnakeParts(vec![
    //         commands
    //             .spawn()
    //             .insert_bundle(SpriteBundle {
    //                 sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //                 material: head_color.clone_weak(),
    //                 ..Default::default()
    //             })
    //             .insert(GridLocation { x: 0, y: 0 })
    //             .insert(Snake)
    //             .id(),
    //         commands
    //             .spawn()
    //             .insert_bundle(SpriteBundle {
    //                 sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //                 material: tail_color.clone_weak(),
    //                 ..Default::default()
    //             })
    //             .insert(GridLocation { x: 0, y: 0 })
    //             .insert(Snake)
    //             .id(),
    //     ]);

    //     for i in -10..10 {
    //         commands
    //             .spawn()
    //             .insert_bundle(SpriteBundle {
    //                 sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //                 material: ground_color(&mut materials),
    //                 ..Default::default()
    //             })
    //             .insert(GridLocation { x: i, y: -1 })
    //             .insert(Ground)
    //             .id();
    //     }

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: ground_color(&mut materials),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: 5, y: 0 })
    //         .insert(Ground)
    //         .id();

    //     for i in 13..20 {
    //         commands
    //             .spawn()
    //             .insert_bundle(SpriteBundle {
    //                 sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //                 material: ground_color(&mut materials),
    //                 ..Default::default()
    //             })
    //             .insert(GridLocation { x: i, y: -1 })
    //             .insert(Ground)
    //             .id();
    //     }

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: ground_color(&mut materials),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: 19, y: 4 })
    //         .insert(Ground)
    //         .id();

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: ground_color(&mut materials),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: 18, y: 8 })
    //         .insert(Ground)
    //         .id();

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: food_color.clone_weak(),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: -5, y: 0 })
    //         .insert(Food)
    //         .id();

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: food_color.clone_weak(),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: -7, y: 0 })
    //         .insert(Food)
    //         .id();

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: food_color.clone_weak(),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: -9, y: 0 })
    //         .insert(Food)
    //         .id();

    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
    //             material: food_color.clone_weak(),
    //             ..Default::default()
    //         })
    //         .insert(GridLocation { x: -8, y: 0 })
    //         .insert(Food)
    //         .id();
}

// spawning scenes is async, we don't have a good callback yet
fn cleanup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,

    q: Query<(&Ground, &GridLocation, Entity), Without<Sprite>>,
) {
    for (_ground, grid_location, e) in q.iter() {
        commands.entity(e).insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            material: ground_color(&mut materials),
            transform: Transform::from_translation(Vec3::new(
                grid_location.x as f32 * GRID_WIDTH,
                grid_location.y as f32 * GRID_HEIGHT,
                0.,
            )),
            ..Default::default()
        });
    }
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_parts: Res<SnakeParts>,
    grounds: Query<&GridLocation, (With<Ground>, Without<Snake>)>,

    mut snakes: Query<&mut GridLocation, (With<Snake>, Without<Ground>)>,
) {
    // TODO: don't allow x and y at the same damn time
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
    // block if collision between head and:
    // - non tail
    // - non second to last!
    // - ground
    // - someday: box + wall (???)

    let block_set = {
        let mut tmp = HashSet::new();

        if snake_parts.0.len() > 2 {
            // exclude head; exclude tail + second to last
            for snake_part_entity in snake_parts.0[1..snake_parts.0.len() - 2].iter() {
                tmp.insert(
                    snakes
                        .get_mut(*snake_part_entity)
                        .expect("snake part grid location")
                        .clone(),
                );
            }
        } else if snake_parts.0.len() == 2 {
            tmp.insert(
                snakes
                    .get_mut(*snake_parts.0.last().expect("tail exists"))
                    .expect("snake part lookup")
                    .clone(),
            );
        }

        for ground in grounds.iter() {
            tmp.insert(ground.clone());
        }

        tmp
    };

    let head_location = snakes
        .get_mut(*snake_parts.0.first().expect("head exists!"))
        .expect("head location exists");
    let proposed_location = head_location.clone() + diff.clone();

    if block_set.contains(&proposed_location) {
        dbg!("blocked; not moving!");
        return;
    }

    for (prev, curr) in snake_parts.0.iter().zip(snake_parts.0[1..].iter()).rev() {
        if let Ok(prev_grid_location) = snakes.get_mut(*prev) {
            let tmp = prev_grid_location.clone();
            if let Ok(mut curr_grid_location) = snakes.get_mut(*curr) {
                *curr_grid_location = tmp;
            }
        }
    }

    if let Some(head) = snake_parts.0.first() {
        if let Ok(mut grid_location) = snakes.get_mut(*head) {
            *grid_location = grid_location.clone() + diff;
        }
    }
}

// TODO: use With<Snake>
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
    }

    for (_snake, e) in snake_parts.iter() {
        let mut snake_grid_location = grid_locations.get_mut(e).expect("snake grid location!");
        snake_grid_location.y += snake_fall;
    }
}

fn food(
    mut commands: Commands,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_parts: ResMut<SnakeParts>,

    snake_locations: Query<(&GridLocation, &Transform), (With<Snake>, Without<Food>)>,
    food_locations: Query<(&GridLocation, Entity), (With<Food>, Without<Snake>)>,
) {
    let head_location = snake_locations
        .get(*snake_parts.0.first().expect("head exists"))
        .expect("head has grid location")
        .0;

    let (tail_location, tail_xform) = snake_locations
        .get(*snake_parts.0.last().expect("tail exists"))
        .expect("head has grid location");

    for (food_location, food_entity) in food_locations.iter() {
        if food_location == head_location {
            // despawn food!
            commands.entity(food_entity).despawn_recursive();

            let body_color =
                materials.add(Color::rgb(117.0 / 255.0, 167.0 / 255.0, 67.0 / 255.0).into());

            let new_snake = commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                    material: body_color.clone_weak(),
                    transform: *tail_xform,
                    ..Default::default()
                })
                .insert(tail_location.clone())
                .insert(Snake)
                .id();

            let index = snake_parts.0.len() - 1;

            snake_parts.0.insert(index, new_snake);
        }
    }
}

const LERP_LAMBDA: f32 = 5.0;

fn gridlocation_to_transform(time: Res<Time>, mut q: Query<(&GridLocation, &mut Transform)>) {
    // TODO: queue of locations
    for (grid_location, mut xform) in q.iter_mut() {
        let target_x = GRID_WIDTH * grid_location.x as f32;
        xform.translation.x = xform.translation.x * (1.0 - LERP_LAMBDA * time.delta_seconds())
            + target_x * LERP_LAMBDA * time.delta_seconds();
        let target_y = GRID_HEIGHT * grid_location.y as f32;
        xform.translation.y = xform.translation.y * (1.0 - LERP_LAMBDA * time.delta_seconds())
            + target_y * LERP_LAMBDA * time.delta_seconds();
    }
}

fn win(snake_parts: Res<SnakeParts>, snake_locations: Query<&GridLocation, With<Snake>>) {
    if snake_parts.0.len() == 2 {
        return;
    }

    let head_location = snake_locations
        .get(*snake_parts.0.first().expect("snake head exists"))
        .expect("head has location");

    let tail_location = snake_locations
        .get(*snake_parts.0.last().expect("snake tail exists"))
        .expect("tail has location");

    if head_location == tail_location {
        println!("You won! Nice.");
    }
}

fn my_cursor_system(
    mut commands: Commands,
    // need to get window dimensions
    wnds: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut my_world: ResMut<MyWorld>,

    // query to get camera transform
    q_camera: Query<&Transform, (With<MainCamera>, Without<Cursor>)>,
    mut cursors: Query<&mut Transform, (With<Cursor>, Without<MainCamera>)>,

    grid_locations: Query<(&GridLocation, Entity)>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        // println!("World coords: ({}, {})", pos_wld.x, pos_wld.y);
        // println!(
        //     "To grid: ({}, {})",
        //     (pos_wld.x / GRID_WIDTH) as i32,
        //     (pos_wld.y / GRID_HEIGHT) as i32
        // );

        let mouse_grid_location = GridLocation {
            x: (pos_wld.x / GRID_WIDTH) as i32,
            y: (pos_wld.y / GRID_HEIGHT) as i32,
        };
        let mouse_xform = Transform::from_translation(Vec3::new(
            mouse_grid_location.x as f32 * GRID_WIDTH,
            mouse_grid_location.y as f32 * GRID_HEIGHT,
            0.,
        ));

        for mut cursor in cursors.iter_mut() {
            *cursor = mouse_xform;
        }

        if keyboard_input.just_pressed(KeyCode::G) {
            let mut q = my_world.0.query::<(&GridLocation, Entity)>();

            let mut to_despawn = vec![];
            for (grid_location, e) in q.iter(&my_world.0) {
                if *grid_location == mouse_grid_location {
                    to_despawn.push(e);
                }
            }
            for e in to_despawn {
                my_world.0.despawn(e);
            }

            for (grid_location, e) in grid_locations.iter() {
                if *grid_location == mouse_grid_location {
                    commands.entity(e).despawn_recursive();
                }
            }
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                    material: ground_color(&mut materials),
                    transform: mouse_xform,
                    ..Default::default()
                })
                .insert(mouse_grid_location.clone())
                .insert(Ground);

            let id = my_world
                .0
                .spawn()
                .insert(mouse_grid_location)
                .insert(Ground)
                .id();
        }

        if keyboard_input.just_pressed(KeyCode::E) {
            let filename = format!(
                "assets/scenes/tmp/{}.scn.ron",
                Local::now().format("%Y%m%d_%H:%M:%S")
            );
            let path = Path::new(&filename);
            let scene = DynamicScene::from_world(&my_world.0, &my_world.1);
            let data = scene.serialize_ron(&my_world.1).unwrap();
            // Open a file in write-only mode, returns `io::Result<File>`
            match File::create(&path) {
                Err(why) => eprintln!("couldn't save to {}: {}", path.display(), why),
                Ok(mut file) => {
                    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
                    match file.write_all(data.as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
                        Ok(_) => println!("Successfully wrote to {}", path.display()),
                    }
                }
            };
        }
    }
}

fn ground_color(mut materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(173.0 / 255.0, 119.0 / 255.0, 87.0 / 255.0).into())
}

fn cursor_color(mut materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(164.0 / 255.0, 221.0 / 255.0, 219.0 / 255.0).into())
}
