use bevy::{core_pipeline::bloom::Bloom, prelude::*};
//use bevy::input::*;
//use bevy::transform;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
        app.add_systems(Update, (cam_move, cam_rotate));
    }
}

fn start_camera(mut cmd: Commands){
    cmd.spawn((
        Camera3d::default(),
        Camera {
            hdr: true, // HDR is required for bloom
            ..default()
        },
        Bloom::NATURAL,
        PointLight {
            shadows_enabled: false,
            intensity: 50000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3{x: 1.0, y: 0.0, z: 0.0}, Vec3::Y),
    ));
}

fn cam_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //cam_query: Query<&Camera3d>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
){
    let mut movement = Vec3::ZERO;
    let movespeed = 3.0;

    for mut transform in &mut query {

        // Calculate the forward and right vectors based on the camera's rotation
        let mut forward: Vec3 = transform.forward().into();
        forward.y = 0.0; // Constrain forward movement to the X-Z plane
        forward = forward.normalize();

        let mut back: Vec3 = transform.back().into();
        back.y = 0.0; // Constrain strafing movement to the X-Z plane
        back = back.normalize();

        let mut right: Vec3 = transform.right().into();
        right.y = 0.0; // Constrain strafing movement to the X-Z plane
        right = right.normalize();

        let mut left: Vec3 = transform.left().into();
        left.y = 0.0; // Constrain strafing movement to the X-Z plane
        left = left.normalize();

        if keyboard_input.pressed(KeyCode::KeyW) {
            movement += forward * movespeed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement += back * movespeed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement += left * movespeed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += right * movespeed;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            movement += Vec3::Y * movespeed;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            movement -= Vec3:: Y * movespeed;
        }
    
        transform.translation += movement * time.delta_secs();
    }
}

fn cam_rotate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut yaw = 0.0;   // Rotation around the Y-axis
    let mut pitch = 0.0; // Rotation around the X-axis
    let rotation_speed = 1.5; // Rotation speed (radians per second)

    // Yaw (left and right rotation) using A and D keys
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        yaw += rotation_speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        yaw -= rotation_speed * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        pitch += rotation_speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        pitch -= rotation_speed * time.delta_secs();
    }

    for mut transform in &mut query {
        // Apply yaw (rotation around world Y-axis)
        let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);

        // Update the transform's rotation with yaw first
        transform.rotation = yaw_quat * transform.rotation;

        // Apply pitch (rotation around camera's local X-axis)
        let pitch_quat = Quat::from_rotation_x(pitch);

        // Apply the pitch to the updated rotation
        transform.rotation = transform.rotation * pitch_quat;
    }
}