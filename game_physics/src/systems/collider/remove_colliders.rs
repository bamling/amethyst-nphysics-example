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
    collider::{PhysicsCollider, PhysicsColliderHandles},
    systems::removed_components,
    PhysicsWorld,
};

/// The `RemoveCollidersSystem` handles the removal of a `PhysicsCollider`s
///// corresponding `Collider` from the physics `World`. This happens based on
///// `ComponentEvent::Removed` for the `PhysicsCollider` `Component`.
#[derive(Default)]
pub struct RemoveCollidersSystem {
    physics_colliders_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for RemoveCollidersSystem {
    type SystemData = (
        ReadStorage<'s, PhysicsCollider>,
        WriteExpect<'s, PhysicsColliderHandles>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_colliders, mut physics_collider_handles, mut physics_world) = data;

        // iterate over the IDs of all removed PhysicsCollider components; we have to
        // work with Index/id in place of the actual PhysicsCollider as the
        // component itself was already removed and cannot be fetched anymore
        for id in removed_components(
            &physics_colliders,
            self.physics_colliders_reader_id.as_mut().unwrap(),
        ) {
            debug!("Removed PhysicsCollider with id: {}", id);
            if let Some(handle) = physics_collider_handles.remove(&id) {
                // remove body if it still exists in the physics world
                if physics_world.collider(handle).is_some() {
                    physics_world.remove_colliders(&[handle]);
                }

                info!("Removed collider from world with id: {}", id);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("RemoveCollidersSystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());
        res.entry::<PhysicsColliderHandles>()
            .or_insert(PhysicsColliderHandles::new());

        // register reader id for the PhysicsCollider storage
        let mut physics_collider_storage: WriteStorage<PhysicsCollider> = SystemData::fetch(&res);
        self.physics_colliders_reader_id = Some(physics_collider_storage.register_reader());
    }
}
