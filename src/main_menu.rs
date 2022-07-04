use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_prototype_debug_lines::DebugLines;

use crate::app_states::*;
use crate::bloodfield::*;
use rand::Rng;

// Components

#[derive(Component, Default)]
pub struct Cube;

#[derive(Component, Default)]
pub struct MainMenuComponent;

#[derive(Component, Default)]
pub struct Pentagram;

// Bundles

#[derive(Bundle, Default)]
struct CubeBundle {
    #[bundle]
    pbr_bundle: PbrBundle,
    cube: Cube,
    marker: MainMenuComponent,
}

// Systems

fn sys_setup_camera(mut commands: Commands) {
    let mut camera_transform = Transform::from_matrix(Mat4::from_rotation_translation(
        Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
        Vec3::new(-10.0, 18.0, 0.0),
    ));

    camera_transform.scale.z = 1.5;

    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: camera_transform,
            ..Default::default()
        })
        .insert(MainMenuComponent);
}

pub fn sys_spawn_circle_of_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bmaterials: ResMut<Assets<BloodfieldMaterial>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    audio.play_looped(asset_server.load("music/biboran.mp3"));

    let texture_handle = asset_server.load("images/abdulovhell.jpg");

    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.6, 0.6, 0.5),
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    let mut image_transform =
        Transform::from_translation(Vec3::new(-5.5, 7.1, 0.0)).with_scale(Vec3::new(5.0, 0.0, 7.3));

    image_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.0));

    let bloodfield_material = bmaterials.add(BloodfieldMaterial {
        time: 0.0,
        seed: rand::thread_rng().gen::<i16>() as f32,
    });

    let bundle = MaterialMeshBundle {
        mesh: mesh.clone(),
        material: red_material_handle,
        transform: image_transform,
        ..default()
    };

    commands.spawn_bundle(bundle).insert(MainMenuComponent);

    // Spawn background shader mesh
    let mut image_transform = Transform::from_translation(Vec3::new(-5.5, 0.0, 0.0))
        .with_scale(Vec3::new(25.0, 0.0, 25.0));

    image_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.0));

    let bundle = MaterialMeshBundle {
        mesh: mesh.clone(),
        material: bloodfield_material,
        // material: red_material_handle,
        transform: image_transform,
        ..default()
    };

    commands.spawn_bundle(bundle).insert(MainMenuComponent);

    // Spawn Circle of Cubes
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
                        material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
                        transform: Transform::from_xyz(x, 0.0, z),
                        ..default()
                    },
                    ..default()
                });

                let rotation = Quat::from_rotation_y(std::f32::consts::PI / 4.0);

                let mut transform = Transform::from_xyz(x, 0.0, z);
                transform.rotate(rotation);

                parent.spawn_bundle(CubeBundle {
                    pbr_bundle: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
                        material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
                        transform: transform,
                        ..default()
                    },
                    ..default()
                });
            }
        })
        .insert(Pentagram)
        .insert(MainMenuComponent);

    // Spawn light source
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 200.0,
                shadows_enabled: false,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(MainMenuComponent);

    // Draw Title
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MainMenuComponent);
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let text = Text::with_section(
        "CVLT OV BIBΩRAN",
        TextStyle {
            font_size: 35.0,
            font: font.clone(),
            color: Color::rgb(0.9, 0.9, 0.9),
        },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        },
    );

    // Draw Button
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(33.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(170.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position: Rect {
                            top: Val::Px(50.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::rgb(0.6, 0.1, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "CONFESS",
                            TextStyle {
                                font: font.clone(),
                                font_size: 15.0,
                                color: Color::WHITE.into(),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent.spawn_bundle(TextBundle {
                text,
                ..Default::default()
            });
        })
        .insert(MainMenuComponent);
}

const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.75, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(1.0, 0.35, 0.25);

fn sys_button_new_game(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (interaction, mut color, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                audio.play(asset_server.load("music/click.mp3"));
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                audio.play(asset_server.load("music/hover.mp3"));
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn sys_rotate_cube(
    time: Res<Time>,
    mut query_cube: Query<&mut Transform, With<Cube>>,
    mut query_pentagram: Query<&mut Transform, (With<Pentagram>, Without<Cube>)>,
    mut lines: ResMut<DebugLines>,
) {
    for mut transform in query_cube.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_y(0.7 * time.delta_seconds());
    }

    let transforms: Vec<&Transform> = query_cube.iter().collect();
    let pentragram_transforms: Vec<&Transform> = query_pentagram.iter().collect();
    let rotation_quat = pentragram_transforms[0].rotation;

    // Spawn Circle of Cubes
    for i in 0..11 {
        let index_end;
        if i + 4 < 11 {
            index_end = i + 4;
        } else {
            index_end = i + 4 - 11;
        }

        let start_line = rotation_quat.mul_vec3(transforms[i * 2].translation);
        let end_line = rotation_quat.mul_vec3(transforms[index_end * 2].translation);

        lines.line_colored(start_line, end_line, 0.2, Color::rgba(0.9, 0.7, 0.2, 0.3));
    }

    // pentagram rotate
    for mut transform in query_pentagram.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.1 * time.delta_seconds());
    }
}

pub fn draw_random_lines(mut lines: ResMut<DebugLines>) {
    let mut rng = rand::thread_rng();
    // Spawn Circle of Cubes
    for _ in 0..60 {
        let start_line = Vec3::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );
        let end_line = Vec3::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );

        lines.line_colored(start_line, end_line, 0.5, Color::rgba(0.1, 0.01, 0.01, 1.0));
    }
}

fn sys_clear_entities(
    mut commands: Commands,
    audio: Res<Audio>,
    mut app_state: ResMut<State<AppState>>,
    mut main_menu_components: Query<Entity, With<MainMenuComponent>>,
) {
    for e in main_menu_components.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    audio.stop();
}

// Plugins

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BloodfieldPlugin)
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(sys_rotate_cube)
                    .with_system(draw_random_lines)
                    .with_system(sys_button_new_game),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(sys_setup_camera)
                    .with_system(sys_spawn_circle_of_cubes),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(sys_clear_entities));
    }
}
