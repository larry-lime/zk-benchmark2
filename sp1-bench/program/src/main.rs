#![no_main]
// #![no_std]
sp1_zkvm::entrypoint!(main);
extern crate alloc;
use alloc::vec::Vec;

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

pub fn main() {
    let output = fib(1000);
    println!("output: {}", output);
}
