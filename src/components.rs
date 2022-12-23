use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Marker component that indicates if a character is currently on the ground
#[derive(Component)]
pub struct Grounded;

/// Component attached to any character that want's to move itself towards a target using forces
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Movement {
    pub goal_velocity: Vec3,
    pub direction: Vec3,
    pub acceleration: f32,
    pub top_speed: f32,
    pub deceleration: f32,
}

/// Marker that indicates a body should rotate based on the direction of an attached [Movement]
/// component
#[derive(Component)]
pub struct RotationDriver;

/// Component that controls making characters hover slightly above the ground
#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Hover {
    pub ray_length: f32,
    pub ride_height: f32,
    pub strength: f32,
    pub damper: f32,
}

impl Hover {
    pub fn calculate_spring_force(&self, distance: f32, linear_velocity: Vec3) -> f32 {
        let ray_direction = Vec3::Y * -1.0;
        let ray_direction_velocity = ray_direction.dot(linear_velocity);
        let opposite_relative = ray_direction.dot(Vec3::ZERO);
        let relative_velocity = ray_direction_velocity - opposite_relative;
        let force_direction = distance - self.ride_height;
        let up_force = force_direction * self.strength;
        let damping_force = relative_velocity * self.damper;
        let spring_force = up_force - damping_force;
        spring_force * -1.0
    }
}

impl Default for Hover {
    fn default() -> Self {
        Hover {
            ray_length: 4.0,
            ride_height: 2.8,
            strength: 900.0,
            damper: 60.0,
        }
    }
}

#[derive(Component)]
pub struct MovingPlatformDriver {
    pub targets: Vec<Vec3>,
    pub target_index: usize,
    pub distance_threshhold: f32,
    pub moving: bool,
}

impl MovingPlatformDriver {
    pub fn with_targets(targets: Vec<Vec3>) -> Self {
        MovingPlatformDriver {
            targets,
            ..default()
        }
    }

    pub fn current_target_location(&self) -> Vec3 {
        self.targets[self.target_index]
    }

    pub fn update(&mut self, current_position: Vec3) {
        if self.should_cycle(current_position) {
            self.cycle_targets();
        }
    }

    fn should_cycle(&self, current_position: Vec3) -> bool {
        self.distance_to_target(current_position) < self.distance_threshhold
    }

    fn distance_to_target(&self, current_position: Vec3) -> f32 {
        self.targets[self.target_index].distance(current_position)
    }

    fn cycle_targets(&mut self) {
        let new_index = self.target_index + 1;
        if new_index > self.targets.len() - 1 {
            self.target_index = 0;
        } else {
            self.target_index = new_index;
        }
    }
}

impl Default for MovingPlatformDriver {
    fn default() -> Self {
        MovingPlatformDriver {
            targets: Vec::new(),
            target_index: 0,
            distance_threshhold: 0.2,
            moving: true,
        }
    }
}

/// Bundle containing all of the necessary components to allow force based movement/rotation for a
/// character
#[derive(Bundle)]
pub struct YureiBundle {
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub locked_axes: LockedAxes,
    pub hover: Hover,
    pub movement: Movement,
    pub collider: Collider,
    pub transform: Transform,
    pub rotation_driver: RotationDriver,
}

impl YureiBundle {
    pub fn capsule_with_position(position: Vec3) -> Self {
        YureiBundle {
            transform: Transform::from_xyz(position.x, position.y, position.x),
            ..default()
        }
    }
}

impl Default for YureiBundle {
    fn default() -> Self {
        YureiBundle {
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            hover: Hover::default(),
            movement: Movement {
                goal_velocity: Vec3::ZERO,
                direction: Vec3::ZERO,
                acceleration: 125.0,
                deceleration: 10.0,
                top_speed: 125.0,
            },
            rotation_driver: RotationDriver,
            collider: Collider::capsule_y(0.5, 0.5),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}
