mod backend;
use backend::*;

fn draw(mut screen: Screen, angle: f64) {
   screen.draw_box_at_angle(angle)
}

fn main() {
  let mut rotation = 0.0;

  backend::main_loop(&mut rotation, 
    |r, scrn| draw(scrn, *r),
    |r, time| *r += 2.0 * time.dt,
  );
}