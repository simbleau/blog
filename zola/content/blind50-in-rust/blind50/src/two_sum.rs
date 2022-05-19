// O(n)
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match complements.get(&(target - num)) {
            Some(&i2) => return vec![i2, i as i32],
            None => complements.insert(*num, i as i32),
        };
    }
    vec![]
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn test_2() {
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test_3() {
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
