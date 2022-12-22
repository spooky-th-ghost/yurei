use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use yurei::prelude::*;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YureiPlugin)
        .add_startup_system(setup_world)
        .add_system(player_input)
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(200.0, 0.2, 200.0))),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Fixed);

    // Spawn Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::WHITE.into()),
            ..default()
        })
        .insert(YureiBundle::default())
        .insert(Player);
}

pub fn player_input(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut ExternalImpulse,
            &mut Movement,
            &Velocity,
            Option<&Grounded>,
        ),
        With<Player>,
    >,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let camera_transform = camera_query.single();

    for (mut impulse, mut player_movement, velocity, is_grounded) in &mut player_query {
        let mut x = 0.0;
        let mut z = 0.0;

        let mut forward = camera_transform.forward();
        forward.y = 0.0;
        forward = forward.normalize();

        let mut left = camera_transform.left();
        left.y = 0.0;
        left = left.normalize();

        if keyboard.pressed(KeyCode::W) {
            z += 1.0;
        }

        if keyboard.pressed(KeyCode::S) {
            z -= 1.0;
        }

        if keyboard.pressed(KeyCode::A) {
            x += 1.0;
        }

        if keyboard.pressed(KeyCode::D) {
            x -= 1.0;
        }

        if keyboard.just_pressed(KeyCode::Space) {
            if let Some(_) = is_grounded {
                //velocity.linvel.y = 0.0;
                impulse.impulse = Vec3::Y * 30.0;
            }
        }

        let left_vec: Vec3 = x * left;
        let forward_vec: Vec3 = z * forward;
        let final_vec = left_vec + forward_vec;
        player_movement.direction = final_vec;
    }
}
