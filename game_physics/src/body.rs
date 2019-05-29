use std::collections::HashMap;

use amethyst::ecs::{world::Index, Component, DenseVecStorage, FlaggedStorage};
use nphysics::object::BodyHandle;
pub use nphysics::object::BodyStatus;

use crate::math::{Matrix3, Point3, Vector3};

/// The `HashMap` of `Index` to physics `BodyHandle` mappings. This is used for
/// the mapping of Amethyst `Entity`s based on their unique `Index` to
/// `RigidBody`s created in the `PhysicsWorld`.
pub type PhysicsBodyHandles = HashMap<Index, BodyHandle>;

/// The `PhysicsBody` `Component` represents a `PhysicsWorld` `RigidBody` in
/// Amethyst/specs and contains all the data required for the synchronisation
/// between both worlds.
///
/// For more information on how the synchronisation is handled, see the
/// following `System`s:
/// - `systems::body::add_rigid_bodies::AddRigidBodiesSystem`
/// - `systems::body::update_rigid_bodies::UpdateRigidBodiesSystem`
/// - `systems::body::remove_rigid_bodies::RemoveRigidBodiesSystem`
///
/// These `System`s work based on the `PhysicsBody` `Component`s.
#[derive(Clone, Copy, Debug)]
pub struct PhysicsBody {
    pub(crate) handle: Option<BodyHandle>,
    pub gravity_enabled: bool,
    pub body_status: BodyStatus,
    pub velocity: Vector3<f32>,
    pub angular_inertia: Matrix3<f32>,
    pub mass: f32,
    pub local_center_of_mass: Point3<f32>,
}

impl Component for PhysicsBody {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

/// The `PhysicsBodyBuilder` implements the builder pattern for `PhysicsBody`s
/// and is the recommended way of instantiating and customising new
/// `PhysicsBody` instances.
///
/// # Example
///
/// ```rust
/// use game_physics::{
///     body::BodyStatus,
///     math::{Matrix3, Point3, Vector3},
///     PhysicsBodyBuilder,
/// };
///
/// let physics_body = PhysicsBodyBuilder::from(BodyStatus::Dynamic)
///     .gravity_enabled(true)
///     .velocity(Vector3::new(1.0, 1.0, 1.0))
///     .angular_inertia(Matrix3::from_diagonal_element(3.0))
///     .mass(1.3)
///     .local_center_of_mass(Point3::new(0.0, 0.0, 0.0))
///     .build();
/// ```
pub struct PhysicsBodyBuilder {
    gravity_enabled: bool,
    body_status: BodyStatus,
    velocity: Vector3<f32>,
    angular_inertia: Matrix3<f32>,
    mass: f32,
    local_center_of_mass: Point3<f32>,
}

impl From<BodyStatus> for PhysicsBodyBuilder {
    /// Creates a new `PhysicsBodyBuilder` from the given `BodyStatus`. This
    /// also populates the `PhysicsBody` with sane defaults.
    fn from(body_status: BodyStatus) -> Self {
        Self {
            gravity_enabled: false,
            body_status,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            angular_inertia: Matrix3::zeros(),
            mass: 1.2,
            local_center_of_mass: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

impl PhysicsBodyBuilder {
    /// Sets the `gravity_enabled` value of the `PhysicsBodyBuilder`.
    pub fn gravity_enabled(mut self, gravity_enabled: bool) -> Self {
        self.gravity_enabled = gravity_enabled;
        self
    }

    // Sets the `velocity` value of the `PhysicsBodyBuilder`.
    pub fn velocity(mut self, velocity: Vector3<f32>) -> Self {
        self.velocity = velocity;
        self
    }

    /// Sets the `angular_inertia` value of the `PhysicsBodyBuilder`.
    pub fn angular_inertia(mut self, angular_inertia: Matrix3<f32>) -> Self {
        self.angular_inertia = angular_inertia;
        self
    }

    /// Sets the `mass` value of the `PhysicsBodyBuilder`.
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    /// Sets the `local_center_of_mass` value of the `PhysicsBodyBuilder`.
    pub fn local_center_of_mass(mut self, local_center_of_mass: Point3<f32>) -> Self {
        self.local_center_of_mass = local_center_of_mass;
        self
    }

    /// Builds the `PhysicsBody` from the values set in the `PhysicsBodyBuilder`
    /// instance.
    pub fn build(self) -> PhysicsBody {
        PhysicsBody {
            handle: None,
            gravity_enabled: self.gravity_enabled,
            body_status: self.body_status,
            velocity: self.velocity,
            angular_inertia: self.angular_inertia,
            mass: self.mass,
            local_center_of_mass: self.local_center_of_mass,
        }
    }
}
