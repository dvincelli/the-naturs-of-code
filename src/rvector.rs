#[derive(Debug, Copy, Clone)]
pub struct RVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        RVector { x, y, z }
    }

    pub fn new2d(x: f32, y: f32) -> Self {
        RVector { x, y, z: 0.0 }
    }

    pub fn set2d(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn lerp(&mut self, v: RVector) {
        let t = 0.5;
        let u = 1.0 - t;
        self.x = self.x * u + v.x * t;
        self.y = self.y * u + v.y * t;
        self.z = self.z * u + v.z * t;
    }

    pub fn add(&mut self, v: &RVector) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    pub fn sub(&mut self, v: &RVector) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }

    pub fn div(&mut self, d: f32) {
        self.x /= d;
        self.y /= d;
        self.z /= d;
    }

    pub fn mult(&mut self, m: f32) {
        self.x *= m;
        self.y *= m;
        self.z *= m;
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let m = self.mag();
        if m != 0.0 && m != 1.0 {
            self.div(m);
        }
    }

    pub fn random2d() -> Self {
        RVector {
            x: rand::random(),
            y: rand::random(),
            z: 0.0,
        }
    }

    pub fn random3d() -> Self {
        RVector {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = RVector::new(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_new2d() {
        let v = RVector::new2d(1.0, 2.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_set() {
        let mut v = RVector::new(0.0, 0.0, 0.0);

        v.set(2.0, 1.5, 0.5);

        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 1.5);
        assert_eq!(v.z, 0.5);
    }

    #[test]
    fn test_lerp() {
        let mut v = RVector::new(1.0, 1.0, 1.0);
        let v2 = RVector::new(2.0, 2.0, 2.0);

        v.lerp(v2);

        assert_eq!(v.x, 1.5);
        assert_eq!(v.y, 1.5);
        assert_eq!(v.z, 1.5);
    }

    #[test]
    fn test_mag() {
        let v = RVector::new(1.0, 1.0, 1.0);
        assert_eq!(v.mag(), 1.7320508);
    }

    #[test]
    fn test_normalize() {
        let mut v = RVector::new(0.5, 0.2, 0.1);

        v.normalize();

        assert_eq!(v.x, 0.912871);
        assert_eq!(v.y, 0.3651484);
        assert_eq!(v.z, 0.1825742);
    }
}
