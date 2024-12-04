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
pub struct TestData {
    pub quantity: f32,
    pub price: f32,
    pub discount_applied: f32,
    pub IsAL: f32,
    pub IsAK: f32,
    pub IsAZ: f32,
    pub IsAR: f32,
    pub IsCA: f32,
    pub IsCO: f32,
    pub IsCT: f32,
    pub IsDE: f32,
    pub IsFL: f32,
    pub IsGA: f32,
    pub IsHI: f32,
    pub IsID: f32,
    pub IsIL: f32,
    pub IsIN: f32,
    pub IsIA: f32,
    pub IsKS: f32,
    pub IsKY: f32,
    pub IsLA: f32,
    pub IsME: f32,
    pub IsMD: f32,
    pub IsMA: f32,
    pub IsMI: f32,
    pub IsMN: f32,
    pub IsMS: f32,
    pub IsMO: f32,
    pub IsMT: f32,
    pub IsNE: f32,
    pub IsNV: f32,
    pub IsNH: f32,
    pub IsNJ: f32,
    pub IsNM: f32,
    pub IsNY: f32,
    pub IsNC: f32,
    pub IsND: f32,
    pub IsOH: f32,
    pub IsOK: f32,
    pub IsOR: f32,
    pub IsPA: f32,
    pub IsRI: f32,
    pub IsSC: f32,
    pub IsSD: f32,
    pub IsTN: f32,
    pub IsTX: f32,
    pub IsUT: f32,
    pub IsVT: f32,
    pub IsVA: f32,
    pub IsWA: f32,
    pub IsWV: f32,
    pub IsWI: f32,
    pub IsWY: f32,
    pub IsCash: f32,
    pub IsPayPal: f32,
    pub IsDebitCard: f32,
    pub IsCreditCard: f32,
    pub IsBooks: f32,
    pub IsHomeDecor: f32,
    pub IsElectronics: f32,
    pub IsClothing: f32,
    pub amount: f32, // Target variable
}


