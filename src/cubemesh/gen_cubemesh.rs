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
	for (row_i, row) in maze.iter().enumerate() {
		for (spot_i, spot) in row.iter().enumerate() {
			if *spot {
				cmd.spawn((
					Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
					MeshMaterial3d(materials.add(Color::WHITE)),
					Transform::from_xyz(spot_i as f32, 0.0, row_i as f32),
				));
			}
			else {
				cmd.spawn((
					Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
					MeshMaterial3d(materials.add(Color::WHITE)),
					Transform::from_xyz(spot_i as f32, -0.5, row_i as f32),
				));
				cmd.spawn((
					PointLight {
						shadows_enabled: false,
						intensity: 1500.0,
						..default()
					},
					Transform::from_xyz(spot_i as f32, 0.0, row_i as f32),
				));
			}
		}
	}
}