use std::time::Duration;

use benimator::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(benimator::AnimationPlugin::default()) // <-- Add the plugin
        .add_startup_system(spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    // Don't forget the camera ;-)
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let row = 10;
    // Create an animation
    // Here we use an index-range (from 0 to 4) where each frame has the same duration
    let animation_handle = animations.add(SpriteSheetAnimation::from_range(
        24 * row + 1..=24 * row + 8,
        Duration::from_millis(100),
    ));

    commands
        // Spawn a bevy sprite-sheet
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("sprites/alextime.png"),
                Vec2::new(64.0, 64.0),
                24,
                22,
            )),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..Default::default()
        })
        // Insert the asset handle of the animation
        .insert(animation_handle)
        // Start the animation immediately. Remove this component in order to pause the animation.
        .insert(Play);
}
