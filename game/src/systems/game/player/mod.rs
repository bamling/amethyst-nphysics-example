use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, error::Error};

pub use self::{input::InputSystem, movement::MovementSystem};

mod input;
mod movement;

/// Bundle containing all `System`s relevant to the player.
#[derive(Default)]
pub struct PlayerSystemsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PlayerSystemsBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        dispatcher.add(InputSystem::default(), "player_input_system", &[]);

        dispatcher.add(
            MovementSystem::default(),
            "player_movement_system",
            &["player_input_system"],
        );

        Ok(())
    }
}
