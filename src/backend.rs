extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Screen<'a> {
    gl: &'a mut GlGraphics,
    args: &'a RenderArgs,
    context: &'a graphics::Context,
}

impl<'a> Screen<'a> {
    pub fn draw_box_at_angle(&mut self, angle: f64) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = (self.args.width / 2.0, self.args.height / 2.0);

        let transform = self.context.transform.trans(x, y)
                                              .rot_rad(angle)
                                              .trans(-25.0, -25.0);

        rectangle(RED, square, transform, self.gl);
    }
}

pub fn render<F>(gl: &mut GlGraphics, args: &RenderArgs, f: &F) where F: Fn(&mut Screen) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    gl.draw(args.viewport(), |c, gl| {
        clear(BLACK, gl);

        let mut screen = Screen {
            gl: gl,
            args: args,
            context: &c,
        };

        f(&mut screen)
    });
}