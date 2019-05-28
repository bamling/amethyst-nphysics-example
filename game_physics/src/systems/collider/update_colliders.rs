use amethyst::ecs::{
    storage::ComponentEvent,
    Join,
    ReadStorage,
    ReaderId,
    Resources,
    System,
    SystemData,
    WriteExpect,
    WriteStorage,
};

use crate::{collider::PhysicsCollider, systems::modified_components, PhysicsWorld};

/// The `UpdateCollidersSystems` the synchronisation of updated
/// `PhysicsCollider` `Component`s with their `PhysicsWorld` counterparts. This
/// happens based on `ComponentEvent::Modified` for the `PhysicsCollider`
/// `Component`.
#[derive(Default)]
pub struct UpdateCollidersSystems {
    physics_colliders_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for UpdateCollidersSystems {
    type SystemData = (
        ReadStorage<'s, PhysicsCollider>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_colliders, mut physics_world) = data;

        // collect all modified PhysicsCollider components
        let modified_physics_colliders = modified_components(
            &physics_colliders,
            self.physics_colliders_reader_id.as_mut().unwrap(),
        );

        // iterate over all modified PhysicsCollider components
        for (physics_collider, id) in (&physics_colliders, &modified_physics_colliders).join() {
            debug!("Modified PhysicsCollider with id: {}", id);
            let collider_handle = physics_collider.handle.unwrap();
            let collider_world = physics_world.collider_world_mut();

            // update collision groups
            collider_world
                .set_collision_groups(collider_handle.clone(), physics_collider.collision_group);

            trace!(
                "Updated collider in world with values: {:?}",
                physics_collider
            );
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("UpdateCollidersSystems.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());

        // register reader id for the PhysicsCollider storage
        let mut physics_collider_storage: WriteStorage<PhysicsCollider> = SystemData::fetch(&res);
        self.physics_colliders_reader_id = Some(physics_collider_storage.register_reader());
    }
}
