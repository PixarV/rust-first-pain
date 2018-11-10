pub fn compare_vecs(vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    let mut it1 = vec1.iter();
    let mut it2 = vec2.iter();

    loop {
        match (it1.next(), it2.next()) {
            (Some(e1), Some(e2)) => {
                if e1 == e2 {
                    continue;
                } else {
                    return false;
                }
            }
            (None, None) => break,
            _ => return false,
        }
    }
    return true;
}
