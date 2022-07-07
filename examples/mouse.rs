use bevy::{
    input::Input,
    prelude::{App, Bundle, Commands, Component, DefaultPlugins, Query, Res, ResMut, With},
    window::Windows,
};
use ezinput::prelude::*;

input! {
    EnumeratedBinding {
        Movement<EnumeratedMovementBinding> {
            Jump = [KeyCode::Space, GamepadButtonType::South],
            Vertical = [KeyCode::W, KeyCode::S => -1., MouseAxisType::Y],
            Horizontal = [KeyCode::A => -1. /* default axis value */, KeyCode::D, MouseAxisType::X],
            Hello = [MouseAxisType::Wheel],
            Hi = [(MouseAxisType::X, MouseAxisDelta(MouseAxisType::X))],
            Combination = [(KeyCode::E, MouseButton::Left)]
        }
    }
}

type EnumeratedInputView = InputView<EnumeratedBinding>;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    #[bundle]
    pub input: InputHandlingBundle<EnumeratedBinding>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            input: InputHandlingBundle::with_deadzone(EnumeratedBinding::view(), (0.25, 0.25)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EZInputPlugin::<EnumeratedBinding>::default())
        .add_startup_system(spawn_player)
        .add_startup_system(sys_mouse_cursor_grab)
        .add_system(check_input)
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle::default());
}

fn check_input(query: Query<&EnumeratedInputView, With<Player>>) {
    use EnumeratedBinding::*;
    use EnumeratedMovementBinding::*;

    let view = query.single();

    let mut x = 0.0;
    let mut y = 0.0;

    if let Some(axis) = view.axis(&Movement(Horizontal)).first() {
        if axis.pressed() {
            x = axis.value;
        }
    }
    if let Some(axis) = view.axis(&Movement(Vertical)).first() {
        if axis.pressed() {
            y = axis.value;
        }
    }

    if x == 0.0 || y == 0.0 {
        return;
    }

    println!("x {} y {}", x, y);
    x = (x - 1280.0 / 2.) / 1280.0;
    y = (720.0 / 2. - y) / 720.0 * 1.77;

    println!("x {} y {}", x, y);

    let angle = f32::atan2(y, x);
    let radius = (x.powf(2.0) + y.powf(2.0)).sqrt();

    let x = f32::cos(angle);
    let y = f32::sin(angle);

    println!("----- {} -----", radius);
}

fn sys_mouse_cursor_grab(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
