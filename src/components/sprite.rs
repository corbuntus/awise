use super::sdl2::render::Texture;
use super::transform::Transform;
use crate::Canvas;
use std::rc::Rc;

pub struct AnimatedSprite {
    pub track: Rc<[Rc<Sprite>]>,
    pub index: usize,
}

impl AnimatedSprite {
    pub fn new(sprites: Rc<[Rc<Sprite>]>) -> Self {
        AnimatedSprite {
            track: sprites,
            index: 0,
        }
    }

    pub fn paint_into(
        &self,
        transform: &Transform,
        canvas: &mut Canvas,
        horizontal_mirror: bool,
    ) -> Result<(), String> {
        self.track[self.index].paint_into(transform, canvas, horizontal_mirror)
    }
}

pub struct Sprite {
    pub texture: Texture,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn new(texture: sdl2::render::Texture) -> Self {
        let query = texture.query();
        let width = query.width;
        let height = query.height;
        Sprite {
            texture,
            width,
            height,
        }
    }

    pub fn paint_into(
        &self,
        transform: &Transform,
        canvas: &mut Canvas,
        horizontal_mirror: bool,
    ) -> Result<(), String> {
        use sdl2::rect::Rect;
        let true_width = (transform.scale * self.width as f32).floor() as u32;
        let true_height = (transform.scale * self.height as f32).floor() as u32;
        canvas.copy_ex(
            &self.texture,
            Some(Rect::new(0, 0, self.width, self.height)),
            Some(Rect::new(
                transform.x - (true_width / 2) as i32,
                transform.y - (true_height / 2) as i32,
                true_width,
                true_height,
            )),
            0.0,
            None,
            horizontal_mirror,
            false,
        )
    }
}

unsafe impl Sync for Sprite {}
unsafe impl Send for Sprite {}
