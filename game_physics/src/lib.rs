/// Reference:
/// https://github.com/distransient/nphysics-ecs-dumb
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;
pub extern crate ncollide2d as ncollide;
pub extern crate nphysics2d as nphysics;

use nalgebra::Vector2;
use nphysics::world::World;
pub use nphysics::{material, math::Velocity};

pub use self::{
    body::{Motion, MotionBuilder, PhysicsBody, PhysicsBodyBuilder},
    collider::{PhysicsCollider, PhysicsColliderBuilder, Shape},
    systems::PhysicsBundle,
};

pub mod body;
pub mod collider;
mod systems;

/// The `PhysicsWorld` containing all physical objects.
pub type PhysicsWorld = World<f32>;

/// `Gravity` is a type alias for `Vector2<f32`. It represents a constant
/// acceleration affecting all physical objects in the scene.
pub type Gravity = Vector2<f32>;
