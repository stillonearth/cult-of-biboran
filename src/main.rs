use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

mod app_states;
mod bloodfield;
mod cutscene;
mod falling;
mod game_end;
mod game_over;
mod indoctrination;
mod main_menu;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        // External plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        // Main menu
        .add_plugin(main_menu::MainMenuPlugin)
        // Screens
        .add_plugin(game_over::GameOverScreenPlugin)
        // Screens
        .insert_resource(game_end::GameStats { time: 0 })
        .add_plugin(game_end::GameEndPlugin)
        // Falling Game
        .add_plugin(falling::FallingMinigamePlugin)
        // States
        .add_state(app_states::AppState::MainMenu);

    app.run();
}
