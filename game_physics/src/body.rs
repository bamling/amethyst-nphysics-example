use amethyst::ecs::{world::Index, Component, DenseVecStorage, FlaggedStorage};
use nalgebra::Vector2;
use nphysics::object::{BodyHandle, BodyStatus};

use std::collections::HashMap;

/// The `HashMap` of `Index` to physics `BodyHandle` mappings. This is used for
/// the mapping of Amethyst `Entity`s based on their unique `Index` to
/// `RigidBody`s created in the physics `World`.
pub type PhysicsBodyHandles = HashMap<Index, BodyHandle>;

/// Custom exported `Point2` type to prevent collisions with forked `nalgebra`
/// versions.
pub type Point2 = nalgebra::Point2<f32>;

/// The `Motion` `Component` contains the `velocity` and the `acceleration` for
/// a moving `Entity`. These values make up a `RigidBody`s velocity and enable
/// it to move within the physics `World`.
#[derive(Builder)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct Motion {
    #[builder(setter(skip))]
    pub(crate) velocity: Vector2<f32>,
    // TODO: currently not used
    #[builder(setter(skip))]
    pub(crate) acceleration: Vector2<f32>,
}

impl Component for Motion {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Default for Motion {
    /// Creates a `Motion` with default values. `velocity` and `acceleration`
    /// are initialised as *zero* vectors, e.g.:
    ///
    /// ```rust,ignore
    /// Vector2::<f32>::new(0.0, 0.0)
    /// ```
    fn default() -> Self {
        Self {
            velocity: Vector2::<f32>::new(0.0, 0.0),
            acceleration: Vector2::<f32>::new(0.0, 0.0),
        }
    }
}

impl Motion {
    /// Sets the `velocity` x value. This only affects the x value and preserves
    /// the current y value.
    pub fn set_velocity_x(&mut self, x: f32) {
        self.velocity.x = x;
    }

    /// Sets the `velocity` y value. This only affects the y value and preserves
    /// the current x value.
    pub fn set_velocity_y(&mut self, y: f32) {
        self.velocity.y = y;
    }

    // Sets the `velocity` with the given x and y values. This overwrites any
    // previously set `velocity` values.
    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = Vector2::<f32>::new(x, y);
    }

    /// Sets the `acceleration` value with the given x and y values.
    pub fn set_acceleration(&mut self, x: f32, y: f32) {
        self.acceleration = Vector2::<f32>::new(x, y);
    }
}

/// The `PhysicsBody` `Component` represents a physics `World` `RigidBody` in
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
///
/// Use the *derived* `PhysicsBodyBuilder` to create new instances of
/// `PhysicsBody`:
///
/// ```rust
/// use game_physics::{body::Point2, PhysicsBodyBuilder};
///
/// let physics_body = PhysicsBodyBuilder::new_dynamic()
///     .gravity_enabled(true)
///     .angular_inertia(0.01)
///     .mass(1.3)
///     .local_center_of_mass(Point2::new(0.0, 0.0))
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Clone, Copy, Debug)]
#[builder(default)]
#[builder(pattern = "owned")]
pub struct PhysicsBody {
    #[builder(setter(skip))]
    pub(crate) handle: Option<BodyHandle>,
    pub gravity_enabled: bool,
    pub(crate) body_status: BodyStatus,
    pub angular_inertia: f32,
    pub mass: f32,
    pub local_center_of_mass: Point2,
}

impl Component for PhysicsBody {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Default for PhysicsBody {
    /// Creates a `PhysicsBody` with default values.
    fn default() -> Self {
        Self {
            handle: None,
            gravity_enabled: false,
            body_status: BodyStatus::Dynamic,
            angular_inertia: 0.0,
            mass: 1.2, // most cases require a mass
            local_center_of_mass: Point2::new(0.0, 0.0),
        }
    }
}

impl PhysicsBody {
    /// Returns the optional `BodyHandle`.
    pub fn handle(&self) -> Option<BodyHandle> {
        self.handle
    }
}

impl PhysicsBodyBuilder {
    /// Creates a new `PhysicsBodyBuilder` with `body_status` set to
    /// `BodyStatus::Disabled`.
    pub fn new_disabled() -> Self {
        PhysicsBodyBuilder::default().body_status(BodyStatus::Disabled)
    }

    /// Creates a new `PhysicsBodyBuilder` with `body_status` set to
    /// `BodyStatus::Static`.
    pub fn new_static() -> Self {
        PhysicsBodyBuilder::default().body_status(BodyStatus::Static)
    }

    /// Creates a new `PhysicsBodyBuilder` with `body_status` set to
    /// `BodyStatus::Dynamic`.
    pub fn new_dynamic() -> Self {
        PhysicsBodyBuilder::default().body_status(BodyStatus::Dynamic)
    }

    /// Creates a new `PhysicsBodyBuilder` with `body_status` set to
    /// `BodyStatus::Kinematic`.
    pub fn new_kinematic() -> Self {
        PhysicsBodyBuilder::default().body_status(BodyStatus::Kinematic)
    }
}
