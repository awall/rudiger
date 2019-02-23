mod backend;
use backend::*;

fn draw<T>(mut screen: Screen, _state: &T) {
    let triangle = 
        equilateral_triangle(GREEN)
        .scale(200.0)
        .translate([300.0, 300.0]);

    screen.draw(&triangle);
    
}

fn handle_event<T>(_event: Event, _size: ScreenSize, _state: &mut T) {
}

fn main() {
    let mut state = ();

    main_loop(
        "rudiger",
        &mut state,
        |screen, state| draw(screen, state),
        |_time, _state| (),
        |event, size, state| handle_event(event, size, state),
    );
}
