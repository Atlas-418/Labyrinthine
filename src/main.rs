use bevy::prelude::*;

/* --TODO--  A very descriptive list of what the fuck I need to do
 * Display something to a window
 * Display 3D space to a window
 * Make camera at reasonable height in 3d space
 * Move camera with WASD
 * Make a wall
 * Make the camera not phase through the wall
 * Make walls automateable
 * Generate map
 * Put walls according to map

 * I'll come up with more steps later, but as of right now, this is what I need to do.

 * Remember to comment any behaviour I need to remember
 */

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
/*
fn do_something(mut commands: Commands) {
    commands.spawn().insert(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(Sprite {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        ..Default::default()
    });
}
*/

pub struct testing;
impl Plugin for tesing {
fn setup(mut cmd: Commands) {
        // Add a 2D camera to the scene
        cmd.spawn().insert_bundle(OrthographicCameraBundle::new_2d());

        // Spawn a colored square entity
        cmd.spawn().insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED, // Set the color of the sprite
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
