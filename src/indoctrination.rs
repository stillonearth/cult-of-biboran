use bevy::{core::FixedTimestep, prelude::*};

use rand::Rng;

// Components

#[derive(Component, Default)]
pub struct IndoctrinationComponent;

#[derive(Debug, Component)]
pub(crate) struct UiFixedZ {
    pub z: f32,
}

// Resources

pub(crate) struct IndoctrinationSettings {
    pub enabled: bool,
}

// Systems

pub(crate) fn sys_ui_apply_fixed_z(
    mut node_query: Query<(&mut Transform, &mut GlobalTransform, &UiFixedZ), With<Node>>,
) {
    for (mut transform, mut global_transform, fixed) in node_query.iter_mut() {
        transform.translation.z = fixed.z;
        global_transform.translation.z = fixed.z;
    }
}

pub(crate) fn sys_show_25_frame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: ResMut<IndoctrinationSettings>,
    query: Query<Entity, With<IndoctrinationComponent>>,
) {
    if !settings.enabled {
        return;
    }

    if query.iter().count() > 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);
    if num > 1 {
        return;
    }

    let num = rng.gen_range(0..20);

    if num <= 10 {
        let mut text = "BIBORAN";

        if num == 2 {
            text = "ANSHA ABDUL";
        } else if num <= 4 {
            text = "VODKA";
        } else if num <= 6 {
            text = "CIGARETTES";
        } else if num <= 8 {
            text = "DRINK";
        } else if num <= 10 {
            text = "SMOKE";
        }

        let text = Text::with_section(
            text,
            TextStyle {
                color: Color::RED.into(),
                font_size: 250.0,
                font: asset_server.load("fonts/AThemeForMurder-3aPG.ttf"),
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        );

        commands
            .spawn_bundle(NodeBundle {
                color: Color::BLACK.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
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
            .insert(UiFixedZ { z: 101. })
            .insert(IndoctrinationComponent)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: text.clone(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(UiFixedZ { z: 101. });
            });
    } else {
        let img;

        if num <= 13 {
            img = "images/flashback-1.jpg";
        } else if num <= 15 {
            img = "images/flashback-2.jpg";
        } else if num <= 17 {
            img = "images/flashback-3.jpg";
        } else {
            img = "images/flashback-4.jpg";
        }

        commands
            .spawn_bundle(NodeBundle {
                color: Color::BLACK.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
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
            .insert(UiFixedZ { z: 101. })
            .insert(IndoctrinationComponent)
            .with_children(|parent| {
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size {
                                width: Val::Auto,
                                height: Val::Px(500.0),
                            },
                            position_type: PositionType::Absolute,
                            align_self: AlignSelf::Center,
                            ..Default::default()
                        },
                        image: asset_server.load(img).into(),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        ..Default::default()
                    })
                    .insert(UiFixedZ { z: 101. });
            });
    }
}

fn sys_clear_25_frame(
    mut commands: Commands,
    mut query: Query<Entity, With<IndoctrinationComponent>>,
) {
    for q in query.iter_mut() {
        commands.entity(q).despawn_recursive();
    }
}

// Plugins

pub struct IndoctrinationPlugin;
impl Plugin for IndoctrinationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, sys_ui_apply_fixed_z)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.041))
                    .with_system(sys_show_25_frame),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))
                    .with_system(sys_clear_25_frame),
            );
    }
}
