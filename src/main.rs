use bevy::prelude::*;

mod physics;

fn main() {
    App::build()
        .add_plugin(physics::Plugin)
        .add_startup_system(initialize.system())
        .add_system(render.system())
        .set_runner(execute)
        .run();
}

use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;

struct Entity;

fn initialize(commands: &mut Commands, mut engine: ResMut<physics::Engine>) {
    let cube_body = RigidBodyBuilder::new_dynamic().translation(0.0, 0.0, 0.0).build();
    let cube_collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0).build();
    let model = engine.add(cube_body, cube_collider);
    commands.spawn((Entity, model));
}

fn render(engine: Res<physics::Engine>, query: Query<&physics::Model, With<Entity>>) {
    for model in query.iter() {
        if let Some(body) = engine.body_set.get(model.body_handle) {
            eprintln!("{:?}", body.position());
        }
    }
}

fn execute(mut app: App) {
    loop {
        app.update();
    }
}
