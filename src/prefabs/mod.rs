use super::components;
use super::Canvas;

mod player;
mod tileset;

pub use player::{Player, PlayerState};
pub use tileset::Tileset;

pub trait Prefab {
    fn update(&mut self) {}
    fn paint_into(&self, _canvas: &mut Canvas) {}
}
