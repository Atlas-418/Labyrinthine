use bevy::prelude::*;

#[derive(component)]
struct Position {   // make a struct for position. will be used in ECS
    x: f32,
    y: f32,
}

fn main() {
    App::new().add_systems(Update, hello_world).run()
}

fn hello_world() {
    println!("Hello, world!")
}
