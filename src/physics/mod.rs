use rapier3d::na::Vector3;
use rapier3d::dynamics::{JointSet, RigidBody, RigidBodySet, IntegrationParameters};
use rapier3d::geometry::{BroadPhase, NarrowPhase, Collider, ColliderSet};
use rapier3d::pipeline::PhysicsPipeline;
use rapier3d::data::arena::Index;

use bevy::prelude::*;

pub struct Model {
    pub body_handle: Index,
    collider_handle: Index,
}

pub struct Engine {
    pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    gravity: Vector3<f32>,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub body_set: RigidBodySet,
    collider_set: ColliderSet,
    joint_set: JointSet,
    event_handler: (),
}

impl Engine {
    fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            joint_set: JointSet::new(),
            event_handler: (),
        }
    }

    pub fn add(&mut self, body: RigidBody, collider: Collider) -> Model {
        let body_handle = self.body_set.insert(body);
        let collider_handle = self.collider_set.insert(collider, body_handle, &mut self.body_set);
        Model { body_handle, collider_handle }
    }

    fn tick(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            None,
            None,
            &self.event_handler);
    }
}

fn tick(mut engine: ResMut<Engine>) {
    engine.tick();
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, simulator: &mut AppBuilder) {
        simulator.add_resource(Engine::new());
        simulator.add_system(tick.system());
    }
}
