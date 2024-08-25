use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

pub fn toggle_vsync(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::KeyV) {
        let mut window = windows.single_mut();

        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

pub fn toggle_window_controls(input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    let toggle_minimize = input.just_pressed(KeyCode::Minus);
    let toggle_maximize = input.just_pressed(KeyCode::Equal);
    let toggle_close = input.just_pressed(KeyCode::Digit0);

    if toggle_minimize || toggle_maximize || toggle_close {
        let mut window = windows.single_mut();

        if toggle_minimize {
            window.enabled_buttons.minimize = !window.enabled_buttons.minimize;
        }
        if toggle_maximize {
            window.enabled_buttons.maximize = !window.enabled_buttons.maximize;
        }
        if toggle_close {
            window.enabled_buttons.close = !window.enabled_buttons.close;
        }
    }
}

pub fn toggle_theme(mut windows: Query<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F11) {
        let mut window = windows.single_mut();

        if let Some(current_theme) = window.window_theme {
            window.window_theme = match current_theme {
                WindowTheme::Light => Some(WindowTheme::Dark),
                WindowTheme::Dark => Some(WindowTheme::Light),
            };
        }
    }
}
