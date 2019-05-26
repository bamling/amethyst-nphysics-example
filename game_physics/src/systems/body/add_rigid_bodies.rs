use amethyst::{
    core::Transform,
    ecs::{
        storage::ComponentEvent,
        Entities,
        Join,
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
use nphysics::{math::Velocity, object::RigidBodyDesc};

use crate::{
    body::{PhysicsBody, PhysicsBodyHandles},
    systems::inserted_components,
    PhysicsWorld,
};

/// The `AddRigidBodiesSystem` handles the creation of new `RigidBody`s in the
/// `PhysicsWorld` instance based on inserted `ComponentEvent`s for the
/// `PhysicsBody` `Component`. A `RigidBody` can only be created if the `Entity`
/// that belongs to the `PhysicsBody` also contains a `Transform` `Component`.
#[derive(Default)]
pub struct AddRigidBodiesSystem {
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for AddRigidBodiesSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, PhysicsBodyHandles>,
        WriteExpect<'s, PhysicsWorld>,
        WriteStorage<'s, PhysicsBody>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, transforms, mut physics_body_handles, mut physics_world, mut physics_bodies) =
            data;

        // collect all inserted PhysicsBody components
        let inserted_physics_bodies = inserted_components(
            &physics_bodies,
            self.physics_bodies_reader_id.as_mut().unwrap(),
        );

        // iterate over inserted PhysicBody components for entities that also have a
        // Transform component; others are ignored as they cannot be positioned
        // in the PhysicsWorld
        for (entity, mut physics_body, transform, id) in (
            &entities,
            &mut physics_bodies,
            &transforms,
            &inserted_physics_bodies,
        )
            .join()
        {
            debug!("Inserted PhysicsBody with id: {}", id);
            // remove already existing bodies for this inserted component;
            // this technically should never happen but we need to keep the list of body
            // handles clean
            if let Some(handle) = physics_body_handles.remove(&id) {
                warn!("Removing orphaned body handle: {:?}", handle);
                physics_world.remove_bodies(&[handle]);
            }

            let delta_time = physics_world.timestep();

            // create a new RigidBody in the PhysicsWorld and store its
            // handle for later usage
            let handle = RigidBodyDesc::new()
                .translation(Vector3::new(
                    transform.translation().x.as_f32(),
                    transform.translation().y.as_f32(),
                    transform.translation().z.as_f32(),
                ))
                .gravity_enabled(physics_body.gravity_enabled)
                .status(physics_body.body_status)
                .velocity(Velocity::<f32>::linear(
                    physics_body.velocity.x / delta_time,
                    physics_body.velocity.y / delta_time,
                    physics_body.velocity.z / delta_time,
                ))
                .angular_inertia(physics_body.angular_inertia)
                .mass(physics_body.mass)
                .local_center_of_mass(physics_body.local_center_of_mass)
                .user_data(entity)
                .build(&mut physics_world)
                .handle();

            physics_body.handle = Some(handle.clone());
            physics_body_handles.insert(entity.id(), handle);

            info!(
                "Inserted rigid body to world with values: {:?}",
                physics_body
            );
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("AddRigidBodiesSystem.setup");
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
