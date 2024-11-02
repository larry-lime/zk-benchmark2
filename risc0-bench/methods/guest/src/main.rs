extern crate alloc;
use alloc::vec::Vec;
use risc0_zkvm::guest::env;

fn fib(n: u32) -> u32 {
    let mut nums = Vec::<u32>::new(); // Initialize an empty vector
    nums.push(1); // Manually push the first two Fibonacci numbers
    nums.push(1);

    // Start the loop from the third Fibonacci number
    for _ in 2..n {
        let c = (nums[nums.len() - 1] + nums[nums.len() - 2]) % 7919;
        nums.push(c);
    }
    nums[nums.len() - 1]
}

fn main() {
    // read the input
    let input: u32 = env::read();

    // do something with the input
    let output = fib(input);

    // writing to the journal makes it public
    env::commit(&output)
}
