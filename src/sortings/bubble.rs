pub fn bubble_sort(nums: &Vec<i32>) -> Vec<i32> {
    let mut arr = nums.clone();
    let len = arr.len();

    for i in (0..len).rev() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    mod util;
    use super::*;

    #[test]
    fn check_bubble() {
        let mut nums = vec![-5, -10, 0, 7, 1, 3, -6, 2];

        let result = bubble_sort(&nums);
        nums.sort();

        assert!(util::compare_vecs(&result, &nums));
    }
}
