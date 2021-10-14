mod math;

use std::path::Path;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use crate::math::vector2::Vector2;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)]
struct Boid {
    pos: Vector2,
    rad: f32
}

impl Boid {
    fn new(pos: Vector2, rad: f32) -> Boid {
        Boid { pos, rad }
    }

    fn translate(&mut self, dir: Vector2) {
        self.pos = self.pos.add(&dir);
    }

    fn render(&self, canvas: &Canvas<Window>, color: Color) {
        canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, self.rad as i16, color).unwrap();
    }
}

fn create_window(video_subsystem: &VideoSubsystem, title: &str) -> Window {
    video_subsystem
        .window(title, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()    
        .build()
        .unwrap()
}

fn create_canvas(window: Window) -> Canvas<Window> {
    window.into_canvas().present_vsync().build().unwrap()
}

fn show_fps<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font_fps: &Font,
    mspf: f32,
    fps: f32,
) -> (Texture<'a>, Rect) {
    let fps_string = format!("ms/f: {:7.3}, fps: {:7.3}", mspf, fps);
    let surface = font_fps.render(&fps_string).blended(Color::GREEN).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let texture_rect = Rect::new((SCREEN_WIDTH - width) as i32, 0, width, height);
    (texture, texture_rect)
}

#[allow(unused_variables)]
fn main() {
    // Initialize
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let perf_freq = timer_subsystem.performance_frequency();

    // Rendering bindings
    let window = create_window(&video_subsystem, "Boids");
    let mut canvas = create_canvas(window);
    let texture_creator = canvas.texture_creator();

    // Font bindings
    let font_path = Path::new("./src/fonts/MesloLGS NF Regular.ttf");
    let font_fps = ttf_context.load_font(font_path, 14).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_perf_counter = timer_subsystem.performance_counter();
    let mut deltatime = 0f32;
    let mut toggle_fps = false;

    let speed = 1f32; // 40f32;
    let radius = 8f32;

    let mut boids: Vec<Boid> = vec![];
    boids.push(Boid::new(Vector2 { x: 400.0, y: 300.0 }, radius));
    boids.push(Boid::new(Vector2 { x: 100.0, y: 100.0 }, radius));

    // let mut x1 = SCREEN_WIDTH as f32 / 2.0 - 3.0;
    // let mut x2 = SCREEN_WIDTH as f32 / 2.0 + 3.0;
    // let mut x3 = SCREEN_WIDTH as f32 / 2.0;
    // let speed = 10f32;

    // let v1 = Vector2::new(1.0, 3.0);
    // let v2 = Vector2::right();
    // println!("v1 + v2 = {:?}", v1.add(&v2));

    // Game loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
                    toggle_fps = !toggle_fps;
                }
                _ => {},
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // RULE 1: Going towards the center of mass
        let mut average_position = Vector2::zero();
        for boid in &boids {
            average_position = average_position.add(&boid.pos);
        }
        let n_boids = boids.len() as f32;
        let average_position = Vector2::new(average_position.x / n_boids, average_position.y / n_boids);

        for boid in &mut boids {
            let direction = average_position.sub(&boid.pos);
            boid.translate(Vector2 { x: direction.x * speed * deltatime, y: direction.y * speed * deltatime });
            boid.render(&canvas, Color::RGB(123, 145, 210));
        }

        // FPS calculations
        let end_perf_counter = timer_subsystem.performance_counter();
        let perf_counter_elapsed = end_perf_counter - last_perf_counter;
        deltatime = perf_counter_elapsed as f32 / perf_freq as f32;
        let mspf = 1_000f32 * deltatime;
        let fps = perf_freq as f32 / perf_counter_elapsed as f32;
        last_perf_counter = end_perf_counter;
        if toggle_fps {
            let (fps_texture, fps_rect) = show_fps(&texture_creator, &font_fps, mspf, fps);
            canvas.set_draw_color(Color::BLACK);
            canvas.fill_rect(fps_rect).unwrap();
            canvas.copy(&fps_texture, None, Some(fps_rect)).unwrap(); // FONT STUFF
        }

        canvas.present();
    }
}
