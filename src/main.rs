use std::thread;

use sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
use raytracer::Worker;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = (800, 600);
 
    let window = video_subsystem.window("HyperTrace", size.0 as u32, size.1 as u32)
    .position_centered().resizable().build().unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(
        PixelFormatEnum::RGBA32,
        TextureAccess::Streaming,
        size.0 as u32,
        size.1 as u32,
    ).unwrap();

    let worker = Worker::new().unwrap();
    let mut screen = worker.create_screen(size).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                _ => {}
            }
        }

        worker.render(&mut screen).unwrap();
        let data = screen.read().unwrap();
        texture.update(None, &data, 4*(screen.size().0 as usize)).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        thread::sleep(Duration::from_millis(20));
    }
}
