mod backend;
use backend::*;

fn bounds((x, y): ScreenSize) -> Rectangle {
    ((x - 80.0, y - 80.0), (x - 20.0, y - 20.0))
}

fn inside(((x0,y0),(x1,y1)): Rectangle, (xp,yp): Point) -> bool {
    x0 <= xp && xp < x1 && y0 <= yp && yp < y1
}

fn draw(mut screen: Screen, on: bool) {
    const RED: Color = [1.0, 0.0, 0.0, 1.0];
    const GREEN: Color = [0.0, 1.0, 0.0, 1.0];

    screen.draw_box(
        if on { GREEN } else { RED },
        bounds(screen.size),
    );
}

fn handle_click(Event::Click(pos): Event, size: ScreenSize, on: &mut bool) {
    if inside(bounds(size), pos) {
        *on = !*on;
    }
}

fn main() {
    let mut on = true;

    main_loop(
        "rudiger",
        &mut on,
        |scrn, s| draw(scrn, *s),
        |time, s| (),
        |evnt, sz, s| handle_click(evnt, sz, s),
    );
}
