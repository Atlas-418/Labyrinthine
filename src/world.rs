use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};
use rand::Rng;

pub struct GenWorldPlugin;
impl Plugin for GenWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_world);
    }
}

#[derive(Default)]
struct Voxel {
    pos: Vec3,
    active: bool,
}

fn add_world(
    mut cmd: Commands,
) {
    let maze = make_maze();
    let world = gen_world(maze);
    
    cmd.spawn((
        Mesh3d(world),
        //MeshMaterial3d(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn gen_world(
    points: Vec2,
) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
        )
}

fn make_maze() -> Vec2 {
    let mut maze = Vec2::new(MAZE_HEIGHT.into(), MAZE_WIDTH.into())

    maze
}
