// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

/// Checks if number is a prime factor.
fn is_prime_factor(x: u64) -> bool {
    for x_ in 2..x {
        if (x_ != x) && (x % x_ == 0) {
            return false;
        }
    }
    true
}

fn main() {
    let mut target: u64 = 600851475143;
    let mut largest_pf: u64 = 0;
    for pf in (2..).filter(|x| is_prime_factor(*x)) {
        loop {
            if target == 1 {
                largest_pf = pf;
                break;
            } else if target % pf == 0 {
                target = target / pf;
            } else {
                break;
            }
        }
        if largest_pf != 0 {
            break;
        }
    }
    println!("{:?}", largest_pf);
}
