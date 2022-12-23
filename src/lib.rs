use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod components;
pub use components::*;

pub mod systems;
pub use systems::*;

pub struct YureiPlugin;

impl Plugin for YureiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::Y * -30.0,
                ..default()
            })
            .register_type::<Hover>()
            .register_type::<Movement>()
            .add_system(handle_hover)
            .add_system(handle_rotation)
            .add_system(handle_moving_platforms)
            .add_system(handle_movement)
            .add_system(handle_damping.after(handle_movement));
    }
}

pub mod prelude {
    pub use crate::Grounded;
    pub use crate::Hover;
    pub use crate::Movement;
    pub use crate::RotationDriver;
    pub use crate::YureiBundle;
    pub use crate::YureiPlugin;
}
