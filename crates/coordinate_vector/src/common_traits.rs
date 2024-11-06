use std::ops::{Add, Sub};

use num_traits::Float;

pub trait CartesianCoordinatesVector<F: Float>: Add + Sub + Sized {
    fn norm(&self) -> F;
    fn norm_sqr(&self) -> F;
    fn scale(&self, t: F) -> Self;
}
