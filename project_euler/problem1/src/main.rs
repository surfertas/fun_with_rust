// Problem 1 from Project Euler
// https://projecteuler.net/problem=1

use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target = u64::from_str(&args[1]).unwrap();
    let mut total: u64 = 0;

    for i in 1..target {
        if i % 3 == 0 || i % 5 == 0 {
            total+=i;
        }
    }            
    println!("{:?}", total);
}
