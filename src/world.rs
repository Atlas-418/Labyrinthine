use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};
use rand::Rng;

pub struct GenWorldPlugin;
impl Plugin for GenWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gen_world);
    }
}

#[derive(Default)]
struct Voxel {
    pos: Vec3,
    active: bool,
}

fn gen_world(
    mut cmd: Commands,
) {

}

fn make_maze() {
    
}
