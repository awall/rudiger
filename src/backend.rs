extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Time {
    pub dt: f64,
}

pub struct Screen<'a> {
    gl: &'a mut GlGraphics,
    args: RenderArgs,
    context: graphics::Context,
}

impl<'a> Screen<'a> {
    pub fn draw_box_at_angle(&mut self, angle: f64) {
        use graphics::*;

        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (self.args.width / 2.0, self.args.height / 2.0);

        let transform = self.context.transform.trans(x, y)
                                              .rot_rad(angle)
                                              .trans(-25.0, -25.0);

        rectangle(RED, square, transform, self.gl);
    }
}

pub fn main_loop<STATE, RF, UF>(state: &mut STATE, render_func: RF, update_func: UF) 
    where RF: Fn(&STATE, Screen), UF: Fn(&mut STATE, Time)
{
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            use graphics::*;

            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
            gl.draw(r.viewport(), |c, gl| {
                clear(BLACK, gl);
                render_func(state, Screen {
                    gl: gl,
                    args: r,
                    context: c,
                });
            });
        }

        if let Some(u) = e.update_args() {
            let time = Time { dt: u.dt };
            update_func(state, time);
        }
    };
}