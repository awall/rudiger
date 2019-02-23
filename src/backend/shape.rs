use super::*;

use graphics::math::{Matrix2d, identity, multiply, scale, translate};

struct Polygon {
    color: Color,
    points: Vec<Point>,
}

struct Lines {
    color: Color,
    points: Vec<Point>,
    open: bool,
}

struct Transform {
    shape: Box<Shape>,
    transform: Matrix2d,
}

pub enum Shape {
    Polygon(Polygon),
    Lines(Lines),
    Transform(Transform),
}

pub fn equilateral_triangle(color: Color) -> Shape {
    let trig = 5.0_f64.sqrt() * 0.25;
    Shape::Polygon(Polygon {
        color: color,
        points: vec![
            [0.0, -trig], 
            [0.5, 0.25], 
            [-0.5, 0.25],
        ],
    })
}

impl Shape {
    pub fn scale(self, scale: f64) -> Self {
        self.scale_both([scale, scale])
    }

    pub fn scale_both(self, [x, y]: Point) -> Self {
        self.multiply(scale(x, y))
    }

    pub fn translate(self, p: Point) -> Self {
        self.multiply(translate(p))
    }

    fn multiply(self, matrix: Matrix2d) -> Self {
        let t = if let Shape::Transform(t) = self {
            Transform { shape: t.shape, transform: multiply(matrix, t.transform) }
        } else {
            Transform { shape: Box::new(self), transform: matrix }
        };
        Shape::Transform(t)
    }
}

impl<'a> Screen<'a> {
    pub fn draw(&mut self, shape: &Shape) {
        match shape {
            Shape::Lines(l) => self.draw_lines(&l.color, &l.points, l.open),
            Shape::Polygon(p) => self.draw_polygon(&p.color, &p.points),
            Shape::Transform(t) => {
                self.multiply(t.transform);
                self.draw(&t.shape);
            },
        }
    }
}