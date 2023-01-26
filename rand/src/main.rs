mod random;
use crate::random::{linear, xorshift};
fn main() {
    println!("Hello, world!");
    let mut seed = 1u32;
    let r1 = linear::rand(&mut seed);
    println!("{}",seed);
    let r2 = xorshift::rand(&mut seed);
    println!("{}, {}",r1, r2);
}
