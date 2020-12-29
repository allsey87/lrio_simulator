use bevy::{prelude::*, render::camera::PerspectiveProjection};
use rapier3d::na::{DMatrix, Vector3};

mod physics;

// Summary https://bevyengine.org/news/bevy-0-4/
// 1. Rendering: WebGL2 is supported, Wgpu/Webgpu is not ready yet
// 2. System stages allow fine grain control regarding what runs in parallel etc
// 3. GLTF support has been improved, TODO: attempt loading the zumo model again?
// 4. Reflection appears to be a very useful feature for semi-automating python bindings for controller and loop functions
// 5. Having local transforms detached from the physics could be useful since it may allow doing collision and ray tracing queries
//    on the bodies while the next physic step is being solved.

// Next
// 1. Add back Pbr components and attempt to render with the default plugins?
// 2. Enable the WebGL2 plugin and attempt to get this code base compiling to WASM
// 3. Attempt to load GLTF in browser?

fn main() {
    App::build()
        // testing
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // testing
        .add_plugin(physics::Plugin)
        .add_startup_system(initialize.system())
        // .add_system(render.system())
        // .set_runner(execute)
        .run();
}

use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;

struct Entity;

fn initialize(commands: &mut Commands,
              mut engine: ResMut<physics::Engine>,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<StandardMaterial>>,
              asset_server: Res<AssetServer>) {
    /* create cube model */
    let cube_body = RigidBodyBuilder::new_dynamic().translation(0.0, 1.0, 0.0).build();
    let cube_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0).build();
    let cube_model = engine.add(cube_body, cube_collider);   
    /* create ground model */
    let ground_heights = DMatrix::from_element(2, 2, 0.0);
    let ground_scale = Vector3::new(0.5, 1.0, 0.5);
    let ground_collider = ColliderBuilder::heightfield(ground_heights, ground_scale).build();
    let ground_body = RigidBodyBuilder::new_static().build();
    let ground_model = engine.add(ground_body, ground_collider);

    //let cube_handle = asset_server.load("models/zumo/zumo.gltf#Mesh0/Primitive0");

    commands
        .spawn_scene(asset_server.load("models/zumo/zumo.gltf"))
        /*
        .spawn((Entity, cube_model)).with_bundle(PbrBundle {
            mesh: cube_handle,
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        */
        .spawn((Entity, ground_model)).with_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 0.25 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(-0.25, 0.5, 0.25)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-0.125, 0.25, 0.125))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            perspective_projection: PerspectiveProjection { 
                fov: std::f32::consts::PI / 4.0,
                near: 0.1,
                far: 100.0,
                aspect_ratio: 1.0,
            },
            ..Default::default()
        });
}

// fn render(engine: Res<physics::Engine>, query: Query<&physics::Model, With<Entity>>) {
//     for model in query.iter() {
//         if let Some(body) = engine.body_set.get(model.body_handle) {
//             eprintln!("{:?}", body.position());
//         }
//     }
// }

// fn execute(mut app: App) {
//     loop {
//         app.update();
//     }
// }
