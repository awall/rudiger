mod backend;
use backend::*;

fn handle_render<T>(_size: ScreenSize, _state: &T) -> Shape {    
    let tri = equilateral_triangle(GREEN);
    let line = triangle_bottom_line(RED, 0.05);
    let both = Shape::join(tri, line);
    let both = both
        .scale(100.0)
        .translate([300.0, 300.0]);
    both
}

fn handle_time<T>(_time: Seconds, _state: &mut T) {
}

fn handle_event<T>(_event: Event, _size: ScreenSize, _state: &mut T) {
}

fn main() {
    let mut state = ();

    main_loop(
        "rudiger",
        &mut state,
        handle_render,
        handle_time,
        handle_event,
    );
}
