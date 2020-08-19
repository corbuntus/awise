use super::Canvas;
use super::Prefab;
use crate::sdl2::{pixels::Color, rect::Rect};

pub struct Region {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Region {
    // TODO: We'll have to handle resizing
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Region { x, y, w, h }
    }

    pub fn check_target(&self, target: &impl Prefab) -> bool {
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

impl Prefab for Region {
    fn paint_into(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .draw_rect(Rect::new(self.x, self.y, self.w as u32, self.h as u32))
            .unwrap();
    }
}
