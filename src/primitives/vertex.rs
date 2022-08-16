use std::ops::{Add, Sub};

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Vertex {
        Vertex { x: x, y: y, z: z, w: 0.0 }
    }
}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Vertex {
        Vertex::new(self.x + rhs.x,
                    self.y + rhs.y,
                    self.z + rhs.z)
    }
}

impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Vertex) -> Vertex {
        Vertex::new(self.x - rhs.x,
                    self.y - rhs.y,
                    self.z - rhs.z)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_vertex() -> (f64, f64, f64, Vertex) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);
        let z = rng.gen_range(-10.0..10.0);
        let v = Vertex::new(x, y, z);

        (x, y, z, v)
    }

    #[test]
    fn test_new() {
        let (x, y, z, v) = random_vertex();

        assert_eq!(v.x, x);
        assert_eq!(v.y, y);
        assert_eq!(v.z, z);
    }


    #[test]
    fn test_add() {
        let (x1, y1, z1, v1) = random_vertex();
        let (x2, y2, z2, v2) = random_vertex();
        let v3 = v1 + v2;

        assert_eq!(v3.x, x1 + x2);
        assert_eq!(v3.y, y1 + y2);
        assert_eq!(v3.z, z1 + z2);
    }


    #[test]
    fn test_sub() {
        let (x1, y1, z1, v1) = random_vertex();
        let (x2, y2, z2, v2) = random_vertex();
        let v3 = v1 - v2;

        assert_eq!(v3.x, x1 - x2);
        assert_eq!(v3.y, y1 - y2);
        assert_eq!(v3.z, z1 - z2);
    }
}
