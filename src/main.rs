extern crate sdl2;

mod components;
mod prefabs;
mod resources;
mod scenes;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

const WINDOW_WIDTH: u32 = 950;
const WINDOW_HEIGHT: u32 = 600;

// TODO: The next step would be either to make basic UI components and prefabs,
// do something with the tileset, or start with regions. Regions are parts of
// the surface of the screen that toggle certain actions (e.g. change the scene,
// hurt the player, interact with some{one,thing}, etc).

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
    let texture_container = resources::load_sprites(&texture_creator);

    let mut mainscene = scenes::mainscene::MainScene::new(texture_container);

    use scenes::Scene;
    mainscene.run(&mut canvas, &mut event_pump);
}
