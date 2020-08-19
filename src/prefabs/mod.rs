use super::components;
use super::Canvas;

mod player;
mod region;
mod tileset;

pub use player::{Player, PlayerState};
pub use region::Region;
pub use tileset::Tileset;

pub trait Prefab {
    fn transform(&self) -> Option<&components::Transform> {
        None
    }

    fn update(&mut self) {}
    fn paint_into(&self, _canvas: &mut Canvas) {}
}
