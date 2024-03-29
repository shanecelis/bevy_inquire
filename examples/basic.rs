use bevy_inquire::*;
use bevy::{render::camera::ClearColorConfig, prelude::*, window::PresentMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Inquire Example".into(),
                resolution: (600., 400.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                // fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                // window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let settings = BevySettings {
    //     style: TextStyle {
    //         font: asset_server.load("fonts/DejaVuSansMono.ttf"),
    //         font_size: 50.0,
    //         color: Color::WHITE,
    //     },
    // };
    // commands.insert_resource(settings);
    commands.spawn(Camera2dBundle {
        camera: Camera {
            // disable clearing completely (pixels stay as they are)
            // (preserves output from previous frame or camera/pass)
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..Default::default()
    });
    commands.spawn((NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }, BevyTerminal::default()));
}
