/// Color struct represents an RGB (Red, Green, Blue) trio. Each color is somewhere
/// between 0 - 1.
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}


impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    /// Generate a color with all elements equal to 0
    pub fn zeroes() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

implement_random_function!(Color, red, green, blue);
implement_common_vector_functions!(Color, f64, red, green, blue);
implement_cross_function!(Color, red, green, blue);
implement_unit_function!(Color, red, green, blue);
