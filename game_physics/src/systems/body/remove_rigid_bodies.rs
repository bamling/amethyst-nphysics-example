use amethyst::ecs::{
    storage::ComponentEvent,
    ReadStorage,
    ReaderId,
    Resources,
    System,
    SystemData,
    WriteExpect,
    WriteStorage,
};

use crate::{
    body::{PhysicsBody, PhysicsBodyHandles},
    systems::removed_components,
    PhysicsWorld,
};

/// The `RemoveRigidBodiesSystem` handles the removal of a `PhysicsBody`s
/// corresponding `RigidBody` from the physics `World`. This happens based on
/// `ComponentEvent::Removed` for the `PhysicsBody` `Component`.
#[derive(Default)]
pub struct RemoveRigidBodiesSystem {
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for RemoveRigidBodiesSystem {
    type SystemData = (
        ReadStorage<'s, PhysicsBody>,
        WriteExpect<'s, PhysicsBodyHandles>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_bodies, mut physics_body_handles, mut physics_world) = data;

        // iterate over the IDs of all removed PhysicsBody components; we have to work
        // with Index/id in place of the actual PhysicsBody as the component
        // itself was already removed and cannot be fetched anymore
        for id in removed_components(
            &physics_bodies,
            self.physics_bodies_reader_id.as_mut().unwrap(),
        ) {
            debug!("Removed PhysicsBody with id: {}", id);
            if let Some(handle) = physics_body_handles.remove(&id) {
                // remove body if it still exists in the physics world
                physics_world.remove_bodies(&[handle]);
                info!("Removed rigid body from world with id: {}", id);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("RemoveRigidBodiesSystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());
        res.entry::<PhysicsBodyHandles>()
            .or_insert(PhysicsBodyHandles::new());

        // register reader id for the PhysicsBody storage
        let mut physics_body_storage: WriteStorage<PhysicsBody> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());
    }
}
