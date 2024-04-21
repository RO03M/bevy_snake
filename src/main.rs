use bevy::{prelude::*, window::WindowResolution};
use bevy_snake::snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SNake".into(),
                        resolution: WindowResolution::new(500.0, 500.0),
                        ..default()
                    }),
                    ..default()
                }),
            SnakePlugin
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}