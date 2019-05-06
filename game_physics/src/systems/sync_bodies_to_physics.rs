use core::ops::Deref;

use amethyst::{
    core::Transform,
    ecs::{
        storage::{ComponentEvent, MaskedStorage},
        BitSet, Component, Entities, Join, ReadStorage, ReaderId, Resources, Storage, System,
        SystemData, Tracked, WriteExpect, WriteStorage,
    },
};

use crate::{body::PhysicsBody, PhysicsWorld};

/// The `SyncBodiesToPhysicsSystem` handles the synchronisation of `PhysicsBody` `Component`s and
/// their `Transform` values from Amethyst to the `PhysicsWorld` instance.
pub struct SyncBodiesToPhysicsSystem {
    transforms_reader_id: Option<ReaderId<ComponentEvent>>,
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,

    inserted_transforms: BitSet,
    modified_transforms: BitSet,
    inserted_physics_bodies: BitSet,
    modified_physics_bodies: BitSet,
}

impl<'s> System<'s> for SyncBodiesToPhysicsSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform<f32>>,
        ReadStorage<'s, PhysicsBody>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, (entities, transforms, physics_bodies, mut physics_world): Self::SystemData) {
        // clear the BitSets before starting work
        self.clear();

        // get ComponentEvent flags for Transforms and removing deleted ones from the physics world
        trace!("Handling Transform storage events...");
        handle_component_events(
            &transforms,
            self.transforms_reader_id.as_mut().unwrap(),
            &mut self.inserted_transforms,
            &mut self.modified_transforms,
            &mut physics_world,
            &entities,
            &physics_bodies,
        );

        // get ComponentEvent flags for PhysicsBody and removing deleted ones from the physics world
        trace!("Handling PhysicsBody storage events...");
        handle_component_events(
            &transforms,
            self.transforms_reader_id.as_mut().unwrap(),
            &mut self.inserted_transforms,
            &mut self.modified_transforms,
            &mut physics_world,
            &entities,
            &physics_bodies,
        );
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        res.entry::<PhysicsWorld>()
            .or_insert_with(PhysicsWorld::new);

        let mut transform_storage: WriteStorage<Transform<f32>> = SystemData::fetch(&res);
        self.transforms_reader_id = Some(transform_storage.register_reader());

        let mut physics_body_storage: WriteStorage<PhysicsBody> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());
    }
}

impl SyncBodiesToPhysicsSystem {
    /// Creates a new `SyncBodiesToPhysicsSystem` with the given reader IDs.
    pub fn new() -> Self {
        Self {
            transforms_reader_id: None,
            physics_bodies_reader_id: None,
            inserted_transforms: BitSet::new(),
            modified_transforms: BitSet::new(),
            inserted_physics_bodies: BitSet::new(),
            modified_physics_bodies: BitSet::new(),
        }
    }

    /// Clears the `BitSet` instances associated with this `System`.
    fn clear(&mut self) {
        self.inserted_transforms.clear();
        self.modified_transforms.clear();
        self.inserted_physics_bodies.clear();
        self.modified_physics_bodies.clear();
    }
}

/// Generic way of handling multiple types of `Component`s and their `ComponentEvent`s. This keeps
/// track of which IDs were inserted and modified and deletes removed IDs from the `PhysicsWorld`.
fn handle_component_events<T, D>(
    tracked_storage: &Storage<T, D>,
    reader_id: &mut ReaderId<ComponentEvent>,
    inserted: &mut BitSet,
    modified: &mut BitSet,
    physics_world: &mut PhysicsWorld,
    entities: &Entities,
    physics_bodies: &ReadStorage<PhysicsBody>,
) where
    T: Component,
    T::Storage: Tracked,
    D: Deref<Target = MaskedStorage<T>>,
{
    for component_event in tracked_storage.channel().read(reader_id) {
        match component_event {
            ComponentEvent::Inserted(id) => {
                info!("Inserted: {}", id);
                inserted.add(*id);
            }
            ComponentEvent::Modified(id) => {
                info!("Modified: {}", id);
                modified.add(*id);
            }
            ComponentEvent::Removed(id) => match physics_bodies.get(entities.entity(*id)) {
                Some(body) => match body.handle() {
                    Some(handle) => {
                        info!("Removing body with id: {}", id);
                        physics_world.remove_bodies(&[handle]);
                    }
                    None => warn!("Missing handle in body: {}", id),
                },
                None => warn!("Missing body with id: {}", id),
            },
        }
    }
}
