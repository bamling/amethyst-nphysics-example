use core::ops::Deref;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{
        storage::{ComponentEvent, MaskedStorage},
        BitSet,
        Component,
        DispatcherBuilder,
        ReaderId,
        Storage,
        Tracked,
    },
    error::Error,
};

use self::{
    body::{
        add_rigid_bodies::AddRigidBodiesSystem,
        remove_rigid_bodies::RemoveRigidBodiesSystem,
        update_rigid_bodies::UpdateRigidBodiesSystems,
    },
    collider::{
        add_colliders::AddCollidersSystem,
        remove_colliders::RemoveCollidersSystem,
        update_colliders::UpdateCollidersSystems,
    },
    debug::DebugSystem,
    physics_stepper::PhysicsStepperSystem,
    sync_gravity::SyncGravitySystem,
    sync_positions::SyncPositionsSystem,
};

mod body;
mod collider;
mod debug;
mod physics_stepper;
mod sync_gravity;
mod sync_positions;

/// Bundle containing all `System`s relevant to the game physics.
#[derive(Default)]
pub struct PhysicsBundle {
    debug_lines: bool,
}

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // synchronise PhysicsBody components with the PhysicsWorld
        dispatcher.add(
            AddRigidBodiesSystem::default(),
            "add_rigid_bodies_system",
            &[],
        );
        dispatcher.add(
            UpdateRigidBodiesSystems::default(),
            "update_rigid_bodies_system",
            &["add_rigid_bodies_system"],
        );
        dispatcher.add(
            RemoveRigidBodiesSystem::default(),
            "remove_rigid_bodies_system",
            &["add_rigid_bodies_system"],
        );

        // synchronise PhysicsCollider components with the PhysicsWorld
        dispatcher.add(
            AddCollidersSystem::default(),
            "add_colliders_system",
            &["add_rigid_bodies_system"],
        );
        dispatcher.add(
            UpdateCollidersSystems::default(),
            "update_colliders_system",
            &["add_colliders_system"],
        );
        dispatcher.add(
            RemoveCollidersSystem::default(),
            "remove_colliders_system",
            &["add_colliders_system"],
        );

        // synchronise Gravity with the PhysicsWorld
        dispatcher.add(SyncGravitySystem::default(), "sync_gravity_system", &[]);

        // progress the PhysicsWorld
        dispatcher.add(
            PhysicsStepperSystem::default(),
            "physics_stepper_system",
            &[
                "add_rigid_bodies_system",
                "update_rigid_bodies_system",
                "remove_rigid_bodies_system",
                "add_colliders_system",
                "update_colliders_system",
                "remove_colliders_system",
                "sync_gravity_system",
            ],
        );

        // synchronise updated position from PhysicsWorld with Amethyst
        dispatcher.add(
            SyncPositionsSystem::default(),
            "sync_positions_system",
            &["physics_stepper_system"],
        );

        // enable DebugSystem on demand
        if self.debug_lines {
            dispatcher.add(DebugSystem::default(), "debug_system", &[]);
        }

        Ok(())
    }
}

impl PhysicsBundle {
    /// Enables the `DebugSystem` which draws `DebugLines` around
    /// `PhysicsCollider` shapes.
    pub fn with_debug_lines(mut self) -> Self {
        self.debug_lines = true;
        self
    }
}

/// Iterated over the `ComponentEvent::Inserted`s of a given, tracked `Storage`
/// and returns the results in a `BitSet`.
pub(crate) fn inserted_components<T, D>(
    tracked_storage: &Storage<T, D>,
    reader_id: &mut ReaderId<ComponentEvent>,
) -> BitSet
where
    T: Component,
    T::Storage: Tracked,
    D: Deref<Target = MaskedStorage<T>>,
{
    let mut inserted = BitSet::new();
    for component_event in tracked_storage.channel().read(reader_id) {
        match component_event {
            ComponentEvent::Inserted(id) => {
                debug!("Got Inserted event with id: {}", id);
                inserted.add(*id);
            }
            _ => {}
        }
    }
    inserted
}

/// Iterated over the `ComponentEvent::Modified`s of a given, tracked `Storage`
/// and returns the results in a `BitSet`.
pub(crate) fn modified_components<T, D>(
    tracked_storage: &Storage<T, D>,
    reader_id: &mut ReaderId<ComponentEvent>,
) -> BitSet
where
    T: Component,
    T::Storage: Tracked,
    D: Deref<Target = MaskedStorage<T>>,
{
    let mut modified = BitSet::new();
    for component_event in tracked_storage.channel().read(reader_id) {
        match component_event {
            ComponentEvent::Modified(id) => {
                debug!("Got Modified event with id: {}", id);
                modified.add(*id);
            }
            _ => {}
        }
    }
    modified
}

/// Iterated over the `ComponentEvent::Removed`s of a given, tracked `Storage`
/// and returns the results in a `BitSet`.
pub(crate) fn removed_components<T, D>(
    tracked_storage: &Storage<T, D>,
    reader_id: &mut ReaderId<ComponentEvent>,
) -> BitSet
where
    T: Component,
    T::Storage: Tracked,
    D: Deref<Target = MaskedStorage<T>>,
{
    let mut removed = BitSet::new();
    for component_event in tracked_storage.channel().read(reader_id) {
        match component_event {
            ComponentEvent::Removed(id) => {
                debug!("Got Removed event with id: {}", id);
                removed.add(*id);
            }
            _ => {}
        }
    }
    removed
}
