#[macro_use]
extern crate log;

pub extern crate ncollide2d as ncollide;
pub extern crate nphysics2d as nphysics;

use std::collections::HashMap;

use amethyst::ecs::Entity;
use nphysics::{object::BodyHandle, world::World};

pub mod body;
pub mod systems;

/// The `PhysicsWorld` containing all physical objects.
pub type PhysicsWorld = World<f32>;

/// The `HashMap` of Amethyst `Entity` to nphysics `BodyHandle` mappings.
pub type EntityBodyHandles = HashMap<Entity, BodyHandle>;
