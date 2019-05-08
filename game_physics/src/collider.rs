use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};
use nalgebra::Isometry3;
use ncollide::{
    shape::ShapeHandle,
    world::{CollisionGroups, GeometricQueryType},
};
use nphysics::{material::BasicMaterial, object::ColliderHandle};

/// `ColliderType` maps to the `GeometricQueryType` internally used by the
/// physics world to differentiate between contact and proximity events.
///
/// e.g.:
/// - `ColliderType::Collider` ->  `GeometricQueryType::Contacts`
/// - `ColliderType::Trigger`  ->  `GeometricQueryType::Proximity`
///
/// The default `ColliderType`is `ColliderType::Collider`.
#[derive(Clone, Debug)]
pub enum ColliderType {
    Collider,
    Trigger,
}

impl Default for ColliderType {
    /// Default to `ColliderType::Collider` as it should be the most common
    /// case.
    fn default() -> Self {
        ColliderType::Collider
    }
}

//impl ColliderType {
//    pub fn to_geometric_query_type(
//        &self,
//        margin: f32,
//        prediction: f32,
//        angular_prediction: f32,
//    ) -> GeometricQueryType<f32> {
//        match *self {
//            ColliderType::Collider => {
//                GeometricQueryType::Contacts(margin + prediction * 0.5,
// angular_prediction)            }
//            ColliderType::Trigger => GeometricQueryType::Proximity(prediction
// * 0.5),        } }
//}

#[derive(Clone)]
pub struct Collider {
    pub(crate) handle: Option<ColliderHandle>,
    pub shape: ShapeHandle<f32>,
    pub query_type: ColliderType,
}

impl From<ShapeHandle<f32>> for Collider {
    fn from(shape: ShapeHandle<f32>) -> Self {
        Self {
            handle: None,
            shape,
            query_type: ColliderType::default(),
        }
    }
}

impl Component for Collider {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
