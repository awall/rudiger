pub const GREEN: Color = Color::new([0.0, 1.0, 0.0, 1.0]);
pub const RED: Color = Color::new([1.0, 0.0, 0.0, 1.0]);

#[derive(Clone, Copy)]
pub struct Color {
    components: [f32; 4],
}

impl Color {
    const fn new(components: [f32; 4]) -> Self {
        Color { components: components }
    }

    pub fn components(&self) -> [f32; 4] {
        self.components
    }
}