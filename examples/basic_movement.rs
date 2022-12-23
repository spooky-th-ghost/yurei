use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use yurei::{prelude::*, MovingPlatformDriver};

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YureiPlugin)
        .add_startup_system(setup_world)
        .add_system(player_input)
        .add_system(update_camera_target_position)
        .add_system(lerp_to_camera_position.after(update_camera_target_position))
        .add_system(rotate_camera)
        .run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default());

    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(200.0, 0.2, 200.0))),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(RigidBody::Fixed);

    //Mover

    let patrol_route = vec![Vec3::new(-5.0, 0.5, 0.0), Vec3::new(5.0, 0.5, 0.0)];
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 0.2, 2.0))),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(25.0, 0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(1.0, 0.1, 1.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Velocity::default())
        .insert(MovingPlatformDriver::with_targets(patrol_route));

    // Spawn Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::WHITE.into()),
            ..default()
        })
        .insert(YureiBundle::default())
        .insert(Player);

    // Light
    commands.insert_resource(AmbientLight {
        color: Color::ANTIQUE_WHITE,
        brightness: 0.45,
    });
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
                impulse.impulse = Vec3::Y * 70.0;
            }
        }

        let left_vec: Vec3 = x * left;
        let forward_vec: Vec3 = z * forward;
        let final_vec = left_vec + forward_vec;
        player_movement.direction = final_vec;
    }
}

#[derive(Component)]
pub struct CameraController {
    pub z_distance: f32,
    pub y_distance: f32,
    pub angle: f32,
    pub easing: f32,
    pub target_position: Vec3,
    pub player_position: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        CameraController {
            z_distance: 20.0,
            y_distance: 20.0,
            angle: 0.0,
            easing: 4.0,
            target_position: Vec3::ZERO,
            player_position: Vec3::ZERO,
        }
    }
}

fn update_camera_target_position(
    mut camera_query: Query<&mut CameraController>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera = camera_query.single_mut();
    let player_transform = player_query.single();

    let mut starting_transform = player_transform.clone();
    starting_transform.rotation = Quat::default();
    starting_transform.rotate_y(camera.angle.to_radians());
    let dir = starting_transform.forward().normalize();
    camera.target_position =
        starting_transform.translation + (dir * camera.z_distance) + (Vec3::Y * camera.y_distance);
    camera.player_position = player_transform.translation;
}

fn lerp_to_camera_position(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &CameraController)>,
) {
    for (mut transform, camera_controller) in &mut camera_query {
        let lerped_position = transform.translation.lerp(
            camera_controller.target_position,
            time.delta_seconds() * camera_controller.easing,
        );
        transform.translation = lerped_position;
        transform.look_at(camera_controller.player_position, Vec3::Y);
    }
}

fn rotate_camera(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut CameraController>,
) {
    let mut camera = camera_query.single_mut();

    if keyboard.pressed(KeyCode::Q) {
        camera.angle -= 45.0 * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::E) {
        camera.angle += 45.0 * time.delta_seconds();
    }

    if camera.angle > 360.0 {
        camera.angle -= 360.0;
    }

    if camera.angle < -360.0 {
        camera.angle += 360.0;
    }
}
