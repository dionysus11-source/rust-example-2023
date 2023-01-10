extern crate libc;
extern crate time;

use std::time::Instant;

extern "C" {
    fn compute_pi(n: i32) -> f64;
}
fn main() {
    // Call the foreign function using FFI
    let start = Instant::now();
    let result = unsafe { compute_pi(1000000) };
    let elapsed = start.elapsed();
    println!("FFI: result = {}, elapsed = {:?}", result, elapsed);

    // Call the native Rust function
    let start = Instant::now();
    let result = native_compute_pi(1000000);
    let elapsed = start.elapsed();
    println!("Native: result = {}, elapsed = {:?}", result, elapsed);
}

fn native_compute_pi(n: i32) -> f64 {
    let mut sum = 0.0;
    for i in 0..n {
        sum += (i as f64 + 0.5).powf(-2.0);
    }
    (sum * 6.0).sqrt()
}