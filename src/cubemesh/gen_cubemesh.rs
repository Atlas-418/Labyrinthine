use bevy::prelude::*;
use crate::maze::*;

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
	let maze: Maze = Maze::make_maze(MazeAlgorithm::HuntAndKill);
	for tile in maze.tiles {
		match tile.is_wall {
			true => {
				cmd.spawn((
					Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
					MeshMaterial3d(materials.add(Color::WHITE)),
					Transform::from_translation(Vec3::new(tile.position.x, 1.0, tile.position.y)),
				));
			}
			false => {
				if tile.illuminated {
					cmd.spawn((
						Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
						MeshMaterial3d(materials.add(StandardMaterial {
							emissive: LinearRgba::rgb(2.0, 12.0, 15.0),
							..default()
						})),
						Transform::from_translation(Vec3::new(tile.position.x, -0.5, tile.position.y)),
					));
				}
				else {
					cmd.spawn((
						Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
						MeshMaterial3d(materials.add(Color::WHITE)),
						Transform::from_translation(Vec3::new(tile.position.x, -0.5, tile.position.y)),
					));
				}
			}
		}
	}
}