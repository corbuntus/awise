#[derive(Debug)]
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            x: 0,
            y: 0,
            scale: 1.0,
        }
    }
}
