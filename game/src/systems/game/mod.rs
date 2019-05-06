use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, error::Error};

pub use self::player::PlayerSystemsBundle;

mod player;

/// Bundle containing all `System`s relevant to the `GameState`.
#[derive(Default)]
pub struct GameSystemsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameSystemsBundle {
    fn build(self, dispatcher: &mut DispatcherBuilder) -> Result<(), Error> {
        // add player systems
        PlayerSystemsBundle::default().build(dispatcher)?;

        Ok(())
    }
}
