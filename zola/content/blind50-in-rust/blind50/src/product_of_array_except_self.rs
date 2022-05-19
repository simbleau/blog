pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix_sum: Vec<i32> = Vec::new();
    for (i, num) in nums.enumerate() {
        // TODO
    }

    let product_array = prefix_sum;
    // TODO

    product_array
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}

#[test]
fn test_2() {
    assert_eq!(
        product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
