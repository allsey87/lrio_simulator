use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::na::{DMatrix, Vector3};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    let cube_body = RigidBodyBuilder::new_dynamic().translation(0.0, 2.0, 0.0);
    let cube_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0);

    let ground_heights = DMatrix::from_element(2, 2, 0.0);
    let ground_scale = Vector3::new(5.0, 1.0, 5.0);
    let ground_collider = ColliderBuilder::heightfield(ground_heights, ground_scale);
    let ground_body = RigidBodyBuilder::new_static();
    
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .with_bundle((ground_body, ground_collider))
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
            ..Default::default()
        })
        .with_bundle((cube_body, cube_collider))
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}