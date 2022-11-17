use std::ops::{Add, Sub};
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        let xo = cos * self.x - sin * self.y;
        let yo = sin * self.x + cos * self.y;
        Self::new(xo, yo)
    }

    pub fn get_middle(self, other: Self) -> Self {
        Self::new((self.x + other.x) / 2., (self.y + other.y) / 2.)
    }

    pub fn rotate_around_origin(self, origin: Self, angle: f64) -> Self {
        let (sin, cos) = (angle.sin(), angle.cos());
        let diff = self - origin;
        let xo = origin.x + cos * diff.x - sin * diff.y;
        let yo = origin.y + sin * diff.x + cos * diff.y;
        Self::new(xo, yo)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, PI};

    fn roughly_equal(a: f64, b: f64) -> bool {
        let eps = f64::EPSILON * 10.; // multiple calculations higher tolerance
        (a - b).abs() < eps
    }

    #[test]
    /// mostly *pointless* checks but ultimately good for my sanity
    fn point_math_sanity_check() {
        let p = Point::new(2., 4.);
        let r = p.rotate(FRAC_PI_2);
        assert!(roughly_equal(r.x, -4.));
        assert!(roughly_equal(r.y, 2.));

        let mid = p.get_middle(Point::new(3., -5.));
        assert!(roughly_equal(mid.x, 2.5));
        assert!(roughly_equal(mid.y, -0.5));

        let orbiter = p.rotate_around_origin(Point::new(4., 8.), FRAC_PI_2);
        assert!(roughly_equal(orbiter.x, 8.));
        assert!(roughly_equal(orbiter.y, 6.));

        let orbiter = p.rotate_around_origin(Point::new(4., 8.), 0.);
        assert!(roughly_equal(orbiter.x, 2.));
        assert!(roughly_equal(orbiter.y, 4.));

        let orbiter = p.rotate_around_origin(Point::new(4., 8.), PI);
        assert!(roughly_equal(orbiter.x, 6.));
        assert!(roughly_equal(orbiter.y, 12.));
    }
}
