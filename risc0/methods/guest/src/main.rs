#![no_main]
#![no_std]
risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use serde::{Serialize,  Deserialize};
use ml_core::{ModelInput};


fn main() {

    // read the input
    let model_input: ModelInput = env::read();

    let X_scaled = model_input.scaler.transform(&model_input.x);
    
    let ridge_pred = model_input.linear_model.predict(&X_scaled);
    
    let combined_predictions = vec![ridge_pred.clone()];

    let output: Vec<_> = combined_predictions.into_iter().flat_map(|array| array.to_vec()).collect();
    // write public output to the journal
    env::commit(&output);
    
}
