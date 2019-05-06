use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};
use nphysics::math::Velocity;
use nphysics::object::{BodyHandle, BodyStatus};

/// The `PhysicsBody` `Component` defines an entity of the `PhysicsWorld`. The velocity value
/// is automatically synchronised with the `PhysicsWorld`.
#[derive(Clone, Copy, Debug)]
pub struct PhysicsBody {
    pub(crate) handle: Option<BodyHandle>,
    pub velocity: Velocity<f32>,

    pub body_status: BodyStatus,
}

impl Component for PhysicsBody {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl PhysicsBody {
    pub fn new_static() -> Self {
        Self {
            handle: None,
            velocity: Velocity::<f32>::zero(),
            body_status: BodyStatus::Static,
        }
    }

    pub fn new_kinematic(velocity: Velocity<f32>) -> Self {
        Self {
            handle: None,
            velocity,
            body_status: BodyStatus::Kinematic,
        }
    }

    pub fn handle(&self) -> Option<BodyHandle> {
        self.handle
    }
}
