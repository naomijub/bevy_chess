#[cfg(feature = "debugger")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
#[cfg(feature = "debugger")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_mod_picking::DefaultPickingPlugins;
use chess::{
    board::BoardPlugin, debugger::DebuggerPlugin, pieces::PiecesPlugin, setup::setup_base,
};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::srgb(0.3, 0.3, 0.3)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Basic Chess!".into(),
                    name: Some("chess.app".into()),
                    resolution: (1200., 800.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        minimize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            #[cfg(feature = "debugger")]
            LogDiagnosticsPlugin::default(),
            #[cfg(feature = "debugger")]
            FrameTimeDiagnosticsPlugin,
            #[cfg(feature = "debugger")]
            WorldInspectorPlugin::default().run_if(
                bevy::input::common_conditions::input_toggle_active(false, KeyCode::Escape),
            ),
        ))
        .add_plugins((
            DefaultPickingPlugins,
            DebuggerPlugin,
            BoardPlugin,
            PiecesPlugin,
        ))
        .add_systems(Startup, setup_base)
        .run();
}
