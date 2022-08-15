use std::ops::{Add, Sub};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }
}

impl Add for Color {
     type Output = Color;

     fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r,
                   self.g + rhs.g,
                   self.b + rhs.b)
    }
}

impl Sub for Color {
     type Output = Color;

     fn sub(self, rhs: Color) -> Color {
        Color::new(self.r - rhs.r,
                   self.g - rhs.g,
                   self.b - rhs.b)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_color() -> (f64, f64, f64, Color) {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0.0..1.0);
        let g = rng.gen_range(0.0..1.0);
        let b = rng.gen_range(0.0..1.0);
        let c = Color::new(r, g, b);

        (r, g, b, c)
    }

    #[test]
    fn test_new() {
        let (r, g, b, c) = random_color();

        assert_eq!(c.r, r);
        assert_eq!(c.g, g);
        assert_eq!(c.b, b);
    }

    #[test]
    fn test_addition() {
        let (r1, g1, b1, c1) = random_color();
        let (r2, g2, b2, c2) = random_color();
        let c3 = c1 + c2;

        assert_eq!(c3.r, r1 + r2);
        assert_eq!(c3.g, g1 + g2);
        assert_eq!(c3.b, b1 + b2);
    }

    #[test]
    fn test_subtraction() {
        let (r1, g1, b1, c1) = random_color();
        let (r2, g2, b2, c2) = random_color();
        let c3 = c1 - c2;

        assert_eq!(c3.r, r1 - r2);
        assert_eq!(c3.g, g1 - g2);
        assert_eq!(c3.b, b1 - b2);
    }
}
