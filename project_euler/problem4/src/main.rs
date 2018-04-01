//A palindromic number reads the same both ways. The largest palindrome made from
//the product of two 2-digit numbers is 9009 = 91 × 99.
//Find the largest palindrome made from the product of two 3-digit numbers.


/// Converts a number to a vector of digits.
fn number_to_vec(mut number: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    loop {
        digits.push((number % 10) as u8);
        number = number / 10;
        if number < 10 {
            digits.push(number as u8);
            break;
        }
    }
    digits.reverse();
    digits
}

/// Converts a vector of digits to a number.
fn vec_to_number(mut digits: Vec<u8>) -> u64 {
    digits.reverse();
    let mut number = 0;
    for (i, digit) in digits.iter().enumerate() {
        number = number + *digit as u32 * 10u32.pow(i as u32);
    }
    number as u64
}

/// Checks if digit representation of a number is a palindrome.
fn is_palindrome(digits: &Vec<u8>) -> bool {
    let mut reversed = digits.clone();
    reversed.reverse();
    (reversed == *digits)
}

fn main() {
    let mut largest: u64 = 0;
    for i in 100..999 {
        for j in 100..999 {
            let digits = number_to_vec(i*j);
            if is_palindrome(&digits) {
                let number = vec_to_number(digits);
                if number > largest {
                    largest = number;
                }
            }
        }
    }
    println!("{}", largest);
}
