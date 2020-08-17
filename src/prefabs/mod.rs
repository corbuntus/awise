use super::components;
use super::Canvas;

pub trait Prefab {
    fn update(&mut self) {}
    fn paint_into(&self, _canvas: &mut Canvas) {}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PlayerState {
    IdleLeft,
    IdleRight,
    IdleUp,
    IdleDown,
    MovingLeft,
    MovingDown,
    MovingUp,
    MovingRight,
}

impl PlayerState {
    pub fn is_idle(self) -> bool {
        self == Self::IdleLeft
            || self == Self::IdleRight
            || self == Self::IdleDown
            || self == Self::IdleUp
    }
}

pub struct Player {
    pub state: PlayerState,
    pub transform: components::Transform,
    pub sprite: components::AnimatedSprite,
}

impl Prefab for Player {
    fn paint_into(&self, canvas: &mut Canvas) {
        let horizontal_mirror =
            self.state == PlayerState::MovingLeft || self.state == PlayerState::IdleLeft;
        self.sprite
            .paint_into(&self.transform, canvas, horizontal_mirror)
            .unwrap();
    }
}

impl Player {
    pub fn new(sprite: components::AnimatedSprite) -> Self {
        Player {
            state: PlayerState::IdleDown,
            transform: components::Transform::new(),
            sprite,
        }
    }
}
