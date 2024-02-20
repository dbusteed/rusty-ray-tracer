use glam::f32::Vec3;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;
use std::f32::consts::PI;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = PI / 2.0;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn ray_intersect(&self, orig: Vec3, dir: Vec3, _dist: f32) -> bool {
        let big_l: Vec3 = self.center - orig;
        let tca: f32 = big_l.dot(dir);
        let d2: f32 = big_l.dot(big_l) - (tca * tca);
        if d2 > (self.radius * self.radius) {
            return false;
        }
        let thc: f32 = f32::sqrt(self.radius*self.radius - d2);
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 { t0 = t1 };
        if t0 < 0.0 { return false; }
        return true;
    }
}

fn cast_ray(orig: Vec3, dir: Vec3, sphere: &Sphere) -> Vec3 {
    let sphere_dist: f32 = f32::MAX;
    if !sphere.ray_intersect(orig, dir, sphere_dist) {
        return Vec3::new(0.2, 0.7, 0.8);
    }
    return Vec3::new(0.4, 0.4, 0.3);
}

fn render(image: &Vec<Vec3>) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("Rusty Ray Tracing", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    for (i, pixel) in image.iter().enumerate() {
        let row = i / WIDTH;
        let col = i % WIDTH;
        canvas.pixel(
            col as i16,
            row as i16,
            Color::RGB(
                (pixel.x * 255.0) as u8,
                (pixel.y * 255.0) as u8,
                (pixel.z * 255.0) as u8
            ),
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
    let sphere = Sphere {
        center: Vec3::new(-3.0, 0.0, -16.0),
        radius: 2.0,
    };

    let mut image = vec![Vec3::ZERO; WIDTH * HEIGHT];

    let mut x: f32;
    let mut y: f32;
    let mut dir: Vec3;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            x = (2.0 * (col as f32 + 0.5) / (WIDTH as f32) - 1.0) * (FOV / 2.0).tan() * (WIDTH as f32 / HEIGHT as f32);
            y = -(2.0 * (row as f32 + 0.5) / (HEIGHT as f32) - 1.0) * (FOV / 2.0).tan();
            dir = Vec3::new(x, y, -1.0).normalize();
            image[row * WIDTH + col] = cast_ray(Vec3::ZERO, dir, &sphere);
        }
    }
    let _ = render(&image);
}
