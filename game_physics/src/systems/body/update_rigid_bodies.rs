use crate::{body::PhysicsBody, systems::modified_components, PhysicsWorld};

use amethyst::{
    core::transform::Transform,
    ecs::{
        storage::ComponentEvent,
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

use nalgebra::Isometry3;
use nphysics::{math::Velocity, object::Body};

/// The `UpdateRigidBodiesSystems` handles the synchronisation of updated
/// `PhysicsBody` `Component`s with their `PhysicsWorld` counterparts. This
/// happens based on `ComponentEvent::Modified` for the `PhysicsBody`
/// `Component`.
#[derive(Default)]
pub struct UpdateRigidBodiesSystems {
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
    transforms_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for UpdateRigidBodiesSystems {
    type SystemData = (
        ReadStorage<'s, PhysicsBody>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, PhysicsWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_bodies, transforms, mut physics_world) = data;

        // collect all modified PhysicsBody components
        let modified_physics_bodies = modified_components(
            &physics_bodies,
            self.physics_bodies_reader_id.as_mut().unwrap(),
        );

        // collect all modified Transform components
        let modified_transforms =
            modified_components(&transforms, self.transforms_reader_id.as_mut().unwrap());

        // iterate over all modified PhysicBody components and their Transforms; we use
        // modified Transforms to update the position of an entity in the PhysicsWorld
        // directly
        for (physics_body, transform, id) in (
            &physics_bodies,
            &transforms,
            &modified_physics_bodies | &modified_transforms,
        )
            .join()
        {
            debug!("Modified PhysicsBody with id: {}", id);
            let delta_time = physics_world.timestep();

            if let Some(rigid_body) = physics_world.rigid_body_mut(physics_body.handle.unwrap()) {
                // the PhysicsBody was modified, update everything but the position
                if modified_physics_bodies.contains(id) {
                    rigid_body.enable_gravity(physics_body.gravity_enabled);
                    rigid_body.set_status(physics_body.body_status);
                    rigid_body.set_velocity(Velocity::<f32>::linear(
                        physics_body.velocity.x / delta_time,
                        physics_body.velocity.y / delta_time,
                        physics_body.velocity.z / delta_time,
                    ));
                    rigid_body.set_angular_inertia(physics_body.angular_inertia);
                    rigid_body.set_mass(physics_body.mass);
                    rigid_body.set_local_center_of_mass(physics_body.local_center_of_mass.clone());
                }

                // the Transform was modified, update the position directly
                if modified_transforms.contains(id) {
                    rigid_body.set_position(Isometry3::translation(
                        transform.isometry().translation.x.as_f32(),
                        transform.isometry().translation.y.as_f32(),
                        transform.isometry().translation.z.as_f32(),
                    ));
                }

                trace!(
                    "Updated rigid body in world with values: {:?}",
                    physics_body
                );
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("UpdateRigidBodiesSystems.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<PhysicsWorld>().or_insert(PhysicsWorld::new());

        // register reader id for the PhysicsBody storage
        let mut physics_body_storage: WriteStorage<PhysicsBody> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());

        // register reader id for the Transform storage
        let mut transform_storage: WriteStorage<Transform> = SystemData::fetch(&res);
        self.transforms_reader_id = Some(transform_storage.register_reader());
    }
}
