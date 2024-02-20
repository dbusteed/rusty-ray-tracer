use glam::f32::Vec3;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::f32::consts::PI;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = PI / 2.0;

#[derive(Clone, Copy)]
struct Material {
    diffuse_color: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    fn new(c: Vec3, r: f32, m: Material) -> Self {
        Sphere {
            center: c,
            radius: r,
            material: m
        }
    }

    fn ray_intersect(&self, orig: Vec3, dir: Vec3, t0: &mut f32) -> bool {
        let big_l: Vec3 = self.center - orig;
        let tca: f32 = big_l.dot(dir);
        let d2: f32 = big_l.dot(big_l) - (tca * tca);
        if d2 > (self.radius * self.radius) {
            return false;
        }
        let thc: f32 = f32::sqrt(self.radius*self.radius - d2);
        *t0 = tca - thc;
        let t1 = tca + thc;
        if *t0 < 0.0 { *t0 = t1 };
        if *t0 < 0.0 { return false; }
        return true;
    }
}

fn scene_intersect(orig: Vec3, dir: Vec3, spheres: &Vec<Sphere>, hit: &mut Vec3, big_n: &mut Vec3, material: &mut Material) -> bool {
    let mut sphere_dist: f32 = f32::MAX;
    for sphere in spheres.iter() {
        let mut dist: f32 = 0.0;
        if sphere.ray_intersect(orig, dir, &mut dist) && dist < sphere_dist {
            sphere_dist = dist;
            *hit = orig + dir * dist;
            *big_n = (*hit - sphere.center).normalize();
            *material = sphere.material;
        }
    }
    sphere_dist < 1000.0 
}

fn cast_ray(orig: Vec3, dir: Vec3, spheres: &Vec<Sphere>) -> Vec3 {
    let mut point = Vec3::ZERO;
    let mut big_n = Vec3::ZERO;
    let mut material = Material { diffuse_color: Vec3::ZERO };

    if !scene_intersect(orig, dir, &spheres, &mut point, &mut big_n, &mut material) {
        return Vec3::new(0.2, 0.7, 0.8); // background material 
    }
    
    return material.diffuse_color;
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
    let ivory = Material { diffuse_color: Vec3::new(0.4, 0.4, 0.3) };
    let red_rubber = Material { diffuse_color: Vec3::new(0.3, 0.1, 0.1) };

    let spheres = vec![
        Sphere::new(Vec3::new(-3.0,  0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
        Sphere::new(Vec3::new( 1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new( 7.0,  5.0, -18.0), 4.0, ivory),
    ];

    let mut image = vec![Vec3::ZERO; WIDTH * HEIGHT];

    let mut x: f32;
    let mut y: f32;
    let mut dir: Vec3;
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            x = (2.0 * (col as f32 + 0.5) / (WIDTH as f32) - 1.0) * (FOV / 2.0).tan() * (WIDTH as f32 / HEIGHT as f32);
            y = -(2.0 * (row as f32 + 0.5) / (HEIGHT as f32) - 1.0) * (FOV / 2.0).tan();
            dir = Vec3::new(x, y, -1.0).normalize();
            image[row * WIDTH + col] = cast_ray(Vec3::ZERO, dir, &spheres);
        }
    }
    
    let _ = render(&image);
}
