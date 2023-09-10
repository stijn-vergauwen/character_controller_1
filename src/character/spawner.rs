use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Character;

const START_POSITION: Vec3 = Vec3::new(0.0, 1.0, 7.0);
const WALKING_STRENGTH: f32 = 8.0;
const RUNNING_STRENGTH: f32 = 13.0;
const ROTATION_STRENGTH: f32 = 0.0007;
const JUMP_STRENGTH: f32 = 3.0;

const CHARACTER_COLOR: Color = Color::ORANGE;

const BODY_RADIUS: f32 = 0.4;
const BODY_HEIGHT: f32 = 1.6; // Must be larger that body radius * 2
const BODY_STRAIGHT_HEIGHT: f32 = BODY_HEIGHT - BODY_RADIUS * 2.0;

const HEAD_SIZE: f32 = 0.5;
const HEAD_HEIGHT_OFFSET: f32 = 1.0;

pub fn spawn_default_character_with_user_input(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::from("Default character body"),
            PbrBundle {
                mesh: meshes.add(
                    shape::Capsule {
                        depth: BODY_STRAIGHT_HEIGHT,
                        radius: BODY_RADIUS,
                        ..default()
                    }
                    .into(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: CHARACTER_COLOR,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                transform: Transform::from_translation(START_POSITION),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule_y(BODY_STRAIGHT_HEIGHT / 2.0, BODY_RADIUS),
            LockedAxes::ROTATION_LOCKED,
            ExternalForce::default(),
            ExternalImpulse::default(),
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.0,
            },
            Character::new(
                WALKING_STRENGTH,
                RUNNING_STRENGTH,
                JUMP_STRENGTH,
                ROTATION_STRENGTH,
            ),
        ))
        .with_children(|body| {
            // Head
            body.spawn((
                Name::from("Default character head"),
                PbrBundle {
                    mesh: meshes.add(shape::Cube::new(HEAD_SIZE).into()),
                    material: materials.add(StandardMaterial {
                        base_color: CHARACTER_COLOR,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, HEAD_HEIGHT_OFFSET, 0.0),
                    ..default()
                },
            ))
            .with_children(|head| {
                spawn_character_camera(head);
            });
        });
}

fn spawn_character_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        Name::from("Character first-person camera"),
        Camera3dBundle { ..default() },
    ));
}
