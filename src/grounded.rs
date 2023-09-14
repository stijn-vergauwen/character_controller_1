use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_grounded);
    }
}

/// Holds state on if this entity is currently grounded. It checks this by raycasting down.
///
/// * NOTE: this entity should also have `rigidbody` and `collider` components.
#[derive(Component)]
pub struct Grounded {
    is_grounded: bool,
    check_distance: f32,
    /// The distance that the raycast will be offsetted down, use this when the raycasts hit it's own colliders.
    check_offset: f32,
}

impl Grounded {
    pub fn new(check_distance: f32, check_offset: f32) -> Self {
        Self {
            is_grounded: false,
            check_distance,
            check_offset,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.is_grounded
    }
}

fn update_grounded(
    mut grounded_components: Query<(&mut Grounded, &GlobalTransform, Entity)>,
    rapier_context: Res<RapierContext>,
) {
    for (mut grounded, global_transform, entity) in grounded_components.iter_mut() {
        let translation = global_transform.translation();
        let direction = Vec3::NEG_Y;

        let ray_origin = translation + direction * grounded.check_offset;
        let max_toi = grounded.check_distance;
        let filter = QueryFilter::default().exclude_rigid_body(entity);
        let solid = true;

        grounded.is_grounded = rapier_context
            .cast_ray(ray_origin, direction, max_toi, solid, filter)
            .is_some();
    }
}
