// #![cfg_attr(feature = "guest", no_std)]
//#![no_main]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Read;
use ndarray::Array2;
use csv::ReaderBuilder;
use serde_json::from_str;
use std::fmt;

mod models;
mod predictions;

use models::{TestData, LinearRegressionParams, RidgeRegressionParams, PolynomialRidgeRegressionParams, ScalerParams};
use predictions::{Scaler, LinearRegressionModel, RidgeRegressionModel, PolynomialRidgeRegressionModel};

#[derive(Debug, Serialize, Deserialize)]
pub enum MyError {
    FileNotFound(String),
    ParseError(String),
    IoError(String)
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MyError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::ParseError(error.to_string())
    }
}

#[jolt::provable(stack_size = 10000, memory_size = 10000000)]
pub fn load_model() -> Result<(), MyError>{
    println!("Starting model loading...");
    let mut file = File::open("./model/Test_Dataset.csv").expect("Test_Dataset.csv not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read Test_Dataset.csv");

    // Parse the CSV into a vector of TestData
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(contents.as_bytes());

    let mut test_features = Vec::new();
    let mut actual_amounts = Vec::new();

    for result in rdr.deserialize() {
        let record: TestData = result.expect("Failed to deserialize record.");
        // Collect features into a vector (excluding 'amount')
        test_features.push(vec![
            record.quantity,
            record.price,
            record.discount_applied,
            record.IsAL,
            record.IsAK,
            record.IsAZ,
            record.IsAR,
            record.IsCA,
            record.IsCO,
            record.IsCT,
            record.IsDE,
            record.IsFL,
            record.IsGA,
            record.IsHI,
            record.IsID,
            record.IsIL,
            record.IsIN,
            record.IsIA,
            record.IsKS,
            record.IsKY,
            record.IsLA,
            record.IsME,
            record.IsMD,
            record.IsMA,
            record.IsMI,
            record.IsMN,
            record.IsMS,
            record.IsMO,
            record.IsMT,
            record.IsNE,
            record.IsNV,
            record.IsNH,
            record.IsNJ,
            record.IsNM,
            record.IsNY,
            record.IsNC,
            record.IsND,
            record.IsOH,
            record.IsOK,
            record.IsOR,
            record.IsPA,
            record.IsRI,
            record.IsSC,
            record.IsSD,
            record.IsTN,
            record.IsTX,
            record.IsUT,
            record.IsVT,
            record.IsVA,
            record.IsWA,
            record.IsWV,
            record.IsWI,
            record.IsWY,
            record.IsCash,
            record.IsPayPal,
            record.IsDebitCard,
            record.IsCreditCard,
            record.IsBooks,
            record.IsHomeDecor,
            record.IsElectronics,
            record.IsClothing,
        ]);
        actual_amounts.push(record.amount);
    }
    println!("CSV data parsed");

    // Convert Vec<Vec<f64>> to Array2<f64>
    let num_samples = test_features.len();
    let num_features = test_features[0].len();
    let flat_features: Vec<f64> = test_features.into_iter().flatten().collect();
    let x: Array2<f64> = Array2::from_shape_vec((num_samples, num_features), flat_features)
        .expect("Failed to create Array2 from shape");
    println!("Data converted to Array2<64>");

    // Load scaler parameters
    let mut file = File::open("./model/scaler_params.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let scaler_params: ScalerParams = from_str(&contents)?;
    let scaler = Scaler::new(scaler_params);

    let X_scaled = scaler.transform(&x);

    // Load Linear Regression parameters
    let mut file = File::open("./model/linear_regression_params.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let linear_params: LinearRegressionParams = from_str(&contents)?;
    let linear_model = LinearRegressionModel::new(linear_params);

    // Load Ridge Regression parameters
    let mut file = File::open("./model/ridge_regression_params.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ridge_params: RidgeRegressionParams = from_str(&contents)?;
    let ridge_model = RidgeRegressionModel::new(ridge_params);
    
    // Load Polynomial Ridge Regression parameters
    let mut file = File::open("./model/polynomial_ridge_regression_params.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let poly_ridge_params: PolynomialRidgeRegressionParams = from_str(&contents)?;
    let poly_ridge_model = PolynomialRidgeRegressionModel::new(poly_ridge_params);
    println!("All models loaded");

    // Make predictions
    let linear_pred = linear_model.predict(&X_scaled);
    let ridge_pred = ridge_model.predict(&X_scaled);
    let poly_ridge_pred = poly_ridge_model.predict(&X_scaled);
    println!("Predictions completed.");

    // Compute evaluation metrics
    let mae_linear = compute_mae(&linear_pred, &actual_amounts);
    let mse_linear = compute_mse(&linear_pred, &actual_amounts);
    let rmse_linear = mse_linear.sqrt();
    let r2_linear = compute_r2(&linear_pred, &actual_amounts);

    let mae_ridge = compute_mae(&ridge_pred, &actual_amounts);
    let mse_ridge = compute_mse(&ridge_pred, &actual_amounts);
    let rmse_ridge = mse_ridge.sqrt();
    let r2_ridge = compute_r2(&ridge_pred, &actual_amounts);

    let mae_poly_ridge = compute_mae(&poly_ridge_pred, &actual_amounts);
    let mse_poly_ridge = compute_mse(&poly_ridge_pred, &actual_amounts);
    let rmse_poly_ridge = mse_poly_ridge.sqrt();
    let r2_poly_ridge = compute_r2(&poly_ridge_pred, &actual_amounts);

    // Print the evaluation metrics
    println!("----- Linear Regression Metrics -----");
    println!("MAE: {}", mae_linear);
    println!("MSE: {}", mse_linear);
    println!("RMSE: {}", rmse_linear);
    println!("R²: {}", r2_linear);

    println!("----- Ridge Regression Metrics -----");
    println!("MAE: {}", mae_ridge);
    println!("MSE: {}", mse_ridge);
    println!("RMSE: {}", rmse_ridge);
    println!("R²: {}", r2_ridge);

    println!("----- Polynomial Ridge Regression Metrics -----");
    println!("MAE: {}", mae_poly_ridge);
    println!("MSE: {}", mse_poly_ridge);
    println!("RMSE: {}", rmse_poly_ridge);
    println!("R²: {}", r2_poly_ridge);

    println!("Linear Regression Prediction: {}", linear_pred[0]);
    println!("Ridge Regression Prediction: {}", ridge_pred[0]);
    println!("Polynomial Ridge Regression Prediction: {}", poly_ridge_pred[0]);

    Ok(())
}

fn compute_mae(predictions: &ndarray::Array1<f64>, actuals: &Vec<f64>) -> f64 {
    let errors = predictions.iter().zip(actuals.iter()).map(|(p, a)| (p - a).abs());
    errors.sum::<f64>() / predictions.len() as f64
}

fn compute_mse(predictions: &ndarray::Array1<f64>, actuals: &Vec<f64>) -> f64 {
    let errors = predictions.iter().zip(actuals.iter()).map(|(p, a)| (p - a).powi(2));
    errors.sum::<f64>() / predictions.len() as f64
}

fn compute_r2(predictions: &ndarray::Array1<f64>, actuals: &Vec<f64>) -> f64 {
    let actual_mean = actuals.iter().sum::<f64>() / actuals.len() as f64;
    let ss_tot: f64 = actuals.iter().map(|a| (*a - actual_mean).powi(2)).sum();
    let ss_res: f64 = predictions.iter().zip(actuals.iter()).map(|(p, a)| (*a - p).powi(2)).sum();
    1.0 - (ss_res / ss_tot)
}

#[jolt::provable]
fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    b
}

#[jolt::provable]
fn add(a: u128, b: u128) -> u128 {
    let sum: u128;

    sum = a + b;

    sum
  }
