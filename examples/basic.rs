use bevy::prelude::*;
use bevy_fly_camera::{
	FlyCamera,
	FlyCameraPlugin,
};

// This is a simple example of a camera that flies around.
// There's an included example of a system that toggles the "enabled"
// property of the fly camera with "T"

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(DirectionalLightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
		..Default::default()
	});
	commands
		.spawn(Camera3dBundle::default())
		.insert(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn(PbrBundle {
					mesh: box_mesh.clone(),
					material: box_material.clone(),
					transform: Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
					..Default::default()
				});
			}
		}
	}

	println!("Started example!");
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
	input: Res<Input<KeyCode>>,
	mut query: Query<&mut FlyCamera>,
) {
	for mut options in query.iter_mut() {
		if input.just_pressed(KeyCode::T) {
			println!("Toggled FlyCamera enabled!");
			options.enabled = !options.enabled;
		}
	}
}

fn main() {
	App::new()
		.insert_resource(Msaa::Sample4)
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, init)
		.add_plugins(FlyCameraPlugin)
		.add_systems(Update, toggle_button_system)
		.run();
}
