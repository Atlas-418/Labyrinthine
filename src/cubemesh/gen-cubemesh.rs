use bevy::{
	prelude::*,
	render::{
		mesh::Indices,
		render_asset::RenderAssetUsages,
		render_resource::PrimitiveTopology,
	},
};
use crate::{MAZE_HEIGHT, MAZE_WIDTH};

pub struct GenWorldPlugin;
impl Plugin for GenWorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, add_world);
	}
}

fn add_world(
	mut cmd: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let maze = make_maze();
	let world: Handle<Mesh> = meshes.add(gen_world(maze));
	
	cmd.spawn((
		Mesh3d(world),
		//MeshMaterial3d(),
		Transform::from_xyz(0.0, 0.0, 0.0),
	));
}

fn gen_world(
	points: Vec<Vec<bool>>,
) -> Mesh {
    
}

fn gen_side(
	part: Vec3,
) {
/* 
    * My idea for this one is to have a function that takes the position, with the enum "Side",
    * and then returns the plane, and all of it's neccecary data, so taht can be put into the mesh
*/
}

