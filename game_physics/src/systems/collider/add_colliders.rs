use amethyst::ecs::{
    storage::ComponentEvent,
    Entities,
    Join,
    ReadExpect,
    ReaderId,
    Resources,
    System,
    SystemData,
    WriteExpect,
    WriteStorage,
};

use crate::{
    body::PhysicsBodyHandles,
    collider::{PhysicsCollider, PhysicsColliderHandles},
    systems::inserted_components,
    PhysicsWorld,
};

use nphysics::object::{BodyHandle, BodyPartHandle, ColliderDesc};

/// The `AddCollidersSystem` handles the creation of new `Collider`s in the
/// `PhysicsWorld` instance based on inserted `ComponentEvent`s for the
/// `PhysicsCollider` `Component`.
#[derive(Default)]
pub struct AddCollidersSystem {
    physics_colliders_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for AddCollidersSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, PhysicsBodyHandles>,
        WriteExpect<'s, PhysicsColliderHandles>,
        WriteExpect<'s, PhysicsWorld>,
        WriteStorage<'s, PhysicsCollider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            physics_body_handles,
            mut physics_collider_handles,
            mut physics_world,
            mut physics_colliders,
        ) = data;

        // collect all inserted PhysicsCollider components
        let inserted_physics_colliders = inserted_components(
            &physics_colliders,
            self.physics_colliders_reader_id.as_mut().unwrap(),
        );

        // iterate over inserted PhysicsCollider components and their entities; the
        // entity is used as user data in the Collider creation
        for (entity, mut physics_collider, id) in (
            &entities,
            &mut physics_colliders,
            &inserted_physics_colliders,
        )
            .join()
        {
            // remove already existing colliders for this inserted event
            if let Some(handle) = physics_collider_handles.remove(&id) {
                warn!("Removing orphaned collider handle: {:?}", handle);
                physics_world.remove_colliders(&[handle]);
            }

            // attempt to find the parents handle; default to BodyHandle::ground()
            let parent_handle = physics_body_handles
                .get(&id)
                .map_or(BodyHandle::ground(), |handle| *handle);

            // attempt to find the actual RigidBody from the PhysicsWorld and
            // fetch its BodyPartHandle; if no RigidBody is found, default to
            // BodyPartHandle::ground()
            let parent_part_handle = physics_world
                .rigid_body(parent_handle)
                .map(|body| body.part_handle())
                .unwrap_or(BodyPartHandle::ground());

            // TODO: is this still relevant?
            //let position = if parent.is_ground() {
            //    tr.isometry() * collider.offset_from_parent
            //} else {
            //    collider.offset_from_parent
            //};
            let handle = ColliderDesc::new(physics_collider.shape.handle())
                .translation(physics_collider.offset_from_parent.translation.vector)
                .density(physics_collider.density)
                .material(physics_collider.material.clone())
                .margin(physics_collider.margin)
                .collision_groups(physics_collider.collision_group)
                .linear_prediction(physics_collider.linear_prediction)
                .angular_prediction(physics_collider.angular_prediction)
                .sensor(physics_collider.sensor)
                .user_data(entity)
                .build_with_parent(parent_part_handle, &mut physics_world)
                .unwrap()
                .handle();

            physics_collider.handle = Some(handle.clone());
            physics_collider_handles.insert(id, handle);

            info!(
                "Inserted collider to world with values: {:?}",
                physics_collider
            );
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("AddCollidersSystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());
        res.entry::<PhysicsBodyHandles>()
            .or_insert(PhysicsBodyHandles::new());
        res.entry::<PhysicsColliderHandles>()
            .or_insert(PhysicsColliderHandles::new());

        // register reader id for the PhysicsCollider storage
        let mut physics_collider_storage: WriteStorage<PhysicsCollider> = SystemData::fetch(&res);
        self.physics_colliders_reader_id = Some(physics_collider_storage.register_reader());
    }
}
