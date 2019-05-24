use crate::{body::PhysicsBody, systems::modified_components, PhysicsWorld};

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

use nphysics::object::Body;

/// The `UpdateRigidBodiesSystems` handles the synchronisation of updated
/// `PhysicsBody` `Component`s with their physics `World` counterparts. This
/// happens based on `ComponentEvent::Modified` for the `PhysicsBody`
/// `Component`.
#[derive(Default)]
pub struct UpdateRigidBodiesSystems {
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for UpdateRigidBodiesSystems {
    type SystemData = (ReadStorage<'s, PhysicsBody>, WriteExpect<'s, PhysicsWorld>);

    fn run(&mut self, data: Self::SystemData) {
        let (physics_bodies, mut physics_world) = data;

        // collect all modified PhysicsBody components
        let modified_physics_bodies = modified_components(
            &physics_bodies,
            self.physics_bodies_reader_id.as_mut().unwrap(),
        );

        // iterate over all modified PhysicBody components; we do not have to ensure
        // that they belong to an entity with a Transform in this place as they were
        // already positioned in the world
        for (physics_body, id) in (&physics_bodies, &modified_physics_bodies).join() {
            debug!("Modified PhysicsBody with id: {}", id);
            if let Some(rigid_body) = physics_world.rigid_body_mut(physics_body.handle().unwrap()) {
                rigid_body.enable_gravity(physics_body.gravity_enabled);
                rigid_body.set_status(physics_body.body_status);
                rigid_body.set_angular_inertia(physics_body.angular_inertia);
                rigid_body.set_mass(physics_body.mass);
                rigid_body.set_local_center_of_mass(physics_body.local_center_of_mass.clone());

                info!(
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
    }
}
