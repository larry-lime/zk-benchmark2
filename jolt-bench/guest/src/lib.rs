use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::path::Path;

#![cfg_attr(feature = "guest", no_std)]
#![no_main]


fn executePython<P: AsRef<Path>>(python_script_path: P) -> PyResult<String> {
    Python::with_gil(|py| {
        // Read the Python script content
        let script_content = std::fs::read_to_string(&python_script_path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!(
                "Failed to read Python script: {}",
                e
            )))?;
        
        // Import the Python script as a module
        let script = PyModule::from_code(
            py,
            &script_content,
            python_script_path.as_ref().to_str().unwrap_or("python.py"),
            "python",
        )?;
        
        // Call the `train_and_save_model` function without arguments
        let result: String = script.call1("python", ())?.extract()?;
        
        Ok(result)
    })
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
    let mut sum: u128;

    sum = a + b;

    sum
  }
