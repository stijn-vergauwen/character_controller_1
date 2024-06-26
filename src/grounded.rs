use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_grounded,
                draw_grounded_check_gizmos,
                draw_ground_normal_gizmos,
            ),
        );
    }
}

/// Holds state on if this entity is currently grounded. It checks this by raycasting down.
///
/// * NOTE: this entity should also have `rigidbody` and `collider` components.
#[derive(Component)]
pub struct Grounded {
    is_grounded: bool,
    /// The normal direction of the ground if grounded, or none.
    ground_normal: Option<Direction3d>,
    /// The amount that the height of the cast origin will be offsetted, use to finetune position.
    height_offset: f32,
    check_method: CheckMethod,

    /// Visualize the behaviour of this component.
    ///
    /// * draws the ray or shape that is used to set `is_grounded`.
    /// * draws the normal direction of the ground if this entity is grounded.
    draw_gizmos: bool,
}

impl Grounded {
    pub fn new(height_offset: f32, check_method: CheckMethod, draw_gizmos: bool) -> Self {
        Self {
            is_grounded: false,
            ground_normal: None,
            height_offset,
            check_method,
            draw_gizmos,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.is_grounded
    }

    pub fn ground_normal(&self) -> Option<Direction3d> {
        self.ground_normal
    }

    pub fn ground_rotation(&self) -> Option<Quat> {
        Some(ground_normal_as_rotation(self.ground_normal?))
    }
}

#[derive(Clone, Copy)]
pub enum CheckMethod {
    Ray { distance: f32 },
    Sphere { radius: f32 },
}

struct CastInfo {
    origin: Vec3,
    direction: Vec3,
    method: CheckMethod,
}

impl CastInfo {
    fn from_translation(
        translation: Vec3,
        direction: Vec3,
        height_offset: f32,
        method: CheckMethod,
    ) -> Self {
        Self {
            origin: translation + Vec3::Y * height_offset,
            direction,
            method,
        }
    }
}

fn update_grounded(
    mut grounded_components: Query<(&mut Grounded, &GlobalTransform, Entity)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut grounded, global_transform, entity) in grounded_components.iter_mut() {
        let cast_info = CastInfo::from_translation(
            global_transform.translation(),
            Vec3::NEG_Y,
            grounded.height_offset,
            grounded.check_method,
        );
        let filter = QueryFilter::default().exclude_rigid_body(entity);

        let cast_result = get_normal_from_cast(&rapier_context, &cast_info, filter);

        grounded.is_grounded = cast_result.is_some();
        grounded.ground_normal = cast_result.and_then(|result| result.try_into().ok());
    }
}

fn get_normal_from_cast(
    rapier_context: &RapierContext,
    cast_info: &CastInfo,
    filter: QueryFilter,
) -> Option<Vec3> {
    match cast_info.method {
        CheckMethod::Ray { distance } => {
            check_ray_hit_normal(&rapier_context, cast_info, distance, filter)
        }
        CheckMethod::Sphere { radius } => {
            if check_sphere_hit(&rapier_context, cast_info, radius, filter) {
                // This way of getting the normal & hard setting the distance to 10 is not ideal, but I don't expect it to cause problems.
                check_ray_hit_normal(rapier_context, cast_info, 10.0, filter)
            } else {
                None
            }
        }
    }
}

fn check_ray_hit_normal(
    rapier_context: &RapierContext,
    cast_info: &CastInfo,
    distance: f32,
    filter: QueryFilter,
) -> Option<Vec3> {
    rapier_context
        .cast_ray_and_get_normal(
            cast_info.origin,
            cast_info.direction,
            distance,
            true,
            filter,
        )
        .and_then(|hit| Some(hit.1.normal))
}

fn check_sphere_hit(
    rapier_context: &RapierContext,
    cast_info: &CastInfo,
    radius: f32,
    filter: QueryFilter,
) -> bool {
    rapier_context
        .intersection_with_shape(
            cast_info.origin,
            Quat::IDENTITY,
            &Collider::ball(radius),
            filter,
        )
        .is_some()
}

fn draw_grounded_check_gizmos(
    grounded_components: Query<(&Grounded, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (grounded, global_transform) in grounded_components
        .iter()
        .filter(|(grounded, _)| grounded.draw_gizmos)
    {
        let cast_info = CastInfo::from_translation(
            global_transform.translation(),
            Vec3::NEG_Y,
            grounded.height_offset,
            grounded.check_method,
        );

        match cast_info.method {
            CheckMethod::Ray { distance } => {
                gizmos.ray(
                    cast_info.origin,
                    cast_info.direction * distance,
                    Color::BLUE,
                );
            }
            CheckMethod::Sphere { radius } => {
                gizmos.sphere(cast_info.origin, Quat::IDENTITY, radius, Color::BLUE);
            }
        };
    }
}

fn draw_ground_normal_gizmos(
    grounded_components: Query<(&Grounded, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (grounded, global_transform) in grounded_components
        .iter()
        .filter(|(grounded, _)| grounded.draw_gizmos)
    {
        if let Some(normal) = grounded.ground_normal {
            let position = global_transform.translation();

            gizmos.circle(position, normal, 0.6, Color::LIME_GREEN);
            gizmos.ray(position, normal * 3.0, Color::LIME_GREEN);
        }
    }
}

// Utilities

fn looking_towards(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}

/// Returns the ground normal direction as a quaternion where up is the Y axis.
fn ground_normal_as_rotation(normal: Direction3d) -> Quat {
    looking_towards(normal.into(), Vec3::Z)
        * Quat::from_axis_angle(Vec3::X, (-90.0 as f32).to_radians())
}
