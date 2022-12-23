use crate::components::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn handle_hover(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut hover_query: Query<(
        &mut ExternalForce,
        &mut ExternalImpulse,
        &mut Velocity,
        &Transform,
        &Hover,
        Option<&Grounded>,
        Entity,
    )>,
    moving_platform_query: Query<&Velocity, (With<MovingPlatformDriver>, Without<Hover>)>,
) {
    for (
        mut external_force,
        mut external_impulse,
        mut velocity,
        transform,
        hover,
        is_grounded,
        hover_entity,
    ) in &mut hover_query
    {
        let ray_pos = transform.translation;
        let ray_dir = Vec3::Y * -1.0;
        let max_distance = hover.ray_length;
        let solid = true;
        let filter = QueryFilter::exclude_dynamic().exclude_sensors();

        if let Some((entity, intersection)) =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_distance, solid, filter)
        {
            external_force.force.y =
                hover.calculate_spring_force(intersection.toi, velocity.linvel);
            if intersection.toi <= hover.ride_height {
                if let None = is_grounded {
                    commands.entity(hover_entity).insert(Grounded);
                }
            }
            if let Ok(platform_velocity) = moving_platform_query.get(entity) {
                println!("Found a moving platform");
                velocity.linvel += platform_velocity.linvel / 2.0;
            }
        } else {
            external_force.force.y = 0.0;
            if let Some(_) = is_grounded {
                commands.entity(hover_entity).remove::<Grounded>();
            }
        }
    }
}

pub fn handle_damping(mut query: Query<(&mut ExternalForce, &Movement, &Velocity)>) {
    for (mut e_force, movement, velocity) in &mut query {
        let flat_velo = Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z);
        let velo_direction = flat_velo.normalize_or_zero();
        let current_speed = flat_velo.length();
        let drag_force = movement.deceleration * -1.0 * flat_velo;

        let mut final_force = e_force.force;

        if current_speed > 0.0 {
            final_force += drag_force;
        }

        if current_speed > movement.top_speed {
            final_force += drag_force;
        }

        e_force.force = final_force;
    }
}

pub fn handle_movement(mut movement_query: Query<(&mut ExternalForce, &Movement, &Velocity)>) {
    for (mut external_force, movement, velocity) in &mut movement_query {
        let mut flat_direction = movement.direction;
        let mut flat_velo = velocity.linvel;
        flat_direction.y = 0.0;
        flat_velo.y = 0.0;

        let acceleration_to_apply = if flat_velo != Vec3::ZERO && flat_direction != Vec3::ZERO {
            let angle_diff = flat_direction.angle_between(flat_velo).to_degrees();
            if angle_diff > 145.0 {
                movement.acceleration * 4.0
            } else if angle_diff > 90.0 && angle_diff < 145.0 {
                movement.acceleration * 3.0
            } else if angle_diff > 45.0 && angle_diff < 90.0 {
                movement.acceleration * 2.0
            } else {
                movement.acceleration
            }
        } else {
            movement.acceleration
        };

        let force_to_add = movement.direction.normalize_or_zero() * acceleration_to_apply;

        let y_force = external_force.force.y;
        external_force.force = Vec3::new(force_to_add.x, y_force, force_to_add.z);
    }
}

pub fn handle_rotation(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Transform), With<RotationDriver>>,
) {
    for (movement, mut transform) in &mut query {
        if movement.direction != Vec3::ZERO {
            let target = transform.translation - movement.direction;
            transform.look_at(target, Vec3::Y);
        }
    }
}

pub fn handle_moving_platforms(
    time: Res<Time>,
    mut query: Query<(&mut MovingPlatformDriver, &mut Transform)>,
) {
    for (mut moving_platform, mut transform) in &mut query {
        if moving_platform.moving {
            let new_position = transform.translation.lerp(
                moving_platform.current_target_location(),
                time.delta_seconds(),
            );

            transform.translation = new_position;

            moving_platform.update(new_position);
        }
    }
}
