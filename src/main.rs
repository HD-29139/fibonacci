use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{mem::replace, time::Instant};


fn fibonacci(n: usize) -> BigUint {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    for _ in 0..n {
        let temp = a + &b;
        a = replace(&mut b, temp);
    }


    a
}
fn main() {
    let n = 1; //just for tests
    let start = Instant::now();
    let res = fibonacci(n);
    let time = start.elapsed();

    println!("Fibonacci({}) = {}", n, res);
    println!("{:?}", time);
}
