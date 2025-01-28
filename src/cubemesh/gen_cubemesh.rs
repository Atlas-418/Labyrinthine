use bevy::prelude::*;

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
	for tile in &crate::MAZE.tiles {

		let mut material: StandardMaterial =  StandardMaterial{ base_color: Color::WHITE, emissive: LinearRgba::new(0.125, 0.1, 0.1, 1.0), ..default() };
		if tile.illuminated {
			material = StandardMaterial{ base_color: Color::WHITE, emissive: LinearRgba::new(-0.075 , -0.075, -0.075, 1.0), ..default() };
		}

		match tile.is_wall {
			true => {
				cmd.spawn((
					Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
					MeshMaterial3d(materials.add(material)),
					Transform::from_translation(Vec3::new(tile.position.x, 1.0, tile.position.y)),
				));
			}
			
			false => {
				cmd.spawn((
					Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(0.5)))),
					MeshMaterial3d(materials.add(material )),
					Transform::from_translation(Vec3::new(tile.position.x, -0.5, tile.position.y)),
				));
			}
		}
	}
}
