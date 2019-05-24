use std::{collections::HashMap, f32::consts::PI, fmt};

use amethyst::ecs::{world::Index, Component, DenseVecStorage, FlaggedStorage};
use nalgebra::Vector2;
use ncollide::{
    shape::{Ball, Cuboid, ShapeHandle},
    world::CollisionGroups,
};
use nphysics::{
    material::{BasicMaterial, MaterialHandle},
    object::ColliderHandle,
};

/// The `HashMap` of `Index` to physics `ColliderHandle` mappings. This is used
/// for the mapping of Amethyst `Entity`s based on their unique `Index` to
/// `Collider`s created in the physics `World`.
pub type PhysicsColliderHandles = HashMap<Index, ColliderHandle>;

/// Custom exported `Isometry2` type to prevent collisions with forked
/// `nalgebra` versions.
pub type Isometry2 = nalgebra::Isometry2<f32>;

/// `Shape` serves as an abstraction over nphysics `ShapeHandle`s and makes it
/// easier to configure and define said `ShapeHandle`s for the user without
/// having to know the underlying nphysics API. e.g:
///
/// ```rust,ignore
/// Shape::Rectangle(10.0, 10.0)
/// ```
/// translate to
/// ```rust,ignore
/// ShapeHandle::new(Cuboid::new(10.0, 10.0))
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
}

impl Shape {
    /// Converts a `Shape` and its values into its corresponding `ShapeHandle`
    /// type. The `ShapeHandle` is used to define a `Collider` in the physics
    /// `World.
    pub(crate) fn handle(&self) -> ShapeHandle<f32> {
        match *self {
            Shape::Circle(radius) => ShapeHandle::new(Ball::new(radius)),
            Shape::Rectangle(x, y) => ShapeHandle::new(Cuboid::new(Vector2::new(x, y))),
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
///
/// Use the *derived* `PhysicsBodyBuilder` to create new instances of
/// `PhysicsBody`:
///
/// ```rust
/// use game_physics::{collider::Isometry2, PhysicsColliderBuilder, Shape};
/// use ncollide2d::world::CollisionGroups;
/// use nphysics2d::material::{BasicMaterial, MaterialHandle};
///
/// let physics_collider = PhysicsColliderBuilder::from(Shape::Rectangle(10.0, 10.0))
///     .offset_from_parent(Isometry2::identity())
///     .density(1.2)
///     .material(MaterialHandle::new(BasicMaterial::default()))
///     .margin(0.02)
///     .collision_group(CollisionGroups::default())
///     .linear_prediction(0.001)
///     .angular_prediction(0.0)
///     .sensor(true)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Builder)]
#[builder(pattern = "owned")]
pub struct PhysicsCollider {
    #[builder(setter(skip))]
    pub(crate) handle: Option<ColliderHandle>,
    pub(crate) shape: ShapeHandle<f32>,
    pub offset_from_parent: Isometry2,
    pub density: f32,
    pub material: MaterialHandle<f32>,
    pub margin: f32,
    pub collision_group: CollisionGroups,
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
            self.collision_group,
            self.linear_prediction,
            self.angular_prediction,
            self.sensor,
        )?;
        Ok(())
    }
}

impl From<Shape> for PhysicsColliderBuilder {
    /// Creates a new `PhysicsColliderBuilder` from the given `Shape`. It is
    /// recommended to use this over `PhysicsColliderColliderBuilder::default()`
    /// as it actually populates the internal `ShapeHandle`.
    fn from(shape: Shape) -> Self {
        PhysicsColliderBuilder::default()
            .shape(shape.handle())
            .offset_from_parent(Isometry2::identity())
            .density(1.3)
            .material(MaterialHandle::new(BasicMaterial::default()))
            .margin(0.01)
            .collision_group(CollisionGroups::default())
            .linear_prediction(0.002)
            .angular_prediction(PI / 180.0 * 5.0)
            .sensor(false)
    }
}
