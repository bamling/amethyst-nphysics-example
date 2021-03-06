use amethyst::{
    core::{Float, Transform},
    ecs::{Join, ReadExpect, ReadStorage, Resources, System, SystemData, WriteStorage},
};
use nalgebra::Isometry3;

use crate::{body::PhysicsBody, PhysicsWorld};

/// The `SyncPositionsSystem` synchronised the updated position of the
/// `RigidBody`s in the `PhysicsWorld` with their Amethyst counterparts. This
/// affects the actual `Transform` `Component` related to the `Entity`.
#[derive(Default)]
pub struct SyncPositionsSystem;

impl<'s> System<'s> for SyncPositionsSystem {
    type SystemData = (
        ReadExpect<'s, PhysicsWorld>,
        ReadStorage<'s, PhysicsBody>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_world, physics_bodies, mut transforms) = data;

        // iterate over all PhysicBody components that also come with a Transform
        for (physics_body, transform) in (&physics_bodies, &mut transforms).join() {
            if let Some(rigid_body) = physics_world.rigid_body(physics_body.handle.unwrap()) {
                let isometry: &Isometry3<f32> = rigid_body.position();

                transform.set_translation_xyz(
                    Float::from(isometry.translation.vector.x),
                    Float::from(isometry.translation.vector.y),
                    Float::from(isometry.translation.vector.z),
                );
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("SyncPositionsSystem.setup");
        Self::SystemData::setup(res);
    }
}
