use amethyst::{
    core::Transform,
    ecs::{prelude::*, Read, ReadExpect, Resources, System, WriteStorage},
    shrev::ReaderId,
};

use crate::resources::{Command, CommandChannel, Player};

/// The `MovementSystem` handles the moving of the player `Entity` in the game
/// world. The `System` listens to the `CommandChannel` and moves the player
/// accordingly.
#[derive(Default)]
pub struct MovementSystem {
    command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Read<'s, CommandChannel>,
        ReadExpect<'s, Player>,
        WriteStorage<'s, Transform<f32>>,
    );

    fn run(&mut self, (commands, player, mut transforms): Self::SystemData) {
        for command in commands.read(self.command_reader.as_mut().unwrap()) {
            match command {
                Command::MoveUpDown(movement) => {
                    if let Some(transform) = transforms.get_mut(player.player) {
                        transform.prepend_translation_y(*movement);
                    }
                }
                Command::MoveLeftRight(movement) => {
                    if let Some(transform) = transforms.get_mut(player.player) {
                        transform.prepend_translation_x(*movement);
                    }
                }
            }
        }
    }

    /// Register reader for the `CommandChannel`.
    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
    }
}
