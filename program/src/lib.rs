//! Fibonacci program, compiled to an SP1 ELF.
//! The host reads the public output and submits it to Soundness Layer.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read private input: which fibonacci number to compute.
    let n: u32 = sp1_zkvm::io::read();

    // Compute fibonacci(n) in the simplest way possible.
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }

    // Commit the result as a public value.
    sp1_zkvm::io::commit(&a);
}
