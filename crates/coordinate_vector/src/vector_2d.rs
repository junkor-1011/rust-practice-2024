use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use num_traits::Float;

pub use crate::common_traits::CartesianCoordinatesVector;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vector2D<F: Float> {
    pub x: F,
    pub y: F,
}

impl<F: Float> CartesianCoordinatesVector<F> for Vector2D<F> {
    fn norm(&self) -> F {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn norm_sqr(&self) -> F {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl<F: Float> Add for Vector2D<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<F: Float> Sub for Vector2D<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<F: Float> Vector2D<F> {
    pub fn new(x: F, y: F) -> Self {
        Self { x, y }
    }
}

impl<F: Float + Display> Display for Vector2D<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_new() {
        let v = Vector2D::new(1.5f64, -3.5f64);
        assert_eq!(
            v,
            Vector2D {
                x: 1.5f64,
                y: -3.5f64
            }
        );

        let w = Vector2D::new(-100.0f32, 15.0f32);
        assert_eq!(w, Vector2D { x: -100., y: 15. });
    }

    #[test]
    fn check_display() {
        let v = Vector2D::new(-1.5, 10.3);
        assert_eq!(format!("{v}"), "(-1.5, 10.3)",);
    }

    #[test]
    fn check_add() {
        let v = Vector2D::new(-1., 2.);
        let w = Vector2D::new(1.5, -1.);

        assert_eq!(v + w, Vector2D::new(0.5, 1.),)
    }

    #[test]
    fn check_sub() {
        let v = Vector2D::new(-1., 2.);
        let w = Vector2D::new(1.5, -1.);

        assert_eq!(v - w, Vector2D::new(-2.5, 3.),);
        assert_eq!(w - v, Vector2D::new(2.5, -3.),);
    }

    #[test]
    fn check_norm() {
        let v = Vector2D::new(-1., 2.);
        assert_eq!(v.norm(), (5.).sqrt());

        let w = Vector2D::new(3., -4.);
        assert_eq!(w.norm(), 5.);
    }

    #[test]
    fn check_norm_sqr() {
        let v = Vector2D::new(-1., 2.);
        assert_eq!(v.norm_sqr(), 5.);

        let w = Vector2D::new(3., -4.);
        assert_eq!(w.norm_sqr(), 25.);

        let x = Vector2D::new(-1.5, 1.);
        assert_eq!(x.norm_sqr(), 3.25);
    }
}
