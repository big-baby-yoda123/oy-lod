use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_game_camera);
    }
}

fn setup_game_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
