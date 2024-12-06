use guest::{ModelInput, predictions::Scaler, predictions::LinearRegressionModel, predictions::RidgeRegressionModel, 
    predictions::ScalerParams, predictions::LinearRegressionParams, predictions::RidgeRegressionParams};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Read;
use ndarray::Array2;
use csv::ReaderBuilder;
use std::fmt;
use serde_json::from_str;
use std::env;
use std::time::Instant;

mod models;

use models::{TestData, ScalerParams as HostScalerParams, LinearRegressionParams as HostLinearRegressionParams, RidgeRegressionParams as HostRidgeRegressionParams};

#[derive(Debug, Serialize, Deserialize)]
pub enum MyError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
    InvalidInput(String)
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MyError::IoError(msg) => write!(f, "IO error: {}", msg),
            MyError::InvalidInput(msg) => write!(f, "Invlaid Input: {}", msg)
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

pub fn read_test_dataset(lines: usize) -> Result<(Vec<Vec<f32>>, Vec<f32>), MyError> {
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    let mut file =
        File::open("./guest/model/Test_Dataset.csv").expect("Test_Dataset.csv not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read Test_Dataset.csv");

    // Parse the CSV into a vector of TestData
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(contents.as_bytes());

    let mut test_features = Vec::new();
    let mut actual_amounts = Vec::new();
    let mut cur: usize = 0;
    for result in rdr.deserialize() {
        if cur == lines {
            break;
        }
        let record: TestData = result.expect("Failed to deserialize record.");
        test_features.push(vec![
            record.CustomerID,
            record.frequency,
            record.monetary,
            record.recency,
            record.Price,
            record.DiscountApplied,
            record.spend_90_flag 
        ]);
        actual_amounts.push(record.actual_spend_90_days);
        cur += 1;
    }

    Ok((test_features, actual_amounts))
}


pub fn read_models(
    scaler_path: &str,
    linear_model_path: &str,
    ridge_model_path: &str,
    poly_ridge_model_path: &str,
) -> Result<(Scaler, LinearRegressionModel, RidgeRegressionModel), MyError> {
    match env::current_dir() {
            Ok(path) => println!("Current working directory in read_model: {}", path.display()),
            Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    // Read and deserialize Scaler

    let mut scaler_file = File::open(scaler_path)?;
    let mut scaler_contents = String::new();
    scaler_file.read_to_string(&mut scaler_contents)?;
    let scaler_params: HostScalerParams = from_str(&scaler_contents)?;
    let guest_scaler_params: ScalerParams = scaler_params.into();
    let scaler = Scaler::new(guest_scaler_params);

    // Read and deserialize LinearRegressionModel
    let mut linear_file = File::open(linear_model_path)?;
    let mut linear_contents = String::new();
    linear_file.read_to_string(&mut linear_contents)?;
    let linear_params: HostLinearRegressionParams =  from_str(&linear_contents)?;
    let guest_linear_params: LinearRegressionParams = linear_params.into();
    let linear_model = LinearRegressionModel::new(guest_linear_params);

    // Read and deserialize RidgeRegressionModel
    let mut ridge_file = File::open(ridge_model_path)?;
    let mut ridge_contents = String::new();
    ridge_file.read_to_string(&mut ridge_contents)?;
    let ridge_params: HostRidgeRegressionParams = from_str(&ridge_contents)?;
    let guest_ridge_params: RidgeRegressionParams = ridge_params.into();
    let ridge_model = RidgeRegressionModel::new(guest_ridge_params);

    // Read and deserialize PolynomialRidgeRegressionModel
    // let mut poly_ridge_file = File::open(poly_ridge_model_path)?;
    // let mut poly_ridge_contents = String::new();
    // poly_ridge_file.read_to_string(&mut poly_ridge_contents)?;
    // let poly_ridge_params: PolynomialRidgeRegressionParams =  from_str(&poly_ridge_contents)?;
    // let poly_ridge_model = PolynomialRidgeRegressionModel::new(poly_ridge_params);
        
    Ok((scaler, linear_model, ridge_model))
}


pub fn main() {
    let scaler_path = "./guest/model/scaler_params.json";
    let linear_model_path = "./guest/model/linear_regression_params.json";
    let ridge_model_path = "./guest/model/ridge_regression_params.json";
    let poly_ridge_model_path = "./guest/model/polynomial_ridge_regression_params.json";

    // Read the test dataset
    let Ok((x, actual_amounts)) = read_test_dataset(10) else { todo!() };
    // println!("test: {:?}", test_features);
    // Read the models
    let Ok((scaler, linear_model, ridge_model)) =
    read_models(scaler_path, linear_model_path, ridge_model_path, poly_ridge_model_path) else {
       
        eprintln!("{:?}", read_models(scaler_path, linear_model_path, ridge_model_path, poly_ridge_model_path));

        return; // Exit or take alternative action
    };

    //let Ok(x) = flatten(test_features) else { todo!() };

     let model_input = ModelInput {
        // test_features,
        // actual_amounts,
        scaler,
        ridge_model,
        x 
    };

    let (prove_model, verify_model) = guest::build_load_model();
    let mut start = Instant::now();
    let (model_output, model_proof) = prove_model(model_input);
    let proof_duration = start.elapsed();
    start = Instant::now();
    let model_is_valid = verify_model(model_proof);
    let verify_duration = start.elapsed();
    println!("Proof time taken: {:?}", proof_duration);
    println!("Verify time taken: {:?}", verify_duration);
 
    println!("output: {:?}", model_output);
    println!("output: {:?}", model_is_valid);
}
