use bevy::prelude::*;
use heron::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands
        // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
        .spawn_bundle(SpriteBundle::default())
        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Sphere { radius: 10.0 })
        // Optionally add other useful components...
        .insert(Velocity::from_linear(Vec3::X * 2.0))
        .insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(PhysicMaterial {
            friction: 1.0,
            density: 10.0,
            ..Default::default()
        })
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_mask(Layer::World),
        );
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
}
