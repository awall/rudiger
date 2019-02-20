extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct Time {
    pub dt: f64,
}

pub struct Screen<'a> {
    gl: &'a mut GlGraphics,
    context: graphics::Context,
    pub size: ScreenSize,
}

pub enum Event {
    Click(Point),
}

pub type Color = [f32; 4];
pub type Point = (f64, f64);
pub type ScreenSize = Point;
pub type Rectangle = (Point, Point);

impl<'a> Screen<'a> {
    pub fn draw_box_at_angle(&mut self, angle: f64) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (self.size.0 / 2.0, self.size.1 / 2.0);

        let transform = self
            .context
            .transform
            .trans(x, y)
            .rot_rad(angle)
            .trans(-25.0, -25.0);

        rectangle(RED, square, transform, self.gl);
    }

    pub fn draw_box(&mut self, color: Color, ((x0, y0), (x1, y1)): Rectangle) {
        use graphics::*;

        let square = rectangle::rectangle_by_corners(x0, y0, x1, y1);
        let transform = self
            .context
            .transform
            .trans(0.0, 0.0);

        rectangle(color, square, transform, self.gl);
    }
}

pub fn main_loop<STATE, RF, UF, EF>(title: &str, state: &mut STATE, render_func: RF, update_func: UF, event_func: EF)
where
    RF: Fn(Screen, &STATE),
    UF: Fn(Time, &mut STATE),
    EF: Fn(Event, ScreenSize, &mut STATE),
{
    let opengl = OpenGL::V3_2;

    let mut size = (200.0, 200.0);

    let mut window: Window = WindowSettings::new(title, [size.0, size.1])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut down = false;
    let mut last_pos = None;

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            use graphics::*;

            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            gl.draw(r.viewport(), |c, gl| {
                size = (r.width, r.height);
                clear(BLACK, gl);
                render_func(
                    Screen {
                        gl: gl,
                        size: (r.width, r.height),
                        context: c,
                    },
                    state,
                );
            });
        }

        if let Some(u) = e.update_args() {
            update_func(Time { dt: u.dt }, state);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                down = true;
            }
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                down = false;
                if let Some(pos) = last_pos {
                    let event = Event::Click(pos);
                    event_func(event, size, state);
                }
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            last_pos = Some((pos[0], pos[1]));
        }
    }
}
