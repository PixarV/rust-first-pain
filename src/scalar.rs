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
