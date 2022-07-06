use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

mod app_states;
mod bloodfield;
mod falling;
mod game_over;
mod main_menu;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // Main menu
        .add_plugin(main_menu::MainMenuPlugin)
        // Main menu
        .add_plugin(game_over::GameOverScreenPlugin)
        // Falling Game
        .add_plugin(falling::FallingMinigamePlugin)
        // States
        .add_state(app_states::AppState::MainMenu);

    app.run();
}
