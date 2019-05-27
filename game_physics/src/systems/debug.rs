use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadStorage, Resources, System, SystemData, Write},
    renderer::{
        debug_drawing::{DebugLines, DebugLinesParams},
        palette::Srgba,
    },
};

use crate::collider::{PhysicsCollider, Shape};

/// The `DebugSystem`s handles the drawing of `DebugLines` elements for
/// `PhysicsCollider`s. This visualises the `PhysicsCollider` and enables easier
/// debugging of collisions.
#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        ReadStorage<'s, PhysicsCollider>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (physics_colliders, transforms, mut debug_lines) = data;

        // iterate over PhysicsColliders and their Transforms and draw lines accordingly
        for (physics_collider, transform) in (&physics_colliders, &transforms).join() {
            let transform: &Transform = transform;
            let physics_collider: &PhysicsCollider = physics_collider;

            // depending on the Shape we draw the DebugLines differently; right now we only
            // support Shape::Rectangle
            match physics_collider.shape {
                Shape::Rectangle(width, height, _) => {
                    // center of the Collider, based on Transform and offset
                    let x = transform.translation().x.as_f32()
                        + physics_collider.offset_from_parent.translation.vector.x;
                    let y = transform.translation().y.as_f32()
                        + physics_collider.offset_from_parent.translation.vector.y;
                    let z = transform.translation().z.as_f32()
                        + physics_collider.offset_from_parent.translation.vector.z;

                    // color based on type
                    let color = if physics_collider.sensor {
                        Srgba::new(0.13, 0.65, 0.94, 1.0) // 1 or 1/255?!
                    } else {
                        Srgba::new(0.81, 0.0, 0.5, 1.0) // 1 or 1/255?!
                    };

                    // draw top line
                    debug_lines.draw_line(
                        [x - width / 2.0, y + height / 2.0, z].into(),
                        [x + width / 2.0, y + height / 2.0, z].into(),
                        color,
                    );

                    // draw right line
                    debug_lines.draw_line(
                        [x + width / 2.0, y + height / 2.0, z].into(),
                        [x + width / 2.0, y - height / 2.0, z].into(),
                        color,
                    );

                    // draw bottom line
                    debug_lines.draw_line(
                        [x + width / 2.0, y - height / 2.0, z].into(),
                        [x - width / 2.0, y - height / 2.0, z].into(),
                        color,
                    );

                    // draw bottom line
                    debug_lines.draw_line(
                        [x - width / 2.0, y - height / 2.0, z].into(),
                        [x - width / 2.0, y + height / 2.0, z].into(),
                        color,
                    );
                }
                _ => {}
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        info!("DebugSystem.setup");
        Self::SystemData::setup(res);

        // initialise required resources
        res.entry::<DebugLines>().or_insert(DebugLines::new());
        res.entry::<DebugLinesParams>()
            .or_insert(DebugLinesParams { line_width: 1.0 });
    }
}
