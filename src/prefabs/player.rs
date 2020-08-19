use super::components;
use super::Canvas;
use super::Prefab;

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

pub struct Player {
    pub state: PlayerState,
    pub transform: components::Transform,
    pub sprite: components::AnimatedSprite,
    pub hidden: bool,
}

impl Prefab for Player {
    fn transform(&self) -> Option<&components::Transform> {
        Some(&self.transform)
    }

    fn paint_into(&self, canvas: &mut Canvas) {
        if self.hidden {
            return;
        }
        let horizontal_mirror =
            self.state == PlayerState::MovingLeft || self.state == PlayerState::IdleLeft;
        self.sprite
            .paint_into(&self.transform, canvas, horizontal_mirror)
            .unwrap();
    }
}

impl Player {
    pub fn new(sprite: components::AnimatedSprite) -> Self {
        let mut transform = components::Transform::new();
        transform.scale = 2.0;
        Player {
            state: PlayerState::IdleDown,
            transform,
            sprite,
            hidden: false,
        }
    }
}
