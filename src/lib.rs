use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
use components::*;

mod systems;
use systems::*;

pub struct YureiPlugin;

impl Plugin for YureiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .register_type::<Hover>()
            .register_type::<Movement>()
            .add_system(handle_hover)
            .add_system(handle_rotation)
            .add_system(handle_movement)
            .add_system(handle_damping);
    }
}

pub mod prelude {
    pub(crate) use crate::Grounded;
    pub(crate) use crate::Hover;
    pub(crate) use crate::Movement;
    pub(crate) use crate::RotationDriver;
    pub(crate) use crate::YureiBundle;
    pub(crate) use crate::YureiPlugin;
}
