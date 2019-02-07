extern crate ocl;
extern crate sdl2; 

use std::thread;

use ocl::{
    flags,
    Platform,
    Device,
    Context,
    Queue,
    Program,
    Buffer,
    Kernel,
};

fn fill(width: u32, height: u32) -> ocl::Result<Vec<u8>> {
    let src = r#"
        __kernel void fill(
            int width, int height,
            __global uchar *buffer
        ) {
            int idx = get_global_id(0);

            int2 pos = (int2)(idx % width, idx / width);
            int2 size = (int2)(width, height);

            uchar4 color = (uchar4)(convert_uchar2((255*pos)/size), 0, 1);
            vstore4(color, idx, buffer);
        }
    "#;

    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::builder()
    .platform(platform)
    .devices(device.clone())
    .build()?;

    let program = Program::builder()
    .devices(device)
    .src(src)
    .build(&context)?;

    let queue = Queue::new(&context, device, None)?;
    let dims = width*height;

    let buffer = Buffer::<u8>::builder()
    .queue(queue.clone())
    .flags(flags::MEM_READ_WRITE)
    .len(4*dims)
    .fill_val(0 as u8)
    .build()?;

    let kernel = Kernel::builder()
    .program(&program)
    .name("fill")
    .queue(queue.clone())
    .global_work_size(dims)
    .arg(&(width as i32))
    .arg(&(height as i32))
    .arg(&buffer)
    .build()?;

    unsafe {
        kernel.cmd()
        .queue(&queue)
        .global_work_offset(kernel.default_global_work_offset())
        .global_work_size(dims)
        .local_work_size(kernel.default_local_work_size())
        .enq()?;
    }

    let mut vec = vec![0 as u8; buffer.len()];
    buffer.cmd()
    .queue(&queue)
    .offset(0)
    .read(&mut vec)
    .enq()?;

    Ok(vec)
}

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = (800u32, 600u32);
 
    let window = video_subsystem.window("HyperTrace", size.0, size.1)
    .position_centered().resizable().build().unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(
        PixelFormatEnum::RGBA32,
        TextureAccess::Streaming,
        size.0,
        size.1,
    ).unwrap();
 
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

        let data = fill(size.0, size.1).unwrap();
        texture.update(None, &data, 4*(size.0 as usize)).unwrap();
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        thread::sleep(Duration::from_millis(20));
    }
}
