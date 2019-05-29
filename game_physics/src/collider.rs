use std::{collections::HashMap, f32::consts::PI, fmt};

use amethyst::ecs::{world::Index, Component, DenseVecStorage, FlaggedStorage};
use nalgebra::{Isometry3, Vector3};
use ncollide::{
    shape::{Ball, Cuboid, ShapeHandle},
    world::CollisionGroups,
};
pub use nphysics::material;
use nphysics::object::ColliderHandle;

use self::material::{BasicMaterial, MaterialHandle};

/// The `HashMap` of `Index` to physics `ColliderHandle` mappings. This is used
/// for the mapping of Amethyst `Entity`s based on their unique `Index` to
/// `Collider`s created in the `PhysicsWorld`.
pub type PhysicsColliderHandles = HashMap<Index, ColliderHandle>;

/// Custom `Isometry` type to prevent collisions with forked
/// `nalgebra` versions.
pub type Isometry = Isometry3<f32>;

/// `Shape` serves as an abstraction over nphysics `ShapeHandle`s and makes it
/// easier to configure and define said `ShapeHandle`s for the user without
/// having to know the underlying nphysics API. e.g:
///
/// ```rust,ignore
/// Shape::Rectangle(10.0, 10.0, 10.0)
/// ```
/// translates to
/// ```rust,ignore
/// ShapeHandle::new(Cuboid::new(10.0, 10.0, 10.0))
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle(f32),
    Rectangle(f32, f32, f32),
}

impl Shape {
    /// Converts a `Shape` and its values into its corresponding `ShapeHandle`
    /// type. The `ShapeHandle` is used to define a `Collider` in the
    /// `PhysicsWorld`.
    fn handle(&self, margin: f32) -> ShapeHandle<f32> {
        match *self {
            Shape::Circle(radius) => ShapeHandle::new(Ball::new(radius)),
            Shape::Rectangle(width, height, depth) => ShapeHandle::new(Cuboid::new(Vector3::new(
                width / 2.0 - margin,
                height / 2.0 - margin,
                depth / 2.0 - margin,
            ))),
        }
    }
}

/// The `PhysicsCollider` `Component` represents a `Collider` in the physics
/// world. A physics `Collider` is automatically created when this `Component`
/// is added to an `Entity`. Value changes are automatically synchronised with
/// the physic worlds `Collider`.
///
/// For more information on how the synchronisation is handled, see the
/// following `System`s:
/// - `systems::collider::add_colliders::AddCollidersSystem`
/// - `systems::collider::update_colliders::UpdateCollidersSystem`
/// - `systems::collider::remove_colliders::RemoveCollidersSystem`
///
/// These `System`s work based on the `PhysicsCollider` `Component`s.
#[derive(Clone)]
pub struct PhysicsCollider {
    pub(crate) handle: Option<ColliderHandle>,
    pub shape: Shape,
    pub offset_from_parent: Isometry,
    pub density: f32,
    pub material: MaterialHandle<f32>,
    pub margin: f32,
    pub collision_groups: CollisionGroups,
    pub linear_prediction: f32,
    pub angular_prediction: f32,
    pub sensor: bool,
}

impl Component for PhysicsCollider {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl fmt::Debug for PhysicsCollider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PhysicsCollider {{ \
             handle: {:?}, \
             offset_from_parent: {:?}, \
             density: {}, \
             margin: {}, \
             collision_group: {:?}, \
             linear_prediction: {}, \
             angular_prediction: {}, \
             sensor: {} \
             }}",
            self.handle,
            self.offset_from_parent,
            self.density,
            self.margin,
            self.collision_groups,
            self.linear_prediction,
            self.angular_prediction,
            self.sensor,
        )?;
        Ok(())
    }
}

impl PhysicsCollider {
    /// Returns the `ShapeHandle` for `shape`, taking the `margin` into
    /// consideration.
    pub(crate) fn shape_handle(&self) -> ShapeHandle<f32> {
        self.shape.handle(self.margin)
    }
}

/// The `PhysicsColliderBuilder` implements the builder pattern for
/// `PhysicsCollider`s and is the recommended way of instantiating and
/// customising new `PhysicsCollider` instances.
///
/// # Example
///
/// ```rust
/// use game_physics::{collider::Isometry, PhysicsColliderBuilder, Shape};
/// use ncollide3d::world::CollisionGroups;
/// use nphysics3d::material::{BasicMaterial, MaterialHandle};
///
/// let physics_collider = PhysicsColliderBuilder::from(Shape::Rectangle(10.0, 10.0, 1.0))
///     .offset_from_parent(Isometry::identity())
///     .density(1.2)
///     .material(MaterialHandle::new(BasicMaterial::default()))
///     .margin(0.02)
///     .collision_groups(CollisionGroups::default())
///     .linear_prediction(0.001)
///     .angular_prediction(0.0)
///     .sensor(true)
///     .build();
/// ```
pub struct PhysicsColliderBuilder {
    shape: Shape,
    offset_from_parent: Isometry,
    density: f32,
    material: MaterialHandle<f32>,
    margin: f32,
    collision_groups: CollisionGroups,
    linear_prediction: f32,
    angular_prediction: f32,
    sensor: bool,
}

impl From<Shape> for PhysicsColliderBuilder {
    /// Creates a new `PhysicsColliderBuilder` from the given `Shape`. This
    //  also populates the `PhysicsCollider` with sane defaults.
    fn from(shape: Shape) -> Self {
        Self {
            shape,
            offset_from_parent: Isometry::identity(),
            density: 1.3,
            material: MaterialHandle::new(BasicMaterial::default()),
            margin: 0.2, // default was: 0.01
            collision_groups: CollisionGroups::default(),
            linear_prediction: 0.002,
            angular_prediction: PI / 180.0 * 5.0,
            sensor: false,
        }
    }
}

impl PhysicsColliderBuilder {
    /// Sets the `offset_from_parent` value of the `PhysicsColliderBuilder`.
    pub fn offset_from_parent(mut self, offset_from_parent: Isometry) -> Self {
        self.offset_from_parent = offset_from_parent;
        self
    }

    /// Sets the `density` value of the `PhysicsColliderBuilder`.
    pub fn density(mut self, density: f32) -> Self {
        self.density = density;
        self
    }

    /// Sets the `material` value of the `PhysicsColliderBuilder`.
    pub fn material(mut self, material: MaterialHandle<f32>) -> Self {
        self.material = material;
        self
    }

    /// Sets the `margin` value of the `PhysicsColliderBuilder`.
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Sets the `collision_groups` value of the `PhysicsColliderBuilder`.
    pub fn collision_groups(mut self, collision_groups: CollisionGroups) -> Self {
        self.collision_groups = collision_groups;
        self
    }

    /// Sets the `linear_prediction` value of the `PhysicsColliderBuilder`.
    pub fn linear_prediction(mut self, linear_prediction: f32) -> Self {
        self.linear_prediction = linear_prediction;
        self
    }

    /// Sets the `angular_prediction` value of the `PhysicsColliderBuilder`.
    pub fn angular_prediction(mut self, angular_prediction: f32) -> Self {
        self.angular_prediction = angular_prediction;
        self
    }

    /// Sets the `sensor` value of the `PhysicsColliderBuilder`.
    pub fn sensor(mut self, sensor: bool) -> Self {
        self.sensor = sensor;
        self
    }

    /// Builds the `PhysicsCollider` from the values set in the
    /// `PhysicsColliderBuilder` instance.
    pub fn build(self) -> PhysicsCollider {
        PhysicsCollider {
            handle: None,
            shape: self.shape,
            offset_from_parent: self.offset_from_parent,
            density: self.density,
            material: self.material,
            margin: self.margin,
            collision_groups: self.collision_groups,
            linear_prediction: self.linear_prediction,
            angular_prediction: self.angular_prediction,
            sensor: self.sensor,
        }
    }
}
