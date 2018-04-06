extern crate libeuler;

use libeuler::prime;

fn main() {
    let mut counter = 0;
    let mut answer = 0;

    for num in (2..).filter(|x| prime::is_prime_factor(*x)) {
        counter += 1;
        if counter == 10001 {
            answer = num;
            break;
        }
    }            
    println!("The 10001st prime is: {}", answer);
}
