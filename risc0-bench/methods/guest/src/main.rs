use risc0_zkvm::guest::env;

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

fn main() {
    // read the input
    let input: u32 = env::read();

    // do something with the input
    let output = fib(input);

    // writing to the journal makes it public
    env::commit(&output)
}
