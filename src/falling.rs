use crate::app_states::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use heron::*;
// Components

#[derive(Component, Default)]
pub struct Cube;

#[derive(Component, Default)]
pub struct Floor {
    direction: u8,
    rotating: bool,
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
}

#[derive(Component, Clone)]
pub(crate) struct Actor {
    health: f32,
}

#[derive(Bundle)]
pub(crate) struct ActorBundle {
    collision_layers: CollisionLayers,
    collision_shape: CollisionShape,
    global_transform: GlobalTransform,
    actor: Actor,
    rigid_body: RigidBody,
    rotation_constraints: RotationConstraints,
    transform: Transform,
    velocity: Velocity,
    physics_material: PhysicMaterial,
}

fn new_actor_bundle() -> ActorBundle {
    return ActorBundle {
        transform: Transform {
            translation: Vec3::new(0.0 as f32, 2000.0, 0.0),
            ..default()
        },
        global_transform: GlobalTransform::identity(),
        velocity: Velocity::from_linear(Vec3::ZERO),
        collision_shape: CollisionShape::Sphere { radius: 1.0 },
        rigid_body: RigidBody::Dynamic,
        physics_material: PhysicMaterial {
            density: 200.0,
            ..Default::default()
        },
        collision_layers: CollisionLayers::new(Layer::Player, Layer::World),
        actor: Actor { health: 100.0 },
        rotation_constraints: RotationConstraints::lock(),
    };
}

// Physics

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Enemies,
}

// Systems
fn sys_spawn_player(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio.play_looped(asset_server.load("music/falling.mp3"));

    let mut camera_transform = Transform::from_matrix(Mat4::from_rotation_translation(
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

pub fn sys_spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Circle of Cubes

    for j in 0..200 {
        let y = (j as f32) * 10.0;

        let material;

        if j % 2 == 0 {
            material = materials.add(Color::rgb(0.8, 0.1, 0.1).into());
        } else {
            material = materials.add(Color::rgb(0.8, 0.2, 0.1).into());
        }

        commands
            .spawn_bundle(PbrBundle { ..default() })
            .with_children(|parent| {
                for i in 0..11 {
                    let angle = std::f32::consts::PI * 2.0 / 11.0 * (i as f32);
                    let radius = 3.5;

                    let x = f32::sin(angle) * radius;
                    let z = f32::cos(angle) * radius;

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
                rotating: true,
                direction: j % 2,
            });
    }
}

pub fn sys_move_cubes(
    time: Res<Time>,
    mut query_cube: Query<&mut Transform, With<Cube>>,
    mut query_floor: Query<(&mut Transform, &Floor), (With<Floor>, Without<Cube>)>,
) {
    for mut transform in query_cube.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_y(0.7 * time.delta_seconds());
    }

    for (mut transform, floor) in query_floor.iter_mut() {
        let dir = match floor.direction {
            0 => -1.0,
            _ => 1.0,
        };

        transform.rotation *= Quat::from_rotation_y(dir * 1.0 * time.delta_seconds());
    }
}

fn sys_check_termination(mut query_actor: Query<(&mut Transform, &Actor)>) {
    for (mut t, _) in query_actor.iter_mut() {
        if t.translation.y <= 000.0 {
            t.translation.y = 2000.0;
        }
    }
}

fn sys_adjust_health(mut query_actor: Query<(&Velocity, &mut Actor)>) {
    for (v, mut a) in query_actor.iter_mut() {
        if f32::abs(v.linear.y) > 100.0 {
            a.health -= f32::abs(v.linear.y) / 1000.0
        }

        if a.health <= 0.0 {
            println!("Game over");
        }
    }
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

// Plugins

pub struct FallingMinigamePlugin;
impl Plugin for FallingMinigamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)));

        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(sys_move_cubes)
                .with_system(sys_update_hud)
                .with_system(sys_check_termination),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.05))
                .with_system(sys_adjust_health),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(sys_spawn_player)
                .with_system(sys_draw_hud)
                .with_system(sys_spawn_cubes),
        );
    }
}
