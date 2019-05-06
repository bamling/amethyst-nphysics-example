use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, error::Error};

use self::sync_bodies_to_physics::SyncBodiesToPhysicsSystem;

mod sync_bodies_to_physics;

/// Bundle containing all `System`s relevant to the game physics.
#[derive(Default)]
pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        dispatcher.add(
            SyncBodiesToPhysicsSystem::new(),
            "sync_bodies_to_physics_system",
            &[],
        );

        Ok(())
    }
}
