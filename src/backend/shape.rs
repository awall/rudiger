use super::*;

use graphics::math::{Matrix2d, identity, multiply, scale, translate};

enum ShapePrivate {
    Polygon {
        color: Color,
        points: Vec<Point>,
    },
    Lines {
        color: Color,
        points: Vec<Point>,
        open: bool,
    },
    Transform {
        shape: Box<Shape>,
        transform: Matrix2d,
    }
}

pub struct Shape(ShapePrivate);

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

    fn multiply(mut self, matrix: Matrix2d) -> Self {
        if let Shape::Transform(ref mut t) = self {
            t.transform = multiply(matrix, t.transform);
            self
        } else {
            Shape::Transform(Transform { shape: Box::new(self), transform: matrix })
        }
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