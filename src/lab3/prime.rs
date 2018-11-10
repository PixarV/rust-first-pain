pub fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 { return false; }

    let mut div: u64 = 2; 
    while div.saturating_pow(2) <= n {
	    if n % div == 0 { return false; }
	    div += 1u64;
    }
    
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(17));
    }
}

