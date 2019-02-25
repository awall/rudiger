use super::*;

use graphics::math::{Matrix2d, rotate_radians, multiply, scale, translate};

enum ShapePrivate {
    Polygon {
        color: Color,
        points: Vec<[f64; 2]>,
    },
    Line {
        color: Color,
        thickness: f64,
        points: [f64; 4],
    },
    Transform {
        shape: Box<ShapePrivate>,
        transform: Matrix2d,
    },
    Join {
        shapes: Vec<ShapePrivate>,
    },
}

pub struct Shape(ShapePrivate);

use ShapePrivate::*;

pub fn line(color: Color, thickness: f64, [[x1,y1],[x2,y2]]: [[f64; 2]; 2]) -> Shape {
    Shape(Line {
        color: color,
        thickness: thickness,
        points: [x1,y1,x2,y2],
    })
}

pub fn polygon(color: Color, points: Vec<Point>) -> Shape {
    Shape(Polygon {
        color: color,
        points: points,
    })
}

impl Shape {
    pub fn join(mut s1: Shape, mut s2: Shape) -> Self {
        if let Join { ref mut shapes } = s1.0 {
            if let Join { shapes: ref mut shapes2 } = s2.0 {
                shapes.append(shapes2);
            } else {
                shapes.push(s2.0);
            }
            s1
        } else if let Join { ref mut shapes } = s2.0 {
            shapes.insert(0, s1.0);
            s2
        } else {
            Shape(Join { shapes: vec![s1.0, s2.0] })
        }
    }

    pub fn behind(self, s: Shape) -> Self {
        Self::join(self, s)
    }

    pub fn scale(self, scale: f64) -> Self {
        self.scale_both([scale, scale])
    }

    pub fn rotate(self, Degrees(angle): Degrees) -> Self {
        self.multiply(rotate_radians(angle.deg_to_rad()))
    }

    pub fn scale_both(self, [x, y]: Point) -> Self {
        self.multiply(scale(x, y))
    }

    pub fn translate(self, p: Point) -> Self {
        self.multiply(translate(p))
    }

    fn multiply(mut self, matrix: Matrix2d) -> Self {
        if let Transform { transform: ref mut t, .. } = self.0 {
            *t = multiply(matrix, *t);
            self
        } else {
            Shape(Transform { shape: Box::new(self.0), transform: matrix })
        }
    }
}

impl<'a> Screen<'a> {
    pub fn draw(&mut self, shape: Shape) {
        self.draw_private(shape.0);
    }

    fn draw_private(&mut self, shape: ShapePrivate) {
        match shape {
            Line { color, thickness, points } => {
                self.draw_line(color, thickness, points );
            },
            Polygon { color, points } => {
                self.draw_polygon(color, &points);
            },
            Transform { transform, shape } => {
                self.multiply(transform);
                self.draw_private(*shape);
            },
            Join { shapes } => {
                for s in shapes.into_iter() {
                    self.draw_private(s);
                }
            }
        }
    }
}