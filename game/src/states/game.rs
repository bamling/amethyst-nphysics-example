use amethyst::{
    assets::{Handle, Prefab},
    core::{transform::Transform, SystemBundle},
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord},
        sprite::{SpriteRender, SpriteSheetHandle},
    },
    ui::FontHandle,
    utils::scene::BasicScenePrefab,
    winit::VirtualKeyCode,
};

use game_physics::{body::BodyStatus, PhysicsBodyBuilder, PhysicsColliderBuilder, Shape};

use crate::{resources::Player, systems::GameSystemsBundle};

pub type GamePrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

/// The `GameState` contains the actual game area and gameplay elements. When
/// the escape key is pressed, the game exists.
pub struct GameState<'a, 'b> {
    /// `State` specific dispatcher.
    dispatcher: Option<Dispatcher<'a, 'b>>,

    scene_handle: Handle<Prefab<GamePrefabData>>,
    font_handle: FontHandle,

    character_handle: SpriteSheetHandle,
    objects_handle: SpriteSheetHandle,
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("GameState.on_start");
        let world = data.world;

        // create dispatcher
        self.create_dispatcher(world);

        // initialise scene
        world
            .create_entity()
            .with(self.scene_handle.clone())
            .build();

        // initialise game elements
        self.initialise_player(world);
        self.initialise_obstacles(world);
    }

    fn on_stop(&mut self, _data: StateData<GameData>) {
        info!("GameState.on_stop");
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        // handle window events and quit the current State if the Escape button is
        // pressed
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // event was not of type StateEvent, so no transition is required
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world.res);
        }

        Trans::None
    }
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new(
        scene_handle: Handle<Prefab<GamePrefabData>>,
        font_handle: FontHandle,
        character_handle: SpriteSheetHandle,
        objects_handle: SpriteSheetHandle,
    ) -> Self {
        Self {
            dispatcher: None,
            scene_handle,
            font_handle,
            character_handle,
            objects_handle,
        }
    }

    /// Creates the `State` specific `Dispatcher`.
    fn create_dispatcher(&mut self, world: &mut World) {
        if self.dispatcher.is_none() {
            let mut dispatcher_builder = DispatcherBuilder::new();
            GameSystemsBundle::default()
                .build(&mut dispatcher_builder)
                .expect("Failed to register GameSystemsBundle");

            let mut dispatcher = dispatcher_builder.build();
            dispatcher.setup(&mut world.res);
            self.dispatcher = Some(dispatcher);
        }
    }

    /// Creates the player `Entity` and its corresponding `Player` `Resource`.
    fn initialise_player(&mut self, world: &mut World) {
        // create the players Transform
        let mut transform = Transform::default();
        transform.set_translation_xyz(25.0, 50.0, 0.0);

        // create player Entity
        let player = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: self.character_handle.clone(),
                sprite_number: 0,
            })
            .with(PhysicsBodyBuilder::from(BodyStatus::Dynamic).build())
            .with(PhysicsColliderBuilder::from(Shape::Rectangle(7.5, 7.5, 1.0)).build())
            .with(transform)
            .build();

        // create the player Resource
        world.add_resource(Player { player });
    }

    /// Creates the random obstacle `Entity`s.
    fn initialise_obstacles(&mut self, world: &mut World) {
        // create the Transform
        let mut transform = Transform::default();
        transform.set_translation_xyz(75.0, 50.0, 0.0);

        // create the Entity
        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: self.objects_handle.clone(),
                sprite_number: 0,
            })
            .with(PhysicsBodyBuilder::from(BodyStatus::Static).build())
            .with(PhysicsColliderBuilder::from(Shape::Rectangle(7.5, 7.5, 1.0)).build())
            .with(transform)
            .build();
    }
}
