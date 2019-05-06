pub extern crate ncollide2d as ncollide;
pub extern crate nphysics2d as nphysics;

#[macro_use]
extern crate log;

pub mod body;
pub mod systems;

/// The `PhysicsWorld` containing all physical objects.
pub type PhysicsWorld = self::nphysics::world::World<f32>;
