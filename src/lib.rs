use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
use components::*;

mod systems;
use systems::*;

pub mod prelude;

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
