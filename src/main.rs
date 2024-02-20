use glam::f32::Vec3;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn render(image: &[Vec3]) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Rusty Ray Tracing", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    for (i, pixel) in image.iter().enumerate() {
        let row = i / WIDTH;
        let col = i % WIDTH;
        canvas.pixel(
            col as i16,
            row as i16,
            Color::RGB(pixel.x as u8, pixel.y as u8, pixel.z as u8),
        )?;
    }
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() {
    let mut image = [Vec3::ZERO; WIDTH * HEIGHT];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            image[row * WIDTH + col] =
                Vec3::new(
                    (row as f32 / HEIGHT as f32) * 255.0, 
                    (col as f32 / WIDTH as f32) * 255.0, 
                    0.0
                );
        }
    }
    let _ = render(&image);
}
