#[macro_use]
extern crate log;

use std::sync::Arc;

use amethyst::{
    assets::{PrefabLoaderSystem, Processor},
    core::transform::TransformBundle,
    ecs::{ReadExpect, Resources, SystemData},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        pass::{DrawDebugLinesDesc, DrawFlat2DDesc},
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
        },
        sprite::SpriteSheet,
        types::DefaultBackend,
        GraphCreator,
        RenderingSystem,
    },
    ui::UiBundle,
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

use game_physics::PhysicsBundle;

use crate::states::{GamePrefabData, LoadingState};

mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    //amethyst::start_logger(Default::default());
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_factory::factory", amethyst::LogLevelFilter::Warn)
        .level_for(
            "rendy_memory::allocator::dynamic",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for(
            "rendy_graph::node::render::pass",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for("rendy_graph::node::present", amethyst::LogLevelFilter::Warn)
        .level_for("rendy_graph::graph", amethyst::LogLevelFilter::Warn)
        .level_for(
            "rendy_memory::allocator::linear",
            amethyst::LogLevelFilter::Warn,
        )
        .level_for("rendy_wsi", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;

    // display configuration
    let display_config_path = app_root.join("resources/display_config.ron");

    // key bindings
    let key_bindings_path = app_root.join("resources/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with_bundle(PhysicsBundle::default().with_debug_lines())? // TODO: move to custom game data?!
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(PrefabLoaderSystem::<GamePrefabData>::default(), "", &[])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            ExampleGraph::default(),
        ));

    let mut game =
        Application::build(app_root.join("assets"), LoadingState::default())?.build(game_data)?;

    game.run();

    Ok(())
}

// This graph structure is used for creating a proper `RenderGraph` for
// rendering. A renderGraph can be thought of as the stages during a render
// pass. In our case, we are only executing one subpass (DrawFlat2D, or the
// sprite pass). This graph also needs to be rebuilt whenever the window is
// resized, so the boilerplate code for that operation is also here.
#[derive(Default)]
struct ExampleGraph {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for ExampleGraph {
    // This trait method reports to the renderer if the graph must be rebuilt,
    // usually because the window has been resized. This implementation checks
    // the screen size and returns true if it has changed.
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the
        // same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    // This is the core of a RenderGraph, which is building the actual graph with
    // subpasses and target images.
    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        use amethyst::renderer::rendy::{
            graph::present::PresentNode,
            hal::command::{ClearDepthStencil, ClearValue},
        };

        self.dirty = false;

        // Retrieve a reference to the target window, which is created by the
        // WindowBundle
        let window = <ReadExpect<'_, Arc<Window>>>::fetch(res);

        // Create a new drawing surface in our window
        let surface = factory.create_surface(&window);
        // cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = image::Kind::D2(
            dbg!(dimensions.width()) as u32,
            dimensions.height() as u32,
            1,
            1,
        );

        // Begin building our RenderGraph
        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.0, 0.0, 0.0, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        // Create our single `Subpass`, which is the DrawFlat2D pass.
        // We pass the subpass builder a description of our pass for construction
        let sprite = graph_builder.add_node(
            SubpassBuilder::new()
                .with_group(DrawDebugLinesDesc::new().builder())
                .with_group(DrawFlat2DDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        // Finally, add the pass to the graph
        let _present = graph_builder
            .add_node(PresentNode::builder(factory, surface, color).with_dependency(sprite));

        graph_builder
    }
}
