mod backend;
use backend::*;

struct State {
    orientation: i32,
    position: [i32; 2],
}

enum Rotation {
    Left, 
    Right,
} 

enum Movement {
    Forward,
    Backward,
    Spin,
}

struct Move(Movement, Rotation);

fn modulo(x: i32, m: i32) -> i32 {
    ((x % m) + m) % m
}

fn z(x: i32) -> i32 {
    if x < 0 { 0 } else { x }
}

fn i(x: i32) -> i32 {
    -x
}

fn do_move(Move(m, r) : Move, state: &mut State) {
    let o = modulo(state.orientation, 6);

    let dir = match r {
        Rotation::Left => -1,
        Rotation::Right => 1,
    };

    let p = match m {
        Movement::Forward =>
            if o == 0 { [0, 1] }
            else if o == 1 { [-1, 0] }
            else if o == 2 { [-1, 0] }
            else if o == 3 { [0, -1] }
            else if o == 4 { [1, 0] }
            else if o == 5 { [1, 0] }
            else { [0, 0] },
        Movement::Backward =>
            if o == 0 { [-dir, 0] }
            else if o == 1 { [z(i(dir)), i(z(dir))] }
            else if o == 2 { [z(dir), 1 0]}
            else if o == 3 { [dir, 0] }
            else if o == 4 { [-1, 0] }
            else if o == 5 { [0, -1] }
            else { [0, 0] },
        _ =>
            [0, 0],
    };

    state.position[0] += p[0];
    state.position[1] += p[1];

    let multiplier = match m {
        Movement::Forward => 1,
        Movement::Backward => 1,
        Movement::Spin => 2,
    };

    state.orientation += multiplier * dir;
}

fn handle_render(_size: ScreenSize, state: &State) -> Shape {
    let even = (state.position[0] + state.position[1]) % 2 == 0;
    let h = if even { 1.0 } else { -1.0 };

    //let trig = 5.0_f64.sqrt() * 0.25;
    let a = [0.0, -0.5 * h];
    let b = [0.5, 0.5 * h];
    let c = [-0.5, 0.5 * h];

    let lp = match modulo(state.orientation, 6) {
        0 => [b,c],
        1 => [c,a],
        2 => [a,c],
        3 => [c,b],
        4 => [b,a],
        5 => [a,b],
        _ => [a,a], // error
    };

    let tri = polygon(GREEN, vec![a,b,c]);
    let line = line(RED, 0.15, lp);

    tri.behind(line)
        .translate([0.5 * state.position[0] as f64, 1.0 * state.position[1] as f64])
        .scale(100.0)
        .translate([300.0, 300.0])
}

fn handle_time<T>(_time: Seconds, _state: &mut T) {
}

fn handle_event(event: Event, _size: ScreenSize, mut state: &mut State) {
    if let Event::KeyPress(key) = event {
        let k = match key {
            Key::Q => Some(Move(Movement::Forward, Rotation::Left)),
            Key::W => Some(Move(Movement::Forward, Rotation::Right)),
            Key::A => Some(Move(Movement::Spin, Rotation::Left)),
            Key::S => Some(Move(Movement::Spin, Rotation::Right)),
            Key::Z => Some(Move(Movement::Backward, Rotation::Left)),
            Key::X => Some(Move(Movement::Backward, Rotation::Right)),
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
