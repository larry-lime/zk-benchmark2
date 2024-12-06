#![cfg_attr(feature = "guest", no_std)]
use serde::{Deserialize, Serialize};

extern crate alloc;
use alloc::vec::Vec;

use guest::predictions;

impl From<ScalerParams> for predictions::ScalerParams {
    fn from(params: ScalerParams) -> predictions::ScalerParams {
        predictions::ScalerParams {
            mean: params.mean,
            scale: params.scale,
        }
    }
}

impl From<LinearRegressionParams> for predictions::LinearRegressionParams {
    fn from(params: LinearRegressionParams) -> predictions::LinearRegressionParams {
        predictions::LinearRegressionParams {
            coefficients: params.coefficients,
            intercept: params.intercept,
        }
    }
}

impl From<RidgeRegressionParams> for predictions::RidgeRegressionParams {
    fn from(params: RidgeRegressionParams) -> predictions::RidgeRegressionParams {
        predictions::RidgeRegressionParams {
            coefficients: params.coefficients,
            intercept: params.intercept,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LinearRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32// intercept is a single value but using Vec for consistency
}

#[derive(Debug, Deserialize)]
pub struct RidgeRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32
}

#[derive(Debug, Deserialize)]
pub struct PolynomialRidgeRegressionParams {
    pub coefficients: Vec<f32>,
    pub intercept: f32,
}

#[derive(Debug, Deserialize)]
pub struct ScalerParams {
    pub mean: Vec<f32>,
    pub scale: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct TestData {
    CustomerID: f32,
    frequency: f32,
    monetary: f32,
    recency: f32,
    Price: f32,
    DiscountApplied: f32,
    spend_90_flag: f32,
    actual_spend_90_days: f32,
}


