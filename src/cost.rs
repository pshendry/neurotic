//! Cost functions

use nalgebra::{DefaultAllocator, Dim, U1, VectorN};
use nalgebra::allocator::Allocator;

/// Cost function
pub trait CostFunction<Y>
where
    Y: Dim,
    DefaultAllocator: Allocator<f64, Y, U1>,
{
    /// Evaluates the cost function
    fn eval(input: &VectorN<f64, Y>, target: &VectorN<f64, Y>) -> f64;

    /// Evaluates the gradient of the cost function
    fn eval_grad(input: &VectorN<f64, Y>, target: &VectorN<f64, Y>) -> VectorN<f64, Y>;
}

/// Quadratic cost function
#[derive(Clone, Copy, Debug)]
pub struct MeanSquared;

impl<Y> CostFunction<Y> for MeanSquared
where
    Y: Dim,
    DefaultAllocator: Allocator<f64, Y, U1>,
{
    fn eval(input: &VectorN<f64, Y>, target: &VectorN<f64, Y>) -> f64 {
        let mut diff = input - target;
        diff.apply(|e| e.powf(2.));
        let sum: f64 = diff.iter().sum();
        sum / 2.0
    }

    fn eval_grad(input: &VectorN<f64, Y>, target: &VectorN<f64, Y>) -> VectorN<f64, Y> {
        input - target
    }
}
