/// Reference:
/// https://github.com/distransient/nphysics-ecs-dumb
#[macro_use]
extern crate log;
pub extern crate ncollide3d as ncollide;
pub extern crate nphysics3d as nphysics;

use nalgebra::Vector3;
use nphysics::world::World;
pub use nphysics::{material, math::Velocity};

pub use self::{
    body::{PhysicsBody, PhysicsBodyBuilder},
    collider::{PhysicsCollider, PhysicsColliderBuilder, Shape},
    systems::PhysicsBundle,
};

pub mod body;
pub mod collider;
mod systems;

/// The `PhysicsWorld` containing all physical objects.
pub type PhysicsWorld = World<f32>;

/// `Gravity` is a type alias for `Vector3<f32>`. It represents a constant
/// acceleration affecting all physical objects in the scene.
pub type Gravity = Vector3<f32>;
