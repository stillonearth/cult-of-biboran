use std::time::Duration;

use bevy::{core::Stopwatch, prelude::*};

use crate::app_states::AppState;

// Components

#[derive(Debug, Component)]
pub(crate) struct CutsceneComponent;

// Resourcesf

pub(crate) struct CutsceneSettings {
    pub next_stage: u8,
}

// Systems

pub(crate) fn sys_show_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<CutsceneSettings>,
    mut query: Query<Entity, With<CutsceneComponent>>,
    mut stopwatch: ResMut<Stopwatch>,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
) {
    if stopwatch.elapsed_secs() < 5.0 && settings.next_stage != 0 {
        stopwatch.tick(Duration::from_secs_f32(time.delta_seconds()));
        return;
    }

    stopwatch.reset();
    settings.next_stage += 1;

    if settings.next_stage == 6 {
        app_state.set(AppState::FallingGame).unwrap();
        return;
    }

    let mut cutscene_items: Vec<(String, String)> = Vec::new();
    cutscene_items.push((
        "And there was God, and there was Satan,\n and there were humans\n, and there were non-humans\n, and there was\n Alexander Gavrilovich Abdulov.".to_string(),
        "images/story/1.png".to_string(),
    ));
    cutscene_items.push((
        "And there were humans\nAnd there were unhumans".to_string(),
        "images/story/2.png".to_string(),
    ));
    cutscene_items.push((
        "And there was\nAlexander\nGavrilovich\nAbdulov".to_string(),
        "images/story/3.png".to_string(),
    ));
    cutscene_items.push((
        "And sayeth Abdulov\nI am the Law".to_string(),
        "images/story/4.png".to_string(),
    ));
    cutscene_items.push((
        "He slayed God and Satan\nto become GodSatan\nall in one".to_string(),
        "images/story/5.png".to_string(),
    ));

    // Clear previous items
    for e in query.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CutsceneComponent);

    // Spawn items

    let text = Text::with_section(
        &cutscene_items[(settings.next_stage - 1) as usize].0,
        TextStyle {
            color: Color::WHITE.into(),
            font_size: 75.0,
            font: asset_server.load("fonts/ARCADECLASSIC.TTF"),
        },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        },
    );

    let img = &cutscene_items[(settings.next_stage - 1) as usize].1;
}

fn sys_clear(
    mut commands: Commands,
    mut query: Query<Entity, With<CutsceneComponent>>,
    // audio: Res<Audio>,
) {
    for q in query.iter_mut() {
        commands.entity(q).despawn_recursive();
    }

    // audio.stop();
}

// Plugins

pub struct CutscenePlugin;
impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(AppState::CutScene).with_system(sys_clear))
            .add_system_set(SystemSet::on_update(AppState::CutScene).with_system(sys_show_scene));
    }
}
