use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_grounded, draw_grounded_check_gizmos));
    }
}

/// Holds state on if this entity is currently grounded. It checks this by raycasting down.
///
/// * NOTE: this entity should also have `rigidbody` and `collider` components.
#[derive(Component)]
pub struct Grounded {
    is_grounded: bool,
    /// The amount that the height of the cast origin will be offsetted, use to finetune position.
    height_offset: f32,
    check_method: CheckMethod,
    draw_gizmos: bool,
}

impl Grounded {
    pub fn new(height_offset: f32, check_method: CheckMethod, draw_gizmos: bool) -> Self {
        Self {
            is_grounded: false,
            height_offset,
            check_method,
            draw_gizmos,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.is_grounded
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
}

impl CastInfo {
    fn from_translation(translation: Vec3, direction: Vec3, height_offset: f32) -> Self {
        Self {
            origin: translation + Vec3::Y * height_offset,
            direction,
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
        );
        let filter = QueryFilter::default().exclude_rigid_body(entity);

        grounded.is_grounded = match grounded.check_method {
            CheckMethod::Ray { distance } => {
                check_ray_hit(&rapier_context, cast_info, distance, filter)
            }
            CheckMethod::Sphere { radius } => {
                check_sphere_hit(&rapier_context, cast_info, radius, filter)
            }
        }
    }
}

fn check_ray_hit(
    rapier_context: &Res<RapierContext>,
    cast_info: CastInfo,
    distance: f32,
    filter: QueryFilter,
) -> bool {
    rapier_context
        .cast_ray(
            cast_info.origin,
            cast_info.direction,
            distance,
            true,
            filter,
        )
        .is_some()
}

fn check_sphere_hit(
    rapier_context: &Res<RapierContext>,
    cast_info: CastInfo,
    radius: f32,
    filter: QueryFilter,
) -> bool {
    rapier_context
        .cast_shape(
            cast_info.origin,
            Quat::IDENTITY,
            cast_info.direction,
            &Collider::ball(radius),
            0.0,
            filter,
        )
        .is_some()
}

fn draw_grounded_check_gizmos(
    mut grounded_components: Query<(&mut Grounded, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (grounded, global_transform) in grounded_components
        .iter_mut()
        .filter(|(grounded, _)| grounded.draw_gizmos)
    {
        let cast_info = CastInfo::from_translation(
            global_transform.translation(),
            Vec3::NEG_Y,
            grounded.height_offset,
        );

        match grounded.check_method {
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
