use amethyst::ecs::{Read, Resources, System, SystemData, WriteExpect};

use crate::PhysicsWorld;

/// The `PhysicsStepperSystem` progresses the `PhysicsWorld` by calling:
/// ```rust,ignore
/// physics_world.step();
/// ```
///
/// This `System` has to be executed after any `Motion`, `Gravity`,
/// `PhysicsBody` or `PhysicsCollider` related `System`s.
#[derive(Default)]
pub struct PhysicsStepperSystem;

impl<'s> System<'s> for PhysicsStepperSystem {
    type SystemData = WriteExpect<'s, PhysicsWorld>;

    fn run(&mut self, (time, mut physics_world): Self::SystemData) {
        physics_world.step();

        // print collisions for debug purposes
        let collision_world = physics_world.collider_world();
        collision_world.contact_events().iter().for_each(|event| {
            info!("Got Contact: {:?}", event);
        });

        collision_world.proximity_events().iter().for_each(|event| {
            info!("Got Proximity: {:?}", event);
        });
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("PhysicsStepperSystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());
    }
}
