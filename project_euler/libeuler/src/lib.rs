

pub mod prime {

    /// Checks if number is a prime factor.
    pub fn is_prime_factor(number: u64) -> bool {
        for x in 2..number {
            if (x != number) && (number % x == 0) {
                return false;
            }
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use prime::is_prime_factor;

    #[test]
    fn test_prime_factor() {
        assert!(is_prime_factor(3));
        assert!(is_prime_factor(19));
        assert_eq!(is_prime_factor(9), false);
    }
}
