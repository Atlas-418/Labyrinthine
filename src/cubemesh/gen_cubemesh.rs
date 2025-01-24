use bevy::prelude::*;
use crate::maze::make_maze;

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
	for tile in maze {
		match tile.is_wall {
			true => {
				cmd.spawn((
					Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
					MeshMaterial3d(materials.add(Color::WHITE)),
					Transform::from_xyz(tile.x, 0.0, tile.z),
				));
			}
			false => {
				if tile.is_path {
					cmd.spawn((
						Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
						MeshMaterial3d(materials.add(StandardMaterial {
							emissive: LinearRgba::rgb(2.0, 12.0, 15.0),
							..default()
						})),
						Transform::from_xyz(tile.x, -0.5, tile.z),
					));
				}
				else {
					cmd.spawn((
						Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
						MeshMaterial3d(materials.add(Color::WHITE)),
						Transform::from_xyz(tile.x, -0.5, tile.z),
					));
				}
			}
		}
	}
}