use crate::app_states::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bitflags::bitflags;
use heron::*;
use rand::Rng;

// Components

#[derive(PartialEq, Default)]
pub enum CubeType {
    #[default]
    Environment,
    Health,
    Speed,
    Brake,
}

#[derive(Component, Default)]
pub struct Cube {
    cube_type: CubeType,
}

#[derive(Component, Default)]
pub struct Teleport;

#[derive(Component, Default)]
pub struct Floor {
    direction: u8,
    _rotating: bool,
}

#[derive(Component, Default)]
pub struct FallingGameComponent;

#[derive(Component, Default)]
pub struct Interface;

#[derive(Component, Default)]
pub struct VelocityText;

#[derive(Component, Default)]
pub struct HealthText;

#[derive(Component, Default)]
pub struct DistanceText;

// Bundles

#[derive(Bundle, Default)]
struct CubeBundle {
    #[bundle]
    pbr_bundle: PbrBundle,
    cube: Cube,
    marker: FallingGameComponent,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
}

#[derive(Bundle, Default)]
struct TeleportBundle {
    #[bundle]
    pbr_bundle: PbrBundle,
    teleport: Teleport,
    marker: FallingGameComponent,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
}

#[derive(Component, Clone)]
pub(crate) struct Actor {
    health: f32,
}

#[derive(Bundle)]
pub(crate) struct ActorBundle {
    // collision_layers_world: CollisionLayers,
    collision_layers_teleport: CollisionLayers,
    collision_shape: CollisionShape,
    global_transform: GlobalTransform,
    actor: Actor,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    transform: Transform,
    velocity: Velocity,
    physics_material: PhysicMaterial,
    marker: FallingGameComponent,
}

fn new_actor_bundle() -> ActorBundle {
    return ActorBundle {
        transform: Transform {
            translation: Vec3::new(0.0 as f32, 3050.0, 0.0),
            ..default()
        },
        global_transform: GlobalTransform::identity(),
        velocity: Velocity::from_linear(Vec3::ZERO),
        collision_shape: CollisionShape::Sphere { radius: 0.5 },
        rigid_body: RigidBody::Dynamic,
        physics_material: PhysicMaterial {
            density: 200.0,
            ..Default::default()
        },
        // collision_layers_world: CollisionLayers::new(Layer::Player, Layer::World),
        collision_layers_teleport: CollisionLayers::none()
            .with_group(Layer::Player)
            .with_masks(&[Layer::World, Layer::Teleport]),
        actor: Actor { health: 10000.0 },
        rotation_constraints: RotationConstraints::lock(),
        marker: FallingGameComponent,
    };
}

// Resources

struct FallingState {
    cycle_number: u8,
}

// Physics

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Teleport,
}

// GAMEPLAY VARIABLES

const RADIUS: f32 = 8.5;

// Systems
fn sys_spawn_player(
    mut commands: Commands,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    audio.play_looped(asset_server.load("music/falling-1.mp3"));

    commands.insert_resource(FallingState { cycle_number: 0 });

    let camera_transform = Transform::from_matrix(Mat4::from_rotation_translation(
        Quat::from_rotation_x(-std::f32::consts::PI / 2.0).normalize(),
        Vec3::new(0.0, 0.0, 0.0),
    ));

    let actor_bundle = new_actor_bundle();

    // Camera
    commands.spawn_bundle(actor_bundle).with_children(|parent| {
        parent.spawn_bundle(PerspectiveCameraBundle {
            transform: camera_transform,
            ..Default::default()
        });
    });
}

fn sys_spawn_teleport(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sphere light

    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        sectors: 128,
        stacks: 64,
        ..default()
    }));

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.1, 0.1),
        unlit: true,
        ..default()
    });

    commands
        .spawn_bundle(TeleportBundle {
            pbr_bundle: PbrBundle {
                mesh: mesh.clone(),
                material: material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(10.0)),
                ..default()
            },
            marker: FallingGameComponent,
            teleport: Teleport,
            collision_shape: CollisionShape::Sphere { radius: 8.5 },
            collision_layers: CollisionLayers::new(Layer::Teleport, Layer::Player),
        })
        .insert(RigidBody::Static)
        .with_children(|children| {
            children.spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    radius: 1500.0,
                    color: Color::rgb(1.0, 0.2, 1.0),
                    ..default()
                },
                ..default()
            });
        });
}

fn sys_spawn_game_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for j in 0..300 {
        let y = (j as f32) * 10.0;

        if j % 5 != 0 {
            continue;
        }

        let cube_type = match rng.gen_range(0..3) {
            0 => CubeType::Brake,
            1 => CubeType::Health,
            2 => CubeType::Speed,
            _ => CubeType::Environment,
        };

        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let radius = rng.gen_range(0.0..RADIUS);

        let color = match cube_type {
            CubeType::Brake => Color::rgba(0.7, 0.1, 0.1, 0.3),
            CubeType::Health => Color::rgba(0.2, 0.7, 0.1, 0.3),
            CubeType::Speed => Color::rgba(0.2, 0.1, 0.7, 0.3),
            _ => Color::WHITE,
        };

        let material = materials.add(StandardMaterial {
            base_color: color,
            reflectance: 0.7,
            alpha_mode: AlphaMode::Opaque,
            perceptual_roughness: 0.08,
            ..default()
        });

        let x = f32::sin(angle) * radius;
        let z = f32::cos(angle) * radius;

        commands
            .spawn_bundle(CubeBundle {
                cube: Cube {
                    cube_type: cube_type,
                },
                pbr_bundle: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.6 })),
                    material: material.clone(),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
                ..default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::ONE,
                border_radius: None,
            })
            .insert(RigidBody::Sensor)
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::World)
                    .with_masks(&[Layer::Player]),
            );
    }
}

fn sys_spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Circle of Cubes

    for j in 0..300 {
        let y = (j as f32) * 10.0;

        let material;

        if j % (2 as u16) == 0 {
            material = materials.add(Color::rgb(0.8, 0.1, 0.1).into());
        } else {
            material = materials.add(Color::rgb(0.8, 0.2, 0.1).into());
        }

        commands
            .spawn_bundle(PbrBundle { ..default() })
            .with_children(|parent| {
                for i in 0..11 {
                    let angle = std::f32::consts::PI * 2.0 / 11.0 * (i as f32);

                    let x = f32::sin(angle) * RADIUS;
                    let z = f32::cos(angle) * RADIUS;

                    parent.spawn_bundle(CubeBundle {
                        pbr_bundle: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
                            material: material.clone(),
                            transform: Transform::from_xyz(x, y, z),
                            ..default()
                        },
                        ..default()
                    });

                    let rotation = Quat::from_rotation_y(std::f32::consts::PI / 4.0);

                    let mut transform = Transform::from_xyz(x, y, z);
                    transform.rotate(rotation);

                    parent.spawn_bundle(CubeBundle {
                        pbr_bundle: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
                            material: material.clone(),
                            transform: transform,
                            ..default()
                        },
                        ..default()
                    });
                }

                // Spawn light source
                parent.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        intensity: 2000.0,
                        shadows_enabled: false,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, y, 0.0),
                    ..Default::default()
                });
            })
            .insert(Floor {
                _rotating: true,
                direction: (j % (2 as u16)) as u8,
            })
            .insert(FallingGameComponent);
    }
}

fn sys_animate_environment(
    time: Res<Time>,
    mut query_cube: Query<(&mut Transform, &Cube), With<Cube>>,
    mut query_floor: Query<(&mut Transform, &Floor), (With<Floor>, Without<Cube>)>,
    mut lines: ResMut<DebugLines>,
    state: Res<FallingState>,
) {
    if state.cycle_number == 0 || state.cycle_number == 2 || state.cycle_number == 4 {
        for (mut transform, cube) in query_cube.iter_mut() {
            if cube.cube_type != CubeType::Environment {
                continue;
            }
            transform.rotation *= Quat::from_rotation_x(1.0 * time.delta_seconds());
            transform.rotation *= Quat::from_rotation_y(0.7 * time.delta_seconds());
        }

        for (mut transform, floor) in query_floor.iter_mut() {
            let mut dir = match floor.direction {
                0 => -1.0,
                _ => 1.0,
            };

            if state.cycle_number == 4 {
                dir = -1.0;
            }

            transform.rotation *= Quat::from_rotation_y(dir * 1.0 * time.delta_seconds());
        }
    }

    if state.cycle_number == 2 || state.cycle_number == 6 {
        let transforms: Vec<&Transform> = query_cube.iter().map(|(t, _)| t).collect();
        let floor_transforms: Vec<(&Transform, &Floor)> = query_floor.iter().collect();

        for j in 0..300 {
            let y = 10.0 * (j as f32);
            let rotation_quat = floor_transforms[j].0.rotation;

            // Spawn Circle of Cubes
            for i in 0..11 {
                let index_end;
                if i + 4 < 11 {
                    index_end = i + 4;
                } else {
                    index_end = i + 4 - 11;
                }

                let mut start_line =
                    rotation_quat.mul_vec3(transforms[j * 11 + i * 2].translation) * 2.0;
                start_line.y = y;

                let mut end_line =
                    rotation_quat.mul_vec3(transforms[j * 11 + index_end * 2].translation) * 2.0;
                end_line.y = y;

                if state.cycle_number == 2 || state.cycle_number == 6 {
                    lines.line_colored(start_line, end_line, 0.1, Color::rgba(0.1, 0.1, 0.1, 0.8));
                }
            }
        }
    }
}

fn sys_adjust_health(
    mut query_actor: Query<(&Velocity, &mut Actor)>,
    mut app_state: ResMut<State<AppState>>,
    audio: Res<Audio>,
    // asset_server: Res<AssetServer>,
) {
    for (v, mut a) in query_actor.iter_mut() {
        let abs_speed = f32::abs(v.linear.y);

        if abs_speed <= 100.0 {
            audio.set_playback_rate(abs_speed / 100.);
        }

        if f32::abs(v.linear.y) > 100.0 {
            a.health -= abs_speed / 1000.0
        }

        if a.health <= 0.0 {
            if *app_state.current() != AppState::GameOver {
                app_state.set(AppState::GameOver).unwrap();
            }
        }
    }
}

fn sys_clear_entities(
    mut commands: Commands,
    mut game_objects: Query<Entity, With<FallingGameComponent>>,
    mut interface: Query<Entity, With<Interface>>,
    mut actors: Query<Entity, With<Actor>>,
) {
    for e in game_objects.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    for e in interface.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    for e in actors.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    // audio.stop();
}

// HUD

fn sys_draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/ARCADECLASSIC.ttf");
    let velocity_text = Text::with_section(
        "",
        TextStyle {
            font_size: 35.0,
            font: font.clone(),
            color: Color::WHITE,
        },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        },
    );

    let health_text = Text::with_section(
        "",
        TextStyle {
            font_size: 35.0,
            font: font.clone(),
            color: Color::GREEN,
        },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        },
    );

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(Interface)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Auto,
                        height: Val::Px(300.0),
                    },
                    position: Rect {
                        top: Val::Px(167.0),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                image: asset_server.load("images/speedometer.png").into(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            });

            parent
                .spawn_bundle(TextBundle {
                    text: velocity_text.clone(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        position: Rect {
                            top: Val::Px(450.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(VelocityText);

            parent
                .spawn_bundle(TextBundle {
                    text: health_text.clone(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        position: Rect {
                            top: Val::Px(480.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HealthText);
        });
}

pub(crate) fn sys_update_hud(
    player_query: Query<(&Actor, &Velocity)>,
    mut velocity_text: Query<&mut Text, (With<VelocityText>, Without<HealthText>)>,
    mut health_text: Query<&mut Text, (With<HealthText>, Without<VelocityText>)>,
) {
    let player = player_query.iter().last().unwrap();
    let velocity = player.1;
    let actor = player.0;

    for mut text in velocity_text.iter_mut() {
        let str = format!("speed   {}", -(velocity.linear.y as i32)).to_string();
        text.sections[0].value = str;
    }

    for mut text in health_text.iter_mut() {
        let str = format!("health   {}", (actor.health as i32)).to_string();
        text.sections[0].value = str;
    }
}

// Control

bitflags! {
    #[derive(Default)]
    pub struct PlayerActionFlags: u32 {
        const IDLE = 1 << 0;
        const UP = 1 << 1;
        const DOWN = 1 << 2;
        const LEFT = 1 << 3;
        const RIGHT = 1 << 4;
        const BRAKE = 1 << 5;
    }
}

pub(crate) fn sys_keyboard_control(
    keys: Res<Input<KeyCode>>,
    player_movement_q: Query<(&mut heron::prelude::Velocity, &mut Transform), With<Actor>>,
    collision_events: EventReader<CollisionEvent>,
) {
    let mut player_action = PlayerActionFlags::IDLE;

    for key in keys.get_pressed() {
        if *key == KeyCode::Left {
            player_action |= PlayerActionFlags::LEFT;
        }
        if *key == KeyCode::Right {
            player_action |= PlayerActionFlags::RIGHT;
        }
        if *key == KeyCode::Up {
            player_action |= PlayerActionFlags::UP;
        }
        if *key == KeyCode::Down {
            player_action |= PlayerActionFlags::DOWN;
        }
        if *key == KeyCode::Space {
            player_action |= PlayerActionFlags::BRAKE;
        }
    }

    control_player(player_action, player_movement_q, collision_events);
}

pub(crate) fn control_player(
    player_action: PlayerActionFlags,
    mut player_movement_q: Query<(&mut heron::prelude::Velocity, &mut Transform), With<Actor>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    fn is_player(layers: CollisionLayers) -> bool {
        layers.contains_group(Layer::Player) && !layers.contains_group(Layer::World)
    }

    fn is_world(layers: CollisionLayers) -> bool {
        !layers.contains_group(Layer::Player) && layers.contains_group(Layer::World)
    }

    const SPEED: f32 = 0.3;

    for (mut velocity, mut transform) in player_movement_q.iter_mut() {
        // *velocity = Velocity::from_linear(Vec3::ZERO);

        if player_action.contains(PlayerActionFlags::UP) {
            let delta = transform.translation + Vec3::new(0.0, 0.0, -SPEED);
            let radius = (delta.x.powf(2.0) + delta.z.powf(2.0)).sqrt();
            if radius <= RADIUS - 1.0 {
                transform.translation = delta;
            }
        }
        if player_action.contains(PlayerActionFlags::LEFT) {
            let delta = transform.translation + Vec3::new(-SPEED, 0.0, 0.0);
            let radius = (delta.x.powf(2.0) + delta.z.powf(2.0)).sqrt();
            if radius <= RADIUS - 1.0 {
                transform.translation = delta;
            }
        }
        if player_action.contains(PlayerActionFlags::DOWN) {
            let delta = transform.translation + Vec3::new(0.0, 0.0, SPEED);
            let radius = (delta.x.powf(2.0) + delta.z.powf(2.0)).sqrt();
            if radius <= RADIUS - 1.0 {
                transform.translation = delta;
            }
        }
        if player_action.contains(PlayerActionFlags::RIGHT) {
            let delta = transform.translation + Vec3::new(SPEED, 0.0, 0.0);
            let radius = (delta.x.powf(2.0) + delta.z.powf(2.0)).sqrt();
            if radius <= RADIUS - 1.0 {
                transform.translation = delta;
            }
        }

        if player_action.contains(PlayerActionFlags::BRAKE) {
            if velocity.linear.y < 0.0 {
                velocity.linear.y += 1.0;
            }
        }

        collision_events
            .iter()
            .filter_map(|event| {
                let (entity_1, entity_2) = event.rigid_body_entities();
                let (layers_1, layers_2) = event.collision_layers();

                if is_player(layers_1) && is_world(layers_2) {
                    Some(entity_2)
                } else if is_player(layers_2) && is_world(layers_1) {
                    Some(entity_1)
                } else {
                    None
                }
            })
            .for_each(|_| {
                *velocity = Velocity::from_linear(Vec3::X * 0.0);
            });
    }
}

fn sys_check_teleport_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut query_actor: Query<(&mut Transform, &mut Velocity, &Actor)>,
    mut state: ResMut<FallingState>,
) {
    fn is_player(layers: CollisionLayers) -> bool {
        layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Teleport)
    }

    fn is_teleport(layers: CollisionLayers) -> bool {
        !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Teleport)
    }

    let events = collision_events
        .iter()
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_teleport(layers_2) {
                Some(entity_2)
            } else if is_player(layers_2) && is_teleport(layers_1) {
                Some(entity_1)
            } else {
                None
            }
        })
        .count();

    if events > 0 {
        for (mut t, mut v, _) in query_actor.iter_mut() {
            if state.cycle_number < 6 {
                v.linear.y = 0.0;
            }
            t.translation.y = 3000.0;
        }

        state.cycle_number += 1;
    }
}

fn sys_check_game_cube_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_actor: Query<(&mut Transform, &mut Velocity, &Actor)>,
    mut state: ResMut<FallingState>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    fn is_player(layers: CollisionLayers) -> bool {
        layers.contains_group(Layer::Player) && !layers.contains_group(Layer::World)
    }

    fn is_world(layers: CollisionLayers) -> bool {
        !layers.contains_group(Layer::Player) && layers.contains_group(Layer::World)
    }

    collision_events
        .iter()
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_world(layers_2) {
                Some(entity_2)
            } else if is_player(layers_2) && is_world(layers_1) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|e| {
            commands.entity(e).despawn_recursive();
            for (_, mut v, a) in query_actor.iter_mut() {
                v.linear.y = -100.0;

                println!("!!!!");
            }

            audio.set_playback_rate(1.0);
            audio.play(asset_server.load("music/box-hit.mp3"));
        });
}

fn sys_scene_change(
    mut commands: Commands,
    state: Res<FallingState>,
    mut app_state: ResMut<State<AppState>>,
    mut query_cube: Query<(&mut Visibility, With<Cube>)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    if !state.is_changed() {
        return;
    }

    match state.cycle_number {
        0 => {}
        2 => {
            commands.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)));

            for (mut v, _) in query_cube.iter_mut() {
                v.is_visible = false;
            }
        }
        4 => {
            commands.insert_resource(ClearColor(Color::rgb(0.0, 0.1, 0.1)));
            audio.stop();
            audio.play_looped(asset_server.load("music/falling-2.mp3"));

            let mut counter = 0;
            for (mut v, _) in query_cube.iter_mut() {
                v.is_visible = false;

                if counter % 5 == 0 {
                    v.is_visible = true;
                } else {
                    v.is_visible = false;
                }

                counter += 1;
            }
        }
        6 => {
            commands.insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)));

            let mut counter = 0;
            for (mut v, _) in query_cube.iter_mut() {
                v.is_visible = false;

                if counter % 7 == 0 {
                    v.is_visible = true;
                } else {
                    v.is_visible = false;
                }

                counter += 1;
            }
        }
        8 => {
            app_state.set(AppState::GameOver).unwrap();
        }
        _ => {}
    }
}
// Plugins

pub struct FallingMinigamePlugin;
impl Plugin for FallingMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default())
            .add_plugin(DebugLinesPlugin::with_depth_test(true))
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)));

        app.insert_resource(FallingState { cycle_number: 0 });

        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(sys_animate_environment)
                .with_system(sys_update_hud)
                .with_system(sys_keyboard_control)
                .with_system(sys_check_teleport_collision)
                .with_system(sys_scene_change)
                .with_system(sys_check_game_cube_collision),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.05))
                .with_system(sys_adjust_health),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(sys_spawn_game_cubes)
                .with_system(sys_spawn_player)
                .with_system(sys_draw_hud)
                .with_system(sys_spawn_environment)
                .with_system(sys_spawn_teleport),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(sys_clear_entities));
    }
}
