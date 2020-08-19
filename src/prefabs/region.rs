use super::Prefab;
use std::rc::Rc;

pub struct Region<T: Prefab> {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    target: Rc<T>,
}

impl<T: Prefab> Region<T> {
    // TODO: We'll have to handle resizing
    pub fn new(x: i32, y: i32, w: i32, h: i32, target: Rc<T>) -> Self {
        Region { x, y, w, h, target }
    }

    pub fn check_target(&self, target: &T) -> bool {
        match target.transform() {
            Some(tf) => {
                self.x <= tf.x
                    && self.y <= tf.y
                    && tf.x <= (self.x + self.w)
                    && tf.y <= (self.y + self.h)
            }
            None => false,
        }
    }
}

impl<T: Prefab> Prefab for Region<T> {
    fn update(&mut self) {
        let target_collides = self.check_target(&self.target);
        println!("player collides: {}", target_collides);
    }
}
