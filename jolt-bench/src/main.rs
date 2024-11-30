use guest::load_model;

pub fn main() {
    //  // let (prove_fib, verify_fib) = guest::build_fib();
    // let (prove_add, verify_add) = guest::build_add();
    //
    //  //let (output, proof) = prove_fib(10);
    // let (add_output, add_proof) = prove_add(1, 2);
    // let (add2_output, add2_proof) = prove_add(2, 3);
    //
    //  //let is_valid = verify_fib(proof);
    // let add_is_valid = verify_add(add_proof);
    // let try_add = verify_add(add2_proof);
    //
    // //println!("output: {}", output);
    // //println!("valid: {}", is_valid);
    //
    // println!("add output: {}", add_output);
    // println!("add valid: {}", add_is_valid);
    if let Err(e) = load_model() {
        eprintln!("Error loading model: {}", e);
    }
}
