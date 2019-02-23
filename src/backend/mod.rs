extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use graphics::Transformed;
use graphics::math::Matrix2d;
use graphics::math::identity;

pub mod color;
pub use color::*;

pub mod types;
pub use types::*;

pub mod shape;
pub use shape::*;

pub struct Screen<'a> {
    gl: &'a mut GlGraphics,
    context: graphics::Context,
    pub size: ScreenSize,
}

pub enum Event {
    Click(Point),
}

impl<'a> Screen<'a> {
    fn multiply(&mut self, matrix: Matrix2d) {
        self.context = self.context.append_transform(matrix);
    }

    fn draw_polygon(&mut self, color: &Color, points: &[Point]) {
        use graphics::polygon::*;
        use graphics::*;

        let poly = Polygon::new(color.components());
        poly.draw(points, &DrawState::default(), self.context.transform, self.gl);
    }

    fn draw_lines(&mut self, color: &Color, points: &[Point], _open: bool) {
        use graphics::polygon::*;
        use graphics::*;

        let poly = Polygon::new(color.components());
        poly.draw(points, &DrawState::default(), identity(), self.gl);
    }
}

pub fn main_loop<STATE, RF, UF, EF>(title: &str, state: &mut STATE, render_func: RF, update_func: UF, event_func: EF)
where
    RF: Fn(Screen, &STATE),
    UF: Fn(Seconds, &mut STATE),
    EF: Fn(Event, ScreenSize, &mut STATE),
{
    let opengl = OpenGL::V3_2;

    let mut size = [200.0, 200.0];

    let mut window: Window = WindowSettings::new(title, size)
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut last_pos = None;

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            use graphics::*;

            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            gl.draw(r.viewport(), |c, gl| {
                size = [r.width, r.height];
                clear(BLACK, gl);
                render_func(
                    Screen {
                        gl: gl,
                        size: [r.width, r.height],
                        context: c,
                    },
                    state,
                );
            });
        }

        if let Some(u) = e.update_args() {
            update_func(Seconds(u.dt), state);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
            }
        }

        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if let Some(pos) = last_pos {
                    let event = Event::Click(pos);
                    event_func(event, size, state);
                }
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            last_pos = Some(pos);
        }
    }
}