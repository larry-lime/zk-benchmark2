//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use ml_lib::ModelInput;
use serde::{Deserialize, Serialize};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let model_input = sp1_zkvm::io::read::<ModelInput>();

    let X_scaled = model_input.scaler.transform(&model_input.x);

    let ridge_pred = model_input.ridge_model.predict(&X_scaled);

    let combined_predictions = vec![ridge_pred.clone()];

    let output: Vec<f32> = combined_predictions.into_iter().flat_map(|array| array.to_vec()).collect();
 
    sp1_zkvm::io::commit(&output);
}
