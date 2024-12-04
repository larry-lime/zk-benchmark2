// src/predictions.rs
extern crate alloc;
use alloc::vec::Vec;

use serde::{Serialize, Deserialize};

pub struct LinearRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32// intercept is a single value but using Vec for consistency
}

pub struct RidgeRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32
}

pub struct PolynomialRidgeRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32,
}

pub struct ScalerParams {
    pub mean: Vec<f32>,
    pub scale: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scaler {
    mean: Vec<f32>,
    scale: Vec<f32>,
}

impl Scaler {
    pub fn new(params: ScalerParams) -> Self {
        Scaler {
            mean: params.mean,
            scale: params.scale,
        }
    }

    pub fn transform(&self, input: &[Vec<f32>]) -> Vec<Vec<f32>> {
        input
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&self.mean)
                    .zip(&self.scale)
                    .map(|((value, mean), scale)| (value - mean) / scale)
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearRegressionModel {
    coefficients: Vec<f32>,
    intercept: f32,
}

impl LinearRegressionModel {
    pub fn new(params: LinearRegressionParams) -> Self {
        LinearRegressionModel {
            coefficients: params.coefficients,
            intercept: params.intercept,
        }
    }

    pub fn predict(&self, x: &[Vec<f32>]) -> Vec<f32> {
        x.iter()
            .map(|row| {
                row.iter()
                    .zip(&self.coefficients)
                    .map(|(value, coef)| value * coef)
                    .sum::<f32>()
                    + self.intercept
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RidgeRegressionModel {
    coefficients: Vec<f32>,
    intercept: f32,
}

impl RidgeRegressionModel {
    pub fn new(params: RidgeRegressionParams) -> Self {
        RidgeRegressionModel {
            coefficients: params.coefficients,
            intercept: params.intercept,
        }
    }

    pub fn predict(&self, x: &[Vec<f32>]) -> Vec<f32> {
        x.iter()
            .map(|row| {
                row.iter()
                    .zip(&self.coefficients)
                    .map(|(value, coef)| value * coef)
                    .sum::<f32>()
                    + self.intercept
            })
            .collect()
    }
}

