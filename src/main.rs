mod backend;
use backend::*;

struct State {
    orientation: i32,
    position: [i32; 2],
}

enum Move {
    ForwardLeft,
    ForwardRight,
    SpinLeft,
    SpinRight,
    BackwardLeft,
    BackwardRight,
}

fn do_move(m : Move, state: &mut State) {
    let o = state.orientation % 6;

    let p = match m {
        Move::ForwardLeft => 
            if o == 0 { [0, 1] }
            else if o == 1 { [-1, 0] } 
            else if o == 2 { [-1, 0] } 
            else if o == 3 { [0, -1] } 
            else if o == 4 { [1, 0] } 
            else { [1, 0] },

        _ => 
            [0, 0],
    };

    state.position[0] += p[0];
    state.position[1] += p[1];

    match m {
        Move::ForwardLeft => state.orientation -= 1,
        Move::SpinLeft => state.orientation -= 2,
        Move::BackwardLeft => state.orientation -= 1,
        Move::ForwardRight => state.orientation += 1,
        Move::SpinRight => state.orientation += 2,
        Move::BackwardRight => state.orientation += 1,
    }
}

fn handle_render(_size: ScreenSize, state: &State) -> Shape {
    let trig = 5.0_f64.sqrt() * 0.25;
    let a = [0.0, -trig];
    let b = [0.5, 0.25];
    let c = [-0.5, 0.25];

    let tri = polygon(GREEN, vec![a,b,c]);
    let line = line(RED, 0.15, [b, c]);

    tri.behind(line)
        .rotate(Degrees(state.orientation as f64 * 60.0))
        .translate([0.5 * state.position[0] as f64, 0.5 * state.position[1] as f64])
        .scale(100.0)
        .translate([300.0, 300.0])
}

fn handle_time<T>(_time: Seconds, _state: &mut T) {
}

fn handle_event(event: Event, _size: ScreenSize, mut state: &mut State) {
    if let Event::KeyPress(key) = event {
        let k = match key {
            Key::Q => Some(Move::ForwardLeft),
            Key::W => Some(Move::ForwardRight),
            Key::A => Some(Move::SpinLeft),
            Key::S => Some(Move::SpinRight),
            Key::Z => Some(Move::BackwardLeft),
            Key::X => Some(Move::BackwardRight),
            _ => None,
        };
        
        if let Some(m) = k { 
            do_move(m, &mut state);
        }
    }
}

fn main() {
    let mut state = State {
        orientation: 0,
        position: [0, 0],
    };

    main_loop(
        "rudiger",
        &mut state,
        handle_render,
        handle_time,
        handle_event,
    );
}
