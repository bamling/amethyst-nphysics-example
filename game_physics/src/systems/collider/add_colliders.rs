use amethyst::{
    core::{transform::Transform, Parent},
    ecs::{
        storage::ComponentEvent,
        Entities,
        Join,
        ReadExpect,
        ReadStorage,
        ReaderId,
        Resources,
        System,
        SystemData,
        WriteExpect,
        WriteStorage,
    },
};
use nalgebra::Vector3;
use nphysics::object::{BodyPartHandle, ColliderDesc};

use crate::{
    body::PhysicsBodyHandles,
    collider::{PhysicsCollider, PhysicsColliderHandles},
    systems::inserted_components,
    PhysicsWorld,
};

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
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, PhysicsColliderHandles>,
        WriteExpect<'s, PhysicsWorld>,
        WriteStorage<'s, PhysicsCollider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            physics_body_handles,
            parent_entities,
            transforms,
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
        for (parent_entity, transform, mut physics_collider, id) in (
            parent_entities.maybe(),
            &transforms,
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

            // attempt to find the parent BodyPartHandle based on stored BodyHandles for the
            // given Entity/Index
            let parent_part_handle = match physics_body_handles.get(&id) {
                Some(parent_handle) => physics_world
                    .rigid_body(*parent_handle)
                    .map_or(BodyPartHandle::ground(), |body| body.part_handle()),
                None => {
                    // if BodyHandle was found for the current Entity/Index, check for a potential
                    // parent Entity and repeat the first step
                    if let Some(parent_entity) = parent_entity {
                        match physics_body_handles.get(&parent_entity.entity.id()) {
                            Some(parent_handle) => physics_world
                                .rigid_body(*parent_handle)
                                .map_or(BodyPartHandle::ground(), |body| body.part_handle()),
                            None => {
                                // ultimately default to BodyPartHandle::ground()
                                BodyPartHandle::ground()
                            }
                        }
                    } else {
                        // no parent Entity exists, default to BodyPartHandle::ground()
                        BodyPartHandle::ground()
                    }
                }
            };

            // translation based on parent handle
            let translation = if parent_part_handle.is_ground() {
                let (offset_x, offset_y, offset_z) = (
                    physics_collider.offset_from_parent.translation.vector.x,
                    physics_collider.offset_from_parent.translation.vector.y,
                    physics_collider.offset_from_parent.translation.vector.z,
                );

                Vector3::<f32>::new(
                    transform.translation().x.as_f32() + offset_x,
                    transform.translation().y.as_f32() + offset_y,
                    transform.translation().z.as_f32() + offset_z,
                )
            } else {
                physics_collider.offset_from_parent.translation.vector
            };

            // create the actual Collider in the PhysicsWorld and fetch its handle
            let handle = ColliderDesc::new(physics_collider.shape_handle())
                .translation(translation)
                .density(physics_collider.density)
                .material(physics_collider.material.clone())
                .margin(physics_collider.margin)
                .collision_groups(physics_collider.collision_group)
                .linear_prediction(physics_collider.linear_prediction)
                .angular_prediction(physics_collider.angular_prediction)
                .sensor(physics_collider.sensor)
                .user_data(entities.entity(id))
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
