use bevy::prelude::*;

#[derive(Component)]
pub struct FirstPersonCamera;

#[derive(Component)]
pub struct ThirdPersonCamera;

pub fn build_first_person_camera() -> (FirstPersonCamera, Camera3dBundle) {
    (FirstPersonCamera, Camera3dBundle::default())
}

pub fn build_third_person_camera(
    distance_from_character: f32,
) -> (ThirdPersonCamera, Camera3dBundle) {
    (
        ThirdPersonCamera,
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::Z * distance_from_character),
            ..default()
        },
    )
}
