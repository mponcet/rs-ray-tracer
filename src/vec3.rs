use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    fn len_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    #[allow(dead_code)]
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[allow(dead_code)]
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    #[allow(dead_code)]
    pub fn unit_vector(self) -> Self {
        self / self.len()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
}

pub type Color = Vec3;

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (255.99 * self.0) as u8,
            (255.99 * self.1) as u8,
            (255.99 * self.2) as u8,
        )
    }
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 1.0, 1.0) + Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(3.0, 3.0, 3.0)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(5.0, 1.0, 1.0) - Vec3::new(5.0, 2.0, 2.0),
            Vec3::new(0.0, -1.0, -1.0)
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * Vec3::new(3.0, 2.0, 1.0),
            Vec3::new(3.0, 4.0, 3.0)
        );
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(5.0, 4.0, 3.0) * 2.0, Vec3::new(10.0, 8.0, 6.0));
    }

    #[test]
    fn test_div_f64() {
        assert_eq!(Vec3::new(9.0, 6.0, 3.0) / 3.0, Vec3::new(3.0, 2.0, 1.0));
    }
}
