#![cfg_attr(feature = "guest", no_std)]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use serde::{Serialize,  Deserialize};

pub mod predictions;
use predictions::{Scaler, LinearRegressionModel, RidgeRegressionModel};

#[derive(Serialize, Deserialize)]
pub struct ModelInput {
    pub scaler: Scaler,
    pub ridge_model: RidgeRegressionModel,
    pub x: Vec<Vec<f32>>
}

#[jolt::provable]
fn alloc(n: u32) -> u32 {
    let mut v = Vec::<u32>::new();
    for i in 0..100 {
        v.push(i);
    }

    v[n as usize]
}

#[jolt::provable(max_input_size = 10000000)]
pub fn load_model(model_input: ModelInput) -> Vec<f32> {
    let X_scaled = model_input.scaler.transform(&model_input.x);
    
    let ridge_pred = model_input.ridge_model.predict(&X_scaled);
    
    let combined_predictions = vec![ridge_pred.clone()];
    combined_predictions.into_iter().flat_map(|array| array.to_vec()).collect()
} 


