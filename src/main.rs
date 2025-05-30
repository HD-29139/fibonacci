use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{fs::File, io::Write, mem::replace, time::Instant};

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
    
    let mut line = String::new();
    println!("");
    std::io::stdin().read_line(&mut line).expect("failed to read");
    let line: usize = line.trim().parse().expect("please input a number");
    let start = Instant::now();
    let res = fibonacci(line);
    println!("Fibonacci({}) = {}", line, res);
    
    
    let value = res.to_string();
    let mut file = File::create("fibonacci.txt").expect("failed to create");
    file.write_all(value.as_bytes()).expect("failed to write");

    let time = start.elapsed();
    println!("{:?}", time);
}
