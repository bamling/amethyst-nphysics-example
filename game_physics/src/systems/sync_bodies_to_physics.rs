use amethyst::{
    core::transform::Transform,
    ecs::{
        storage::ComponentEvent,
        ReadStorage,
        ReaderId,
        Resources,
        System,
        SystemData,
        WriteExpect,
        WriteStorage,
    },
};

use crate::{body::PhysicsBody, Physics};

///
#[derive(Default)]
pub struct SyncBodiesToPhysics {
    physics_bodies_reader_id: Option<ReaderId<ComponentEvent>>,
    transforms_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl<'s> System<'s> for SyncBodiesToPhysics {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteExpect<'s, Physics>,
        WriteStorage<'s, PhysicsBody>,
    );

    fn run(&mut self, data: Self::SystemData) {
        unimplemented!()
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("SyncBodiesToPhysics.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<Physics>().or_insert_with(Physics::default);

        // register reader id for the PhysicsBody storage
        let mut physics_body_storage: WriteStorage<PhysicsBody> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());

        // register reader id for the Transform storage
        let mut transform_storage: WriteStorage<Transform> = SystemData::fetch(&res);
        self.physics_bodies_reader_id = Some(physics_body_storage.register_reader());
    }
}
