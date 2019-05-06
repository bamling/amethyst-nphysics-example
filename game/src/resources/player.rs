use amethyst::ecs::prelude::Entity;

/// The Player `Resources` contains player relevant data and holds a reference to the `Entity`
/// that defines the player.
#[derive(Debug)]
pub struct Player {
    /// The player `Entity`.
    pub player: Entity,
}