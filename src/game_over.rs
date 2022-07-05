use crate::app_states::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::egui::style::Margin;
use bevy_kira_audio::Audio;

// Components

#[derive(Component, Default)]
pub struct Interface;

#[derive(Component, Default)]
pub struct GameOverText;

// HUD

fn sys_draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/ARCADECLASSIC.ttf");
    let game_over_text = Text::with_section(
        "game over",
        TextStyle {
            font_size: 65.0,
            font: font.clone(),
            color: Color::WHITE,
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
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(Interface)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: game_over_text.clone(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        // position: Rect {
                        //     top: Val::Px(450.0),
                        //     ..Default::default()
                        // },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(GameOverText);

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
                        margin: Rect {
                            top: Val::Px(60.0),
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
                                font: asset_server.load("fonts/ARCADECLASSIC.TTF"),
                                font_size: 20.0,
                                color: Color::WHITE.into(),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
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

fn sys_clear_entities(
    mut commands: Commands,
    audio: Res<Audio>,
    mut app_state: ResMut<State<AppState>>,
    mut main_menu_components: Query<Entity, With<Interface>>,
) {
    for e in main_menu_components.iter_mut() {
        commands.entity(e).despawn_recursive();
    }

    audio.stop();
}

// Plugins

pub struct GameOverScreenPlugin;
impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(sys_draw_hud))
            .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(sys_clear_entities))
            .add_system_set(
                SystemSet::on_update(AppState::GameOver).with_system(sys_button_new_game),
            );
    }
}
