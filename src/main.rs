extern crate sdl2;

mod resources;
mod scenes;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rustling!", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let sprites = resources::load_sprites(&texture_creator);

    let mut mainscene = scenes::mainscene::MainScene::new(sprites);

    use scenes::Scene;
    mainscene.run(&mut canvas, &mut event_pump);
}
