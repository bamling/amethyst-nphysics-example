use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteExpect};
use nphysics::math::Velocity;

use crate::{
    body::{Motion, PhysicsBody},
    PhysicsWorld,
};

/// The `SyncMotionsSystem` synchronises the motion values of an `Entity`, e.g.
/// the velocity with corresponding `RigidBody` entries in the physics `World`.
///
/// `RigidBody`s have to be moved via velocity rather than setting their
/// position/translation directly, as setting these values ignores any kind of
/// collision and allows these bodies to penetrate one another.
#[derive(Default)]
pub struct SyncMotionsSystem;

impl<'s> System<'s> for SyncMotionsSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Motion>,
        ReadStorage<'s, PhysicsBody>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, motions, physics_bodies, mut physics_world) = data;

        // iterate over all entities that have a Motion and RigidBody component
        for (entity, motion, physics_body) in (&entities, &motions, &physics_bodies).join() {
            debug!("Synchronising Motion with id: {}", entity.id());

            let delta_time = physics_world.timestep();
            if let Some(rigid_body) = physics_world.rigid_body_mut(physics_body.handle.unwrap()) {
                rigid_body.set_velocity(Velocity::<f32>::linear(
                    motion.velocity.x / delta_time,
                    motion.velocity.y / delta_time,
                ));

                //info!("Updated velocity for rigid body with id: {}", entity.id());
            }
        }
    }
}
