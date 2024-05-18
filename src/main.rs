extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings};
use rand::random;

const WINDOW_SIZE: f64 = 1000.0;
const CENTER_COORD: f64 = WINDOW_SIZE / 2.0;
const NUM_POINTS: i32 = 300;
const POINT_RADIUS: f64 = 2.5;
const ORBIT_RADIUS_FACTOR: f64 = 1.5;
const BASE_SPEED: f64 = 0.02;

struct FlowingPoint {
    position: [f64; 2],
    radius: f64,
    speed: f64,
    angle: f64,
    orbit_radius: f64,
    color: [f32; 4],
}

pub struct App {
    gl: GlGraphics,
    flowing_points: Vec<FlowingPoint>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            for point in &self.flowing_points {
                let transform = c.transform.trans(point.position[0], point.position[1]);

                let shape = ellipse::circle(0.0, 0.0, point.radius);
                ellipse(point.color, shape, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for point in &mut self.flowing_points {
            point.angle += point.speed * args.dt;
            point.position[0] = CENTER_COORD + point.orbit_radius * point.angle.cos();
            point.position[1] = CENTER_COORD + point.orbit_radius * point.angle.sin();
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Euler's Dream", [WINDOW_SIZE, WINDOW_SIZE])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        flowing_points: (0..NUM_POINTS)
            .map(|i| {
                let orbit_radius = (i as f64 + 1.0) * ORBIT_RADIUS_FACTOR;
                let color: [f32; 4] = [random::<f32>(), random::<f32>(), random::<f32>(), 1.0];
                FlowingPoint {
                    position: [CENTER_COORD, CENTER_COORD],
                    radius: POINT_RADIUS,
                    speed: orbit_radius * BASE_SPEED,
                    angle: 0.0,
                    orbit_radius,
                    color,
                }
            })
            .collect(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
