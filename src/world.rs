use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_static_objects,
                spawn_light,
                spawn_cubes,
                spawn_slopes,
                spawn_sphere,
            ),
        );
    }
}

fn spawn_static_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Name::from("Ground plane"),
        Collider::cuboid(100.0, 0.1, 100.0),
        PbrBundle {
            mesh: meshes.add(shape::Box::new(200.0, 0.2, 200.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::from("Directional light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 20.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_position = Vec3::new(2.0, 4.0, -20.0);
    let max_offset = Vec3::new(10.0, 1.0, 10.0);
    let spawn_count = 20;
    let size = 1.2;

    let mesh_handle = build_cube_mesh(&mut meshes, size);
    let material_handle = build_material(&mut materials, Color::YELLOW);

    for i in 0..spawn_count {
        let position_offset = random_vec3() * max_offset;
        let spawn_position = base_position + position_offset + Vec3::Y * i as f32;

        commands.spawn((
            Name::from(format!("Piled physics cube {}", i + 1)),
            build_cube(
                spawn_position,
                Quat::IDENTITY,
                Vec3::splat(size),
                mesh_handle.clone(),
                material_handle.clone(),
            ),
            RigidBody::Dynamic,
            Damping {
                linear_damping: 0.5,
                ..default()
            },
        ));
    }
}

fn spawn_slopes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_position = Vec3::new(10.0, 0.0, 0.0);
    let base_rotation = Quat::from_axis_angle(Vec3::Y, (0.0 as f32).to_radians());
    let spacing = 5.0;
    let size = Vec3::new(2.0, 0.5, 6.0);
    let spawn_count = 4;

    let start_angle_deg = 20.0;
    let angle_increase = 10.0;

    let mesh_handle = build_rectangle_mesh(&mut meshes, size);
    let material_handle = build_material(&mut materials, Color::AQUAMARINE);

    for i in 0..spawn_count {
        let spawn_position = base_position + get_position_offset(i, base_rotation, spacing);

        commands.spawn((
            Name::from(format!("Slope {}", i + 1)),
            build_cube(
                spawn_position,
                Quat::from_axis_angle(
                    Vec3::X,
                    get_slope_angle_deg(i, start_angle_deg, angle_increase).to_radians(),
                ),
                size,
                mesh_handle.clone(),
                material_handle.clone(),
            ),
        ));
    }

    fn get_position_offset(index: i32, rotation: Quat, spacing: f32) -> Vec3 {
        index as f32 * (rotation * (Vec3::X * spacing))
    }

    fn get_slope_angle_deg(index: i32, start_angle_deg: f32, angle_increase: f32) -> f32 {
        start_angle_deg + index as f32 * angle_increase
    }
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 10.0;
    let position = Vec3::new(-5.0, -9.0, -3.0);

    let mesh_handle = build_sphere_mesh(&mut meshes, radius);
    let material_handle = build_material(&mut materials, Color::AQUAMARINE);

    commands.spawn((
        Name::from("Sphere inside ground"),
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform: Transform::from_translation(position),
            ..default()
        },
        Collider::ball(radius),
    ));
}

fn random_vec3() -> Vec3 {
    let mut rng = thread_rng();
    Vec3 {
        x: rng.gen(),
        y: rng.gen(),
        z: rng.gen(),
    }
}

fn build_cube_mesh(meshes: &mut ResMut<Assets<Mesh>>, size: f32) -> Handle<Mesh> {
    meshes.add(shape::Cube::new(size).into())
}

fn build_rectangle_mesh(meshes: &mut ResMut<Assets<Mesh>>, size: Vec3) -> Handle<Mesh> {
    meshes.add(shape::Box::new(size.x, size.y, size.z).into())
}

fn build_sphere_mesh(meshes: &mut ResMut<Assets<Mesh>>, radius: f32) -> Handle<Mesh> {
    meshes.add(
        shape::UVSphere {
            radius,
            ..default()
        }
        .into(),
    )
}

fn build_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    color: Color,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        perceptual_roughness: 1.0,
        ..default()
    })
}

fn build_cube(
    position: Vec3,
    rotation: Quat,
    size: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) -> (PbrBundle, Collider) {
    (
        PbrBundle {
            mesh,
            material,
            transform: Transform {
                translation: position,
                rotation,
                ..default()
            },
            ..default()
        },
        Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
    )
}
