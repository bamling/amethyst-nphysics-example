/// Reference:
/// https://github.com/distransient/nphysics-ecs-dumb
#[macro_use]
extern crate log;
extern crate ncollide3d as ncollide;
extern crate nphysics3d as nphysics;

use std::collections::HashMap;

use amethyst::ecs::world::Index;
pub use nalgebra as math;
use nphysics::{
    object::{BodyHandle, ColliderHandle},
    world::World,
};

use self::math::Vector3;
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
