// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{ML_GUEST_ELF, ML_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;

use ml_core::{
    LinearRegressionModel, LinearRegressionParams, ModelInput, RidgeRegressionModel,
    RidgeRegressionParams, Scaler, ScalerParams,
};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum MyError {
    FileNotFound(String),
    ParseError(String),
    IoError(String),
    InvalidInput(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            MyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MyError::IoError(msg) => write!(f, "IO error: {}", msg),
            MyError::InvalidInput(msg) => write!(f, "Invlaid Input: {}", msg),
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

pub fn read_test_dataset() -> Result<(Vec<Vec<f32>>, Vec<f32>), MyError> {
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    let mut file =
        File::open("./host/model/Test_Dataset.csv").expect("Test_Dataset.csv not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read Test_Dataset.csv");

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

    Ok((test_features, actual_amounts))
}

pub fn read_models(
    scaler_path: &str,
    linear_model_path: &str,
    ridge_model_path: &str,
    poly_ridge_model_path: &str,
) -> Result<(Scaler, LinearRegressionModel, RidgeRegressionModel), MyError> {
    match env::current_dir() {
        Ok(path) => println!(
            "Current working directory in read_model: {}",
            path.display()
        ),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    // Read and deserialize Scaler

    let mut scaler_file = File::open(scaler_path)?;
    let mut scaler_contents = String::new();
    scaler_file.read_to_string(&mut scaler_contents)?;
    let scalar_params: ScalerParams = from_str(&scaler_contents)?;
    let scaler = Scaler::new(scalar_params);

    // Read and deserialize LinearRegressionModel
    let mut linear_file = File::open(linear_model_path)?;
    let mut linear_contents = String::new();
    linear_file.read_to_string(&mut linear_contents)?;
    let linear_params: LinearRegressionParams = from_str(&linear_contents)?;
    let linear_model = LinearRegressionModel::new(linear_params);

    // Read and deserialize RidgeRegressionModel
    let mut ridge_file = File::open(ridge_model_path)?;
    let mut ridge_contents = String::new();
    ridge_file.read_to_string(&mut ridge_contents)?;
    let ridge_params: RidgeRegressionParams = from_str(&ridge_contents)?;
    let ridge_model = RidgeRegressionModel::new(ridge_params);

    // Read and deserialize PolynomialRidgeRegressionModel
    // let mut poly_ridge_file = File::open(poly_ridge_model_path)?;
    // let mut poly_ridge_contents = String::new();
    // poly_ridge_file.read_to_string(&mut poly_ridge_contents)?;
    // let poly_ridge_params: PolynomialRidgeRegressionParams =  from_str(&poly_ridge_contents)?;
    // let poly_ridge_model = PolynomialRidgeRegressionModel::new(poly_ridge_params);

    Ok((scaler, linear_model, ridge_model))
}

fn main() {
    let scaler_path = "./host/model/scaler_params.json";
    let linear_model_path = "./host/model/linear_regression_params.json";
    let ridge_model_path = "./host/model/ridge_regression_params.json";
    let poly_ridge_model_path = "./host/model/polynomial_ridge_regression_params.json";

    // Read the test dataset
    let Ok((x, actual_amounts)) = read_test_dataset() else {
        todo!()
    };
    // println!("test: {:?}", test_features);
    // Read the models
    let Ok((scaler, linear_model, ridge_model)) = read_models(
        scaler_path,
        linear_model_path,
        ridge_model_path,
        poly_ridge_model_path,
    ) else {
        eprintln!(
            "{:?}",
            read_models(
                scaler_path,
                linear_model_path,
                ridge_model_path,
                poly_ridge_model_path
            )
        );

        return; // Exit or take alternative action
    };

    let model_input = ModelInput {
        // test_features,
        // actual_amounts,
        scaler,
        linear_model,
        x,
    };

    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().
    println!("STARTING ENV");
    // For example:
    let env = ExecutorEnv::builder()
        .write(&model_input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();
    println!("STARTING PROOF");
    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, ML_GUEST_ELF).unwrap();

    // extract the receipt.
    println!("Proof completed");
    let receipt = prove_info.receipt;

    // TODO: Implement code for retrieving receipt journal here.

    println!("Verifying receipt");

    // For example:
    let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    println!("Verifying proof");
    receipt.verify(ML_GUEST_ID).unwrap();
}
