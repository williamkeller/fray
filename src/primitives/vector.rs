use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z, w: 0.0 }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x +
         self.y * self.y +
         self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector {
        let length = self.length();

        Vector::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Add for Vector {
     type Output = Vector;

     fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
     type Output = Vector;

     fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector {
     type Output = Vector;

     fn mul(self, rhs: f64) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_vector() -> (f64, f64, f64, Vector) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);
        let z = rng.gen_range(-10.0..10.0);
        let v = Vector::new(x, y, z);

        (x, y, z, v)
    }

    #[test]
    fn test_new() {
        let (x, y, z, v) = random_vector();

        assert_eq!(v.x, x);
        assert_eq!(v.y, y);
        assert_eq!(v.z, z);
    }

    #[test]
    fn test_addition() {
        let (x1, y1, z1, v1) = random_vector();
        let (x2, y2, z2, v2) = random_vector();
        let v3 = v1 + v2;

        assert_eq!(v3.x, x1 + x2);
        assert_eq!(v3.y, y1 + y2);
        assert_eq!(v3.z, z1 + z2);
    }

    #[test]
    fn test_subtraction() {
        let (x1, y1, z1, v1) = random_vector();
        let (x2, y2, z2, v2) = random_vector();
        let v3 = v1 - v2;

        assert_eq!(v3.x, x1 - x2);
        assert_eq!(v3.y, y1 - y2);
        assert_eq!(v3.z, z1 - z2);
    }

    #[test]
    fn test_multiply_by_scalar() {
        let (x, y, z, v1) = random_vector();

        let mut rng = rand::thread_rng();
        let scalar = rng.gen_range(0.0..100.0);
        let v2 = v1 * scalar;

        assert_eq!(v2.x, x * scalar);
        assert_eq!(v2.y, y * scalar);
        assert_eq!(v2.z, z * scalar);
    }

    #[test]
    fn test_length() {
        let (x, y, z, v) = random_vector();
        let length = (x * x + y * y + z * z).sqrt();

        assert_eq!(v.length(), length);
    }

    #[test]
    fn test_normalize() {
        let v1 = Vector::new(4.0, 0.0, 0.0);
        let v2 = v1.normalize();

        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
        assert_eq!(v2.length(), 1.0);
    }
}
