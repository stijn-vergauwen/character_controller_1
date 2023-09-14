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
    check_method: CastMethod,
}

impl Grounded {
    pub fn new(height_offset: f32, check_method: CastMethod) -> Self {
        Self {
            is_grounded: false,
            height_offset,
            check_method,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.is_grounded
    }
}

#[derive(Clone, Copy)]
pub enum CastMethod {
    Ray { distance: f32 },
    Sphere { radius: f32 },
    // TODO: add collider type
}

fn update_grounded(
    mut grounded_components: Query<(&mut Grounded, &GlobalTransform, Entity)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut grounded, global_transform, entity) in grounded_components.iter_mut() {
        let translation = global_transform.translation();
        let direction = Vec3::NEG_Y;

        let ray_origin = translation + direction * grounded.height_offset;
        let filter = QueryFilter::default().exclude_rigid_body(entity);

        grounded.is_grounded = match grounded.check_method {
            CastMethod::Ray { distance } => check_ray_hit(
                &rapier_context,
                ray_origin,
                direction,
                distance,
                filter,
            ),
            CastMethod::Sphere { radius } => check_sphere_hit(
                &rapier_context,
                ray_origin,
                direction,
                radius,
                filter,
            ),
        }
    }
}

fn check_ray_hit(
    rapier_context: &Res<RapierContext>,
    origin: Vec3,
    direction: Vec3,
    distance: f32,
    filter: QueryFilter,
) -> bool {
    rapier_context
        .cast_ray(origin, direction, distance, true, filter)
        .is_some()
}

fn check_sphere_hit(
    rapier_context: &Res<RapierContext>,
    origin: Vec3,
    direction: Vec3,
    radius: f32,
    filter: QueryFilter,
) -> bool {
    rapier_context
        .cast_shape(
            origin,
            Quat::IDENTITY,
            direction,
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
    for (grounded, global_transform) in grounded_components.iter_mut() {
        let translation = global_transform.translation();
        let direction = Vec3::NEG_Y;

        let ray_origin = translation + direction * grounded.height_offset;

        match grounded.check_method {
            CastMethod::Ray { distance } => {
                gizmos.ray(ray_origin, direction * distance, Color::BLUE);
            }
            CastMethod::Sphere { radius } => {
                gizmos.sphere(ray_origin, Quat::IDENTITY, radius, Color::BLUE);
            }
        };
    }
}
