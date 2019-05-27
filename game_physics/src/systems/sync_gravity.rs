use crate::{Gravity, PhysicsWorld};
use amethyst::ecs::{ReadExpect, Resources, System, SystemData, WriteExpect};

/// The `SyncGravitySystem` handles the synchronisation of `Gravity`
/// changes to the `PhysicsWorld`.
#[derive(Default)]
pub struct SyncGravitySystem;

impl<'s> System<'s> for SyncGravitySystem {
    type SystemData = (ReadExpect<'s, Gravity>, WriteExpect<'s, PhysicsWorld>);

    fn run(&mut self, (gravity, mut physics_world): Self::SystemData) {
        physics_world.set_gravity(*gravity);
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("SyncGravitySystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<Gravity>()
            .or_insert_with(|| Gravity::new(0.0, 0.0, 0.0));
    }
}
