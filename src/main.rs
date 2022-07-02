use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;

mod menu;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup_camera)
        .add_startup_system(menu::spawn_circle_of_cubes)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(menu::sys_rotate_cube)
        .add_system(menu::draw_random_lines);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera_transform = Transform::from_matrix(Mat4::from_rotation_translation(
        Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
        Vec3::new(-10.0, 18.0, 0.0),
    ));

    camera_transform.scale.z = 1.5;

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    });
}
