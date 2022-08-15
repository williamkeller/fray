
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





#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_new() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);
        let z = rng.gen_range(-10.0..10.0);
        let p = Vertex::new(x, y, z);

        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.z, z);
    }
}
