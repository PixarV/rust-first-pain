pub fn compute_scalar(arr1: &[i32], arr2: &[i32]) -> i32 {
    let mut sum = 0;
    let mut it1 = arr1.iter();   
    let mut it2 = arr2.iter();   
   
    loop {
        match  (it1.next(), it2.next())  {
            (Some(a1), Some(a2)) => sum += a1*a2,
            _ => break,
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_for_equal_arrays() {
        let arr1 = &[1, 2, 3];
        let arr2 = &[1, 2, 3];
        
        let res = compute_scalar(arr1, arr2);
        assert_eq!(res, 14);
    }

    #[test]
    fn scalar_for_unequal_arrays() {
        let arr1 = &[1, 2, 3];
        let arr2 = &[1, 2, 3, 4, 5];
        
        let res = compute_scalar(arr1, arr2);
        assert_eq!(res, 14);
    }
}
