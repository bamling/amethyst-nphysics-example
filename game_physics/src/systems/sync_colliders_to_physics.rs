use core::ops::Deref;

use amethyst::{
    core::Transform,
    ecs::{
        storage::{ComponentEvent, MaskedStorage},
        BitSet,
        Component,
        Entities,
        ReadStorage,
        ReaderId,
        Resources,
        Storage,
        System,
        SystemData,
        Tracked,
        WriteExpect,
        WriteStorage,
    },
};

use crate::{body::PhysicsBody, collider::Collider, EntityColliderHandles, PhysicsWorld};

/// The `SyncBodiesToPhysicsSystem` handles the synchronisation of `Collider`
/// `Component`s into `PhysicsWorld` instance.
pub struct SyncCollidersToPhysicsSystem {
    colliders_reader_id: Option<ReaderId<ComponentEvent>>,

    inserted_colliders: BitSet,
    modified_colliders: BitSet,
}

impl<'s> System<'s> for SyncCollidersToPhysicsSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform<f32>>,
        ReadStorage<'s, PhysicsBody>,
        WriteExpect<'s, EntityColliderHandles>,
        WriteExpect<'s, PhysicsWorld>,
        WriteStorage<'s, Collider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, transforms, bodies, mut collider_handles, mut world, mut colliders) = data;

        // clear the BitSets before starting work
        self.clear();

        // iterate over Collider storage events, keep track of inserted and modified
        // entries and delete removed entries from the physics world
        trace!("Iterating Collider storage events...");
        iterate_component_events(
            &colliders,
            self.colliders_reader_id.as_mut().unwrap(),
            &mut self.inserted_colliders,
            &mut self.modified_colliders,
            &mut world,
            &entities,
            &mut collider_handles,
        );
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        // TODO: move this somewhere more "global"?
        res.entry::<EntityColliderHandles>()
            .or_insert_with(EntityColliderHandles::new);

        let mut collider_storage: WriteStorage<Collider> = SystemData::fetch(&res);
        self.colliders_reader_id = Some(collider_storage.register_reader());
    }
}

impl SyncCollidersToPhysicsSystem {
    /// Creates a new `SyncCollidersToPhysicsSystem` with initialised `BitSet`s.
    pub fn new() -> Self {
        Self {
            colliders_reader_id: None,
            inserted_colliders: BitSet::new(),
            modified_colliders: BitSet::new(),
        }
    }

    /// Clears the `BitSet` instances associated with this `System`.
    fn clear(&mut self) {
        self.inserted_colliders.clear();
        self.modified_colliders.clear();
    }
}

/// Generic way of handling multiple types of `Component`s and their
/// `ComponentEvent`s. This keeps track of which IDs were inserted and modified
/// and deletes removed IDs from the `PhysicsWorld`.
fn iterate_component_events<T, D>(
    tracked_storage: &Storage<T, D>,
    reader_id: &mut ReaderId<ComponentEvent>,
    inserted: &mut BitSet,
    modified: &mut BitSet,
    world: &mut PhysicsWorld,
    entities: &Entities,
    collider_handles: &mut EntityColliderHandles,
) where
    T: Component,
    T::Storage: Tracked,
    D: Deref<Target = MaskedStorage<T>>,
{
    for component_event in tracked_storage.channel().read(reader_id) {
        match component_event {
            ComponentEvent::Inserted(id) => {
                debug!("Got Inserted event with id: {}", id);
                inserted.add(*id);
            }
            ComponentEvent::Modified(id) => {
                // TODO:
                //debug!("Got Modified event with id: {}", id);
                modified.add(*id);
            }
            ComponentEvent::Removed(id) => {
                debug!("Got Removed event with id: {}", id);
                match collider_handles.remove(&entities.entity(*id)) {
                    Some(handle) => {
                        debug!("Removing collider with id: {}", id);
                        world.remove_colliders(&[handle]);
                    }
                    None => warn!("Missing collider handle with id: {}", id),
                }
            }
        };
    }
}
