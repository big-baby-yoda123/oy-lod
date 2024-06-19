use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Players(2.0))
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_shoolhan.after(spawn_camera),
                spawn_light,
                spawn_cards.after(spawn_shoolhan),
            ),
        )
        .run();
}

#[derive(Resource)]
pub struct Players(pub f32);

pub fn spawn_cards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    num_players: Res<Players>, //card_type: String,
) {
    //let card_type: String = "white".to_string();
    for i in 0..num_players.0 as u32 {
        let angle = 2.0 * PI * (i as f32) / num_players.0;
        commands.spawn((PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::CYAN),
            transform: Transform::from_xyz(angle.cos() * 2.0, 1.0, angle.sin() * 2.0)
                .with_scale(Vec3::new(0.6, 1.0, 0.05))
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },));
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_shoolhan(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cylinder::default()),
        material: materials.add(Color::ORANGE),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(4.0, 0.2, 4.0)),
        ..default()
    },));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
}
