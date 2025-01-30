use bevy::{core_pipeline::bloom::Bloom, math::vec3, prelude::*};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_camera);
        app.add_systems(Update, (cam_move, cam_rotate, update_time_display));
    }
}

fn start_camera(mut cmd: Commands){
    cmd.spawn((
        Camera3d::default(),
        Camera {
            hdr: true, // HDR is required for bloom
            ..default()
        },
        //Bloom::NATURAL,  // do we even like having bloom?
        PointLight {
            shadows_enabled: false,
            intensity: 700000.0,
            ..default()
        },
        Transform::from_xyz(crate::MAZE.start_position.x, 0.0, crate::MAZE.start_position.y).looking_at(Vec3{x: 1.0, y: 0.0, z: 0.0}, Vec3::Y),
    ));
    cmd.spawn((
        Text::new("Scoring: \n   Time elapsed: \n   Morality score: \n   Choices made: \n\nMaze data: \n   Generation type: \n   Dimensions: \n   Total questions: \n "),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }
    ));
}

fn update_time_display (
    mut text: Single<&mut Text, With<Text>>,
    time: Res<Time>,
) {
    let morality = crate::MAZE.morality * 100.0;
    let elapsed_time = time.elapsed_secs_f64();
    let choices_made = crate::MAZE.choices_made;
    let gen_type = &crate::MAZE.algorythm;
    let width = crate::MAZE.width;
    let height =  crate::MAZE.height;
    let total_questions = crate::MAZE.questions;

    let display_text: String = format!("Scoring: \n   Time elapsed: {:.2} \n   Morality: {:.1}% \n   Choices made: {choices_made} \n\nMaze data: \n   Generation type: {gen_type} \n   Dimensions: {width} * {height} \n   Total questions: {total_questions}\n", elapsed_time, morality).to_string();
    text.0 = display_text;
}

fn check_for_clipping(position: Vec3) -> bool {
    let mut is_clipping = false;
    for tile in &crate::MAZE.tiles {

        // * hell in a boolean. basically just checks if the camera is trying to go within a 1 wide square of any wall's position
        // also checks vertical and whatnot, but who gives a shit?
        let is_position_valid: bool = (
            tile.is_wall &&
            (position.x - tile.position.x).abs() <= 0.6 &&
            (position.y - 1.0).abs() <= 1.6 &&
            (position.z - tile.position.y).abs() <= 0.6 
            ) | ( 
                (position.y - -0.5).abs() <= 0.1 
            );

        is_clipping = is_position_valid | is_clipping;
    }
    is_clipping
}

fn cam_move (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //cam_query: Query<&Camera3d>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;
    let movespeed = 1.0;

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
        if check_for_clipping(transform.translation) {
            transform.translation = vec3(-1.0, 0.0, -1.0)
        }
        if !check_for_clipping(transform.translation + (movement * time.delta_secs())) {
            transform.translation += movement * time.delta_secs();
        }
    }
}

fn cam_rotate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut yaw = 0.0;   // Rotation around the Y-axis
    let mut pitch = 0.0; // Rotation around the X-axis
    let rotation_speed = 1.0; // Rotation speed (radians per second)

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
