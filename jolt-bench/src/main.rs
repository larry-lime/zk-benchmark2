pub fn main() {
    let (prove_fib, _verify_fib) = guest::build_fib();

    let (output, _proof) = prove_fib(10);
    // let is_valid = verify_fib(proof);
    println!("output: {}", output);
    // println!("valid: {}", is_valid);
}
