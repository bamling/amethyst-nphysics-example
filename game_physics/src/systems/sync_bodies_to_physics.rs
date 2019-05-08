use core::ops::Deref;

use amethyst::{
    core::Transform,
    ecs::{
        storage::{ComponentEvent, MaskedStorage},
        BitSet,
        Component,
        Entities,
        Entity,
        Join,
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
use nalgebra::Isometry2;
use nphysics::object::{Body, RigidBodyDesc};

use crate::{body::PhysicsBody, EntityBodyHandles, PhysicsWorld};

/// `SyncBodiesToPhysicsSystem` collects `PhysicsBody` `Component`s and their
/// corresponding `Transform`s and synchronises their combined values (as so
/// called `RigidBody`s) into the `PhysicsWorld`.
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
        WriteExpect<'s, EntityBodyHandles>,
        WriteExpect<'s, PhysicsWorld>,
        WriteStorage<'s, PhysicsBody>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, transforms, mut body_handles, mut world, mut bodies) = data;

        // clear the BitSets before starting work
        self.clear();

        // iterate over Transform storage events, keep track of inserted and modified
        // entries and delete removed entries from the physics world
        trace!("Iterating Transform storage events...");
        iterate_component_events(
            &transforms,
            self.transforms_reader_id.as_mut().unwrap(),
            &mut self.inserted_transforms,
            &mut self.modified_transforms,
            &mut world,
            &entities,
            &mut body_handles,
        );

        // iterate over PhysicsBody storage events, keep track of inserted and modified
        // entries and delete removed entries from the physics world
        trace!("Iterating PhysicsBody storage events...");
        iterate_component_events(
            &bodies,
            self.physics_bodies_reader_id.as_mut().unwrap(),
            &mut self.inserted_physics_bodies,
            &mut self.modified_physics_bodies,
            &mut world,
            &entities,
            &mut body_handles,
        );

        // update physics world with the value of components flagged as inserted or
        // modified
        for (entity, transform, body, id) in (
            &entities,
            &transforms,
            &mut bodies,
            &self.inserted_transforms
                | &self.inserted_physics_bodies
                | &self.modified_transforms
                | &self.modified_physics_bodies,
        )
            .join()
        {
            // check if components were inserted, then insert the new elements in the
            // PhysicsWorld
            if self.inserted_transforms.contains(id) || self.inserted_physics_bodies.contains(id) {
                info!("Detected inserted physics body with id {}", id);
                add_rigid_body_to_physics(entity, body, transform, &mut body_handles, &mut world);
            }

            // check if components were modified, then modify the elements in the
            // PhysicsWorld
            if self.modified_transforms.contains(id) || self.modified_physics_bodies.contains(id) {
                // TODO: see https://github.com/amethyst/amethyst/issues/1563
                //info!("Detected modified physics body with id {}", id);
                update_rigid_body_in_physics(body, transform, &mut world);
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        // TODO: move these somewhere more "global"?
        res.entry::<PhysicsWorld>()
            .or_insert_with(PhysicsWorld::new);
        res.entry::<EntityBodyHandles>()
            .or_insert_with(EntityBodyHandles::new);

        let mut transform_storage: WriteStorage<Transform<f32>> = SystemData::fetch(&res);
        self.transforms_reader_id = Some(transform_storage.register_reader());

        let mut physics_body_storage: WriteStorage<PhysicsBody> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());
    }
}

impl SyncBodiesToPhysicsSystem {
    /// Creates a new `SyncBodiesToPhysicsSystem` with initialised `BitSet`s.
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
    body_handles: &mut EntityBodyHandles,
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
                match body_handles.remove(&entities.entity(*id)) {
                    Some(handle) => {
                        debug!("Removing physics body with id: {}", id);
                        world.remove_bodies(&[handle]);
                    }
                    None => warn!("Missing body handle with id: {}", id),
                }
            }
        }
    }
}

/// Helper function for adding new rigid body elements to the `PhysicsWorld`
/// instance. If a `EntityBodyHandles` entry already exists for said body
/// element it is removed before insertion. Normally this should not happen as
/// we make sure to clean up the list of `EntityBodyHandles` for removals.
fn add_rigid_body_to_physics(
    entity: Entity,
    body: &mut PhysicsBody,
    transform: &Transform<f32>,
    body_handles: &mut EntityBodyHandles,
    world: &mut PhysicsWorld,
) {
    // remove already existing bodies for this inserted event
    if let Some(handle) = body_handles.remove(&entity) {
        info!("Removing inserted body that already exists: {:?}", handle);
        world.remove_bodies(&[handle]);
    }

    // add new RigidBodyDesc to PhysicsWorld and keep handle for later use
    let handle = RigidBodyDesc::new()
        // ignore Z axis since we're simulating a 2D world without depth
        .position(Isometry2::translation(
            transform.translation().x,
            transform.translation().y,
        ))
        .status(body.body_status)
        .velocity(body.velocity)
        .user_data(entity)
        .build(world)
        .handle();

    body.handle = Some(handle.clone());
    body_handles.insert(entity, handle);

    info!("Inserted rigid body to world with values: {:?}", body);
}

/// Helper function for updating existing rigid body elements in the
/// `PhysicsWorld`. The updates are based on the `Components `Transform` and
/// `PhysicsBody`.
fn update_rigid_body_in_physics(
    body: &mut PhysicsBody,
    transform: &Transform<f32>,
    world: &mut PhysicsWorld,
) {
    if let Some(rigid_body) = world.rigid_body_mut(body.handle().unwrap()) {
        let position = Isometry2::translation(transform.translation().x, transform.translation().y);
        trace!(
            "Updating rigid body in physics world with position: {}",
            position
        );
        rigid_body.set_position(position);
        rigid_body.set_velocity(body.velocity);
        rigid_body.set_status(body.body_status);
    }
}
