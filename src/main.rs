extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod backend;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


fn draw(screen: &mut backend::Screen, angle: f64) {
    screen.draw_box_at_angle(angle)
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
            backend::render(&mut gl, &r, &|mut screen| draw(&mut screen, rotation));
        }

        if let Some(u) = e.update_args() {
            rotation += 2.0 * u.dt;
        }
    }
}