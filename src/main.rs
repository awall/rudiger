extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn render(gl: &mut GlGraphics, args: &RenderArgs, rotation: f64) {
    use graphics::*;

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    let square = rectangle::square(0.0, 0.0, 50.0);
    let (x, y) = (args.width / 2.0, args.height / 2.0);

    gl.draw(args.viewport(), |c, gl| {
        clear(GREEN, gl);

        let transform = c.transform.trans(x, y)
                                    .rot_rad(rotation)
                                    .trans(-25.0, -25.0);

        rectangle(RED, square, transform, gl);
    });
}

fn main() {
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
    let mut rotation = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render(&mut gl, &r, rotation);
        }

        if let Some(u) = e.update_args() {
            rotation += 2.0 * u.dt;
        }
    }
}