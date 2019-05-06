use amethyst::shrev::EventChannel;

/// List of `Command`s that are interpreted by `System`s.
#[derive(Debug)]
pub enum Command {
    MoveUpDown(f32),
    MoveLeftRight(f32),
}

/// Custom type alias for `EventChannel<Command>`.
pub type CommandChannel = EventChannel<Command>;
