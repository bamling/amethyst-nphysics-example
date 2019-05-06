use amethyst::{
    ecs::{
        Read,
        System,
        Write,
    },
    input::InputHandler,
};

use crate::resources::{
    Command,
    CommandChannel,
};

/// `InputSystem` encapsulates player input handling and converts receiver input into
/// `Command`s. These `Command`s are then published to other `System`s via the `CommandChannel`.
#[derive(Default)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        Write<'s, CommandChannel>
    );

    fn run(&mut self, (input, mut commands): Self::SystemData) {
        // handle movement on X axis
        if let Some(movement) = input.axis_value("leftright") {
            if movement != 0.0 {
                commands.single_write(Command::MoveLeftRight(movement as f32));
            }
        }

        // handle movement on Y axis
        if let Some(movement) = input.axis_value("updown") {
            if movement != 0.0 {
                commands.single_write(Command::MoveUpDown(movement as f32));
            }
        }
    }
}
