// 2520 is the smallest number that can be divided by each of the numbers from 1 to
// 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?

/// Checks if given number is divisible by all the numbers starting from 1 to
/// to n.
fn is_divisble(number: u64, n: usize) -> bool {
    for divisor in 1..n {
        if number % divisor as u64 != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n: usize = 20;
    for i in 1.. {
        if is_divisble(i, n) {
            println!("{:?}", i);
            break;
        }
    }
}
