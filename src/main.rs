use bevy::{prelude::*, reflect::TypeRegistry};
use chrono::Local;
use std::{collections::HashSet, path::Path};
use std::{env, fs::File, io::Write};
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct FoodLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct PoisonLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SnakeMovementLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct SpriteLabel;

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
#[derive(Debug, Hash, PartialEq, Eq, Clone, Reflect, Default)]
#[reflect(Component)]
struct GridLocation {
    x: i32,
    y: i32,
}

struct LocationQueue(Vec<GridLocation>);

impl std::ops::Add for GridLocation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Snake;

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Ground;

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Food;

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Poison;

struct SnakeParts(Vec<Entity>);

struct SnakeAssets {
    head: Handle<TextureAtlas>,
    tail: Handle<TextureAtlas>,
    light_body: Handle<TextureAtlas>,
    dark_body: Handle<TextureAtlas>,
}

const GRID_WIDTH: f32 = 32.0;
const GRID_HEIGHT: f32 = 32.0;

struct MainCamera;

struct Cursor;

struct MyWorld(World, TypeRegistry);

#[derive(Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Copy)]
struct Orientation {
    from: Direction,
    to: Direction,
}

#[derive(Clone, Debug)]
struct Transition {
    from: Orientation,
    to: Orientation,
    index: u32,
}

#[derive(Clone, Debug)]
struct TransitionQueue(Vec<Transition>);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    StartMenu,
    InGame,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.last() == Some(&String::from("-l")) {
        App::build()
            .insert_resource(WindowDescriptor {
                title: "EDITOR".to_string(),
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .insert_resource(MyWorld(World::new(), TypeRegistry::default()))
            .register_type::<Ground>()
            .register_type::<GridLocation>()
            .register_type::<Snake>()
            .register_type::<Food>()
            .register_type::<Poison>()
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_startup_system(
                (|world: &mut World| {
                    let real_type_registry = world.get_resource::<TypeRegistry>().unwrap().clone();
                    let mut my_world = world.get_resource_mut::<MyWorld>().unwrap();
                    my_world.1 = real_type_registry;

                    let asset_server = world.get_resource::<AssetServer>().expect("scene spawner");
                    let level = "assets/scenes/drafts/downsizing_2.scn.ron";
                    let scene_handle: Handle<DynamicScene> =
                        asset_server.load(format!("../{}", level).as_str());

                    let mut scene_spawner = world
                        .get_resource_mut::<SceneSpawner>()
                        .expect("scene spawner");

                    scene_spawner.spawn_dynamic(scene_handle);
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
            .add_system(
                (|mut commands: Commands,
                  mut materials: ResMut<Assets<ColorMaterial>>,
                  grounds: Query<(Entity, &GridLocation), (With<Ground>, Without<Sprite>)>,
                  snakes: Query<(Entity, &GridLocation), (With<Snake>, Without<Sprite>)>,
                  foods: Query<(Entity, &GridLocation), (With<Food>, Without<Sprite>)>,
                  poisons: Query<(Entity, &GridLocation), (With<Poison>, Without<Sprite>)>| {
                    for (e, grid_location) in grounds.iter() {
                        commands.entity(e).insert_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                            transform: Transform::from_translation(Vec3::new(
                                grid_location.x as f32 * GRID_WIDTH,
                                grid_location.y as f32 * GRID_HEIGHT,
                                0.
                            )),
                            material: ground_color(&mut materials),
                            ..Default::default()
                        });
                    }
                    
                    for (e, grid_location) in snakes.iter() {
                        commands.entity(e).insert_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                            transform: Transform::from_translation(Vec3::new(
                                grid_location.x as f32 * GRID_WIDTH,
                                grid_location.y as f32 * GRID_HEIGHT,
                                0.
                            )),
                            material: snake_color(&mut materials),
                            ..Default::default()
                        });
                    }
                    
                    for (e, grid_location) in foods.iter() {
                        commands.entity(e).insert_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                            transform: Transform::from_translation(Vec3::new(
                                grid_location.x as f32 * GRID_WIDTH,
                                grid_location.y as f32 * GRID_HEIGHT,
                                0.
                            )),
                            material: food_color(&mut materials),
                            ..Default::default()
                        });
                    }
                    
                    for (e, grid_location) in poisons.iter() {
                        commands.entity(e).insert_bundle(SpriteBundle {
                            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
                            transform: Transform::from_translation(Vec3::new(
                                grid_location.x as f32 * GRID_WIDTH,
                                grid_location.y as f32 * GRID_HEIGHT,
                                0.
                            )),
                            material: poison_color(&mut materials),
                            ..Default::default()
                        });
                    }

                })
                .system(),
            )
            .add_system(editor.system())
            .run();
    } else {
        let mut app = App::build();

        app.insert_resource(WindowDescriptor {
            title: "TAILEATER".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            235. / 255.,
            237. / 255.,
            233. / 255.,
        )));

        #[cfg(target_arch = "wasm32")]
        app.add_plugins(bevy_webgl2::DefaultPlugins);

        #[cfg(target_arch = "x86_64")]
        app.add_plugins(DefaultPlugins);

        // start menu stuff
        app.add_state(GameState::StartMenu)
            .register_type::<Ground>()
            .register_type::<GridLocation>()
            .register_type::<Snake>()
            .register_type::<Food>()
            .register_type::<Poison>()
            .insert_resource(SnakeParts(vec![]))
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_system_set(
                SystemSet::on_enter(GameState::StartMenu).with_system(enter_start_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::StartMenu).with_system(update_start_menu.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::StartMenu).with_system(exit_start_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::StartMenu).with_system(update_start_menu.system()),
            )
            // ingame stuff
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(cleanup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(food.system())
                    .label(FoodLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(poison.system())
                    .label(PoisonLabel)
                    .after(FoodLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(snake_movement.system())
                    .label(SnakeMovementLabel)
                    .after(PoisonLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(sprite.system().label(SpriteLabel).after(SnakeMovementLabel)),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(gravity.system())
                    .label(GravityLabel)
                    .after(SnakeMovementLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(gridlocation_to_transform.system())
                    .label(TransformLabel), // .after(GravityLabel),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(win.system())
                    .label(WinLabel)
                    .after(GravityLabel),
            )
            // gravity
            // gridlocation to transform
            .run();
    }
}

struct Logo;

fn enter_start_menu(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    dbg!("entering start menu!");
    let logo = asset_server.load("sprites/drafts/logo-Sheet.png");
    let logo = TextureAtlas::from_grid(logo, Vec2::new(371.0, 96.0), 5, 1);
    let logo = texture_atlases.add(logo);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: logo.clone(),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Logo);
}

fn update_start_menu(
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,

    mut q: Query<(&mut TextureAtlasSprite, &mut Timer), With<Logo>>,
) {
    for (mut sprite, mut timer) in q.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % 5;
        }
    }

    if time.seconds_since_startup() > 3. {
        state.set(GameState::InGame);
    }
}

fn exit_start_menu(mut commands: Commands, mut q: Query<Entity, With<Logo>>) {
    for e in q.iter_mut() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let light_body = asset_server.load("sprites/drafts/light_worksheet.png");
    let light_body = TextureAtlas::from_grid(light_body, Vec2::new(96.0, 96.0), 17, 36);
    let light_body = texture_atlases.add(light_body);

    let dark_body = asset_server.load("sprites/drafts/dark_worksheet.png");
    let dark_body = TextureAtlas::from_grid(dark_body, Vec2::new(96.0, 96.0), 17, 36);
    let dark_body = texture_atlases.add(dark_body);

    let head = asset_server.load("sprites/drafts/head_thick.png");
    let head = TextureAtlas::from_grid(head, Vec2::new(32.0, 32.0), 4, 1);
    let head = texture_atlases.add(head);

    let tail = asset_server.load("sprites/drafts/tail_thick.png");
    let tail = TextureAtlas::from_grid(tail, Vec2::new(32.0, 32.0), 4, 1);
    let tail = texture_atlases.add(tail);

    commands.insert_resource(SnakeAssets {
        head,
        tail,
        light_body,
        dark_body,
    });

    let args: Vec<String> = env::args().collect();
    if let Some(level) = args.last() {
        let scene_handle: Handle<DynamicScene> =
            asset_server.load(format!("../{}", level).as_str());
        scene_spawner.spawn_dynamic(scene_handle);
    } else {
        eprintln!("No level provided.");
    }
}

// spawning scenes is async, we don't have a good callback yet
fn cleanup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_parts: ResMut<SnakeParts>,

    snake_assets: Res<SnakeAssets>,

    grounds: Query<(&Ground, &GridLocation, Entity), Without<Sprite>>,
    snakes: Query<(&Snake, &GridLocation, Entity), Without<TextureAtlasSprite>>,
    foods: Query<(&Food, &GridLocation, Entity), Without<Sprite>>,
    poisons: Query<(&Poison, &GridLocation, Entity), Without<Sprite>>,
) {
    for (_ground, grid_location, e) in grounds.iter() {
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

    for (_food, grid_location, e) in foods.iter() {
        commands.entity(e).insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            material: food_color(&mut materials),
            transform: Transform::from_translation(Vec3::new(
                grid_location.x as f32 * GRID_WIDTH,
                grid_location.y as f32 * GRID_HEIGHT,
                0.,
            )),
            ..Default::default()
        });
    }

    for (_poison, grid_location, e) in poisons.iter() {
        commands.entity(e).insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(GRID_WIDTH, GRID_HEIGHT)),
            material: poison_color(&mut materials),
            transform: Transform::from_translation(Vec3::new(
                grid_location.x as f32 * GRID_WIDTH,
                grid_location.y as f32 * GRID_HEIGHT,
                0.,
            )),
            ..Default::default()
        });
    }

    // HACK: assume that the rightmost snake is first in the array
    // TODO: use MapEntities
    let mut internal_snake_parts = vec![];
    let mut max_x = None;
    for (_snake, grid_location, e) in snakes.iter() {
        let id = commands
            .entity(e)
            .insert(LocationQueue(vec![]))
            .insert(TransitionQueue(vec![]))
            .insert(Orientation {
                to: Direction::Right,
                from: Direction::Left,
            })
            .id();
        match max_x {
            Some(max_x) => {
                if grid_location.x > max_x {
                    internal_snake_parts.insert(0, id)
                } else {
                    internal_snake_parts.push(id);
                }
            }
            None => {
                internal_snake_parts.push(id);
                max_x = Some(grid_location.x)
            }
        }
    }
    if !internal_snake_parts.is_empty() {
        let head_entity = *internal_snake_parts.first().expect("head exists");
        let (_snake, head_grid_location, _e) = snakes.get(head_entity).expect("head lookup");

        // TODO: assign sprites here now that we have order. Do something smarter!
        commands
            .entity(head_entity)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: snake_assets.head.clone(),
                transform: Transform::from_translation(Vec3::new(
                    head_grid_location.x as f32 * GRID_WIDTH,
                    head_grid_location.y as f32 * GRID_HEIGHT,
                    0.,
                )),
                ..Default::default()
            });

        let tail_entity = *internal_snake_parts.last().expect("tail exists");
        let (_snake, tail_grid_location, _e) = snakes.get(tail_entity).expect("tail lookup");

        // TODO: assign sprites here now that we have order. Do something smarter!
        commands
            .entity(tail_entity)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: snake_assets.tail.clone(),
                transform: Transform::from_translation(Vec3::new(
                    tail_grid_location.x as f32 * GRID_WIDTH,
                    tail_grid_location.y as f32 * GRID_HEIGHT,
                    0.,
                )),
                ..Default::default()
            });

        *snake_parts = SnakeParts(internal_snake_parts);
    }
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_parts: Res<SnakeParts>,
    grounds: Query<&GridLocation, (With<Ground>, Without<Snake>)>,

    mut snakes: Query<
        (&mut GridLocation, &mut LocationQueue, &mut Orientation),
        (With<Snake>, Without<Ground>),
    >,

    mut transitions: Query<&mut TransitionQueue, With<Snake>>,
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

    let block_set = {
        let mut tmp = HashSet::new();

        if snake_parts.0.len() > 3 {
            // exclude head; exclude tail + second to last
            for snake_part_entity in snake_parts.0[1..snake_parts.0.len() - 2].iter() {
                tmp.insert(
                    snakes
                        .get_mut(*snake_part_entity)
                        .expect("snake part grid location")
                        .0
                        .clone(),
                );
            }
        } else if snake_parts.0.len() == 2 {
            tmp.insert(
                snakes
                    .get_mut(*snake_parts.0.last().expect("tail exists"))
                    .expect("snake part lookup")
                    .0
                    .clone(),
            );
        } else if snake_parts.0.len() == 3 {
            tmp.insert(
                snakes
                    .get_mut(*snake_parts.0.get(1).expect("middle exists"))
                    .expect("snake part lookup")
                    .0
                    .clone(),
            );
            tmp.insert(
                snakes
                    .get_mut(*snake_parts.0.last().expect("tail exists"))
                    .expect("snake part lookup")
                    .0
                    .clone(),
            );
        }

        for ground in grounds.iter() {
            tmp.insert(ground.clone());
        }

        tmp
    };

    let (head_location, _queue, _orientation) = snakes
        .get_mut(*snake_parts.0.first().expect("head exists!"))
        .expect("head location exists");
    let proposed_location = head_location.clone() + diff.clone();

    if block_set.contains(&proposed_location) {
        dbg!("blocked; not moving!");
        return;
    }

    for (prev, curr) in snake_parts.0.iter().zip(snake_parts.0[1..].iter()).rev() {
        if let Ok((prev_grid_location, _prev_queue, _orientation)) = snakes.get_mut(*prev) {
            let tmp = prev_grid_location.clone();
            if let Ok((mut curr_grid_location, mut curr_queue, _orientation)) =
                snakes.get_mut(*curr)
            {
                *curr_grid_location = tmp.clone();
                curr_queue.0.push(tmp);
            }
        }
    }

    if let Some(head) = snake_parts.0.first() {
        if let Ok((mut grid_location, mut queue, _orientation)) = snakes.get_mut(*head) {
            *grid_location = grid_location.clone() + diff.clone();
            queue.0.push(grid_location.clone());
        }
    }

    if snake_parts.0.len() > 1 {
        for (prev, curr) in snake_parts.0[1..]
            .iter()
            .zip(snake_parts.0[2..].iter())
            .rev()
        {
            if let Ok((_prev_grid_location, _prev_queue, prev_orientation)) = snakes.get_mut(*prev)
            {
                let tmp = prev_orientation.clone();

                if let Ok((_curr_grid_location, _curr_queue, mut curr_orientation)) =
                    snakes.get_mut(*curr)
                {
                    if let Ok(mut transition_queue) = transitions.get_mut(*curr) {
                        transition_queue.0.push(Transition {
                            from: curr_orientation.clone(),
                            to: tmp.clone(),
                            index: 0,
                        });
                    }

                    *curr_orientation = tmp;
                }
            }
        }
    }

    if let Some(second) = snake_parts.0.get(1) {
        if let Ok((_grid_location, _queue, mut orientation)) = snakes.get_mut(*second) {
            // handle transition here?

            let old_orientation = orientation.clone();

            orientation.from = match orientation.to.clone() {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };

            if keyboard_input.just_pressed(KeyCode::A) {
                orientation.to = Direction::Left;
            }
            if keyboard_input.just_pressed(KeyCode::D) {
                orientation.to = Direction::Right;
            }
            if keyboard_input.just_pressed(KeyCode::S) {
                orientation.to = Direction::Down;
            }
            if keyboard_input.just_pressed(KeyCode::W) {
                orientation.to = Direction::Up;
            }

            let new_orientation = orientation.clone();

            if let Ok(mut transition_queue) = transitions.get_mut(*second) {
                transition_queue.0.push(Transition {
                    from: old_orientation,
                    to: new_orientation,
                    index: 0,
                });
            }
        }
    }

    if let Some(head) = snake_parts.0.first() {
        if let Ok((_grid_location, _queue, mut orientation)) = snakes.get_mut(*head) {
            orientation.from = orientation.to.clone();

            if keyboard_input.just_pressed(KeyCode::A) {
                orientation.to = Direction::Left;
                orientation.from = Direction::Right;
            }
            if keyboard_input.just_pressed(KeyCode::D) {
                orientation.to = Direction::Right;
                orientation.from = Direction::Left;
            }
            if keyboard_input.just_pressed(KeyCode::S) {
                orientation.to = Direction::Down;
                orientation.from = Direction::Up;
            }
            if keyboard_input.just_pressed(KeyCode::W) {
                orientation.to = Direction::Up;
                orientation.from = Direction::Down;
            }
        }
    }
}

// TODO: use With<Snake>
fn gravity(
    snake_parts: Query<(&Snake, Entity)>,
    ground: Query<(&Ground, Entity)>,
    mut grid_locations: Query<&mut GridLocation>,
    mut queues: Query<&mut LocationQueue>,
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

    if snake_fall == 0 {
        return;
    }

    for (_snake, e) in snake_parts.iter() {
        let mut snake_grid_location = grid_locations.get_mut(e).expect("snake grid location!");
        snake_grid_location.y += snake_fall;

        match queues.get_mut(e) {
            Ok(mut queue) => {
                queue.0.push(snake_grid_location.clone());
            }
            Err(_) => {
                eprintln!("Scene not loaded?")
            }
        }
    }
}

fn food(
    mut commands: Commands,

    mut snake_parts: ResMut<SnakeParts>,

    assets: Res<SnakeAssets>,

    snake_locations: Query<(&GridLocation, &Transform, &Orientation), (With<Snake>, Without<Food>)>,
    food_locations: Query<(&GridLocation, Entity), (With<Food>, Without<Snake>)>,
) {
    if snake_parts.0.is_empty() {
        return;
    }

    let head_location = match snake_locations.get(*snake_parts.0.first().expect("head exists")) {
        Ok(x) => x.0,
        Err(_) => return,
    };

    let (tail_location, tail_xform, tail_orientation) = snake_locations
        .get(*snake_parts.0.last().expect("tail exists"))
        .expect("tail has grid location");

    for (food_location, food_entity) in food_locations.iter() {
        if food_location == head_location {
            // despawn food!
            commands.entity(food_entity).despawn_recursive();

            let texture_atlas = {
                if snake_parts.0.len() % 2 == 0 {
                    assets.dark_body.clone()
                } else {
                    assets.light_body.clone()
                }
            };

            let new_snake = commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas,
                    transform: *tail_xform,
                    ..Default::default()
                })
                .insert(tail_location.clone())
                // transforms only update is queue is nonempty...
                .insert(LocationQueue(vec![tail_location.clone()]))
                .insert(TransitionQueue(vec![]))
                .insert(Snake)
                // what is orientation??
                .insert(tail_orientation.clone())
                .id();

            let index = match snake_parts.0.len() {
                1 => 1,
                _ => snake_parts.0.len() - 1,
            };

            snake_parts.0.insert(index, new_snake);
        }
    }
}

fn poison(
    mut commands: Commands,

    mut snake_parts: ResMut<SnakeParts>,

    snake_locations: Query<
        (&GridLocation, &Transform, &Orientation),
        (With<Snake>, Without<Poison>),
    >,
    poison_locations: Query<(&GridLocation, Entity), (With<Poison>, Without<Snake>)>,
) {
    if snake_parts.0.is_empty() {
        return;
    }

    let head_location = match snake_locations.get(*snake_parts.0.first().expect("head exists")) {
        Ok(x) => x.0,
        Err(_) => return,
    };

    for (poison_location, poison_entity) in poison_locations.iter() {
        if poison_location == head_location {
            // despawn poison!
            commands.entity(poison_entity).despawn_recursive();

            let to_despawn_index = match snake_parts.0.len() {
                1 => 0,
                2 => 1,
                _ => snake_parts.0.len() - 2,
            };
            let to_despawn = snake_parts.0.remove(to_despawn_index);

            dbg!("despawning", to_despawn_index);

            commands.entity(to_despawn).despawn_recursive();

            // TODO: update transitions / locations?
        }
    }
}

const RATE: f32 = 2.0;

fn gridlocation_to_transform(mut q: Query<(&mut LocationQueue, &mut Transform)>) {
    for (mut location_queue, mut xform) in q.iter_mut() {
        // if item in location queue, move to it.
        // if arrive there, remove it.

        if let Some(grid_location) = location_queue.0.first() {
            let target_x = GRID_WIDTH * grid_location.x as f32;
            let dx = target_x - xform.translation.x;
            if dx.abs() > f32::EPSILON {
                xform.translation.x += RATE * dx.signum();
            }

            let target_y = GRID_HEIGHT * grid_location.y as f32;
            let dy = target_y - xform.translation.y;
            if dy.abs() > f32::EPSILON {
                xform.translation.y += RATE * dy.signum();
            }

            if xform
                .translation
                .distance(Vec3::new(target_x, target_y, 0.))
                <= f32::EPSILON
            {
                location_queue.0.remove(0);
            }
        }
    }
}

fn win(snake_parts: Res<SnakeParts>, snake_locations: Query<&GridLocation, With<Snake>>) {
    if snake_parts.0.len() <= 2 {
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

fn editor(
    mut commands: Commands,

    wnds: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut my_world: ResMut<MyWorld>,

    camera: Query<&Transform, (With<MainCamera>, Without<Cursor>)>,
    mut cursors: Query<&mut Transform, (With<Cursor>, Without<MainCamera>)>,
    grid_locations: Query<(&GridLocation, Entity), Without<Cursor>>,
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
        let camera_transform = camera.single().unwrap();

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

        if keyboard_input.pressed(KeyCode::G) {
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

            my_world
                .0
                .spawn()
                .insert(mouse_grid_location.clone())
                .insert(Ground);
        }

        if keyboard_input.pressed(KeyCode::S) {
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
                    material: snake_color(&mut materials),
                    transform: mouse_xform,
                    ..Default::default()
                })
                .insert(mouse_grid_location.clone())
                .insert(Snake);

            my_world
                .0
                .spawn()
                .insert(mouse_grid_location.clone())
                .insert(Snake);
        }

        if keyboard_input.pressed(KeyCode::F) {
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
                    material: food_color(&mut materials),
                    transform: mouse_xform,
                    ..Default::default()
                })
                .insert(mouse_grid_location.clone())
                .insert(Food);

            my_world
                .0
                .spawn()
                .insert(mouse_grid_location.clone())
                .insert(Food);
        }

        if keyboard_input.pressed(KeyCode::P) {
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
                    material: poison_color(&mut materials),
                    transform: mouse_xform,
                    ..Default::default()
                })
                .insert(mouse_grid_location.clone())
                .insert(Poison);

            my_world
                .0
                .spawn()
                .insert(mouse_grid_location.clone())
                .insert(Poison);
        }

        if keyboard_input.pressed(KeyCode::D) {
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

fn ground_color(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(173.0 / 255.0, 119.0 / 255.0, 87.0 / 255.0).into())
}
fn snake_color(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(117.0 / 255.0, 167.0 / 255.0, 67.0 / 255.0).into())
}

fn cursor_color(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(164.0 / 255.0, 221.0 / 255.0, 219.0 / 255.0).into())
}

fn food_color(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(165.0 / 255.0, 48.0 / 255.0, 48.0 / 255.0).into())
}

fn poison_color(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
    materials.add(Color::rgb(122.0 / 255.0, 54.0 / 255.0, 123.0 / 255.0).into())
}

// update sprite based on each direction
fn sprite(
    snake_parts: Res<SnakeParts>,

    // TODO: shouldn't need both of these once everything has a transition (???)
    mut orientation_query: Query<(&Orientation, &mut TextureAtlasSprite, &mut TransitionQueue)>,
) {
    for (index, e) in snake_parts.0.iter().enumerate() {
        let tail = snake_parts.0.len() - 1;
        match index {
            // HEAD
            0 => match orientation_query.get_mut(*e) {
                Ok((orientation, mut sprite, _queue)) => {
                    sprite.index = match &orientation.to {
                        Direction::Up => 1,
                        Direction::Down => 3,
                        Direction::Left => 2,
                        Direction::Right => 0,
                    }
                }
                Err(_) => {
                    dbg!("Someone should look into this...");
                }
            },
            // TAIL
            x if x == tail => match orientation_query.get_mut(*e) {
                Ok((orientation, mut sprite, _queue)) => {
                    sprite.index = match &orientation.to {
                        Direction::Up => 3,
                        Direction::Down => 1,
                        Direction::Left => 0,
                        Direction::Right => 2,
                    }
                }
                Err(_) => {
                    dbg!("Someone should look into this...");
                }
            },
            // TWEENER
            _ => match orientation_query.get_mut(*e) {
                // increment transition; match transition from and to to determine offset
                // offset depends on transition values;
                Ok((_orientation, mut sprite, mut transition_queue)) => {
                    if let Some(transition) = transition_queue.0.first_mut() {
                        // TODO: this is a function of transition to / from

                        if index == 1 {
                            dbg!(transition.clone());
                        }

                        let offset = match (transition.from, transition.to) {
                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                            ) => 0,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                            ) => 1,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                            ) => 2,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                            ) => 3,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                            ) => 4,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                            ) => 5,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                            ) => 6,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                            ) => 7,

                            (
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                            ) => 8,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                            ) => 9,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                            ) => 10,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                            ) => 11,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                            ) => 12,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                            ) => 13,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                            ) => 14,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                            ) => 15,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                            ) => 16,

                            (
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                            ) => 17,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                            ) => 18,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                            ) => 19,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Up,
                                },
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                            ) => 20,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                            ) => 21,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                            ) => 22,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                            ) => 23,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                            ) => 24,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                            ) => 25,

                            (
                                Orientation {
                                    from: Direction::Down,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                            ) => 26,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                            ) => 27,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                            ) => 28,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Down,
                                },
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                            ) => 29,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Up,
                                },
                            ) => 30,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Left,
                                },
                            ) => 31,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Left,
                                },
                                Orientation {
                                    from: Direction::Right,
                                    to: Direction::Down,
                                },
                            ) => 32,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Up,
                                },
                            ) => 33,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Right,
                                },
                            ) => 34,

                            (
                                Orientation {
                                    from: Direction::Up,
                                    to: Direction::Right,
                                },
                                Orientation {
                                    from: Direction::Left,
                                    to: Direction::Down,
                                },
                            ) => 35,

                            _ => {
                                dbg!("missed one");
                                dbg!(transition.clone());
                                1000
                            }
                        };
                        transition.index = (transition.index + 1).min(16);

                        sprite.index = offset * 17 + transition.index;

                        if transition.index == 16 {
                            transition_queue.0.remove(0);
                        }
                    }
                }
                Err(_) => {
                    dbg!("Someone should look into this...");
                }
            },
        }
    }
}
