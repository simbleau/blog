pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    assert!(nums.is_empty() == false);
    let mut max_subarray = *nums.get(0).unwrap();
    let mut current_subarray = max_subarray;
    for i in 1..nums.len() {
        if current_subarray < 0 {
            current_subarray = 0;
        }
        current_subarray += nums.get(i).unwrap();
        max_subarray = std::cmp::max(current_subarray, max_subarray);
    }

    max_subarray
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}

#[test]
fn test_2() {
    assert_eq!(max_sub_array(vec![1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
