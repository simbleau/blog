pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut zeros = 0;
    let mut zeros_pos = 0;
    let mut product = 1;
    for (i, num) in nums.iter().enumerate() {
        if num == &0 {
            zeros += 1;
            if zeros > 1 {
                return vec![0; nums.len()];
            } else {
                zeros_pos = i;
            }
        } else {
            product *= num;
        }
    }

    if zeros == 1 {
        let mut buffer: Vec<i32> = vec![0; nums.len()];
        let item = buffer.get_mut(zeros_pos).unwrap();
        *item = product;
        buffer
    } else {
        let mut buffer: Vec<i32> = vec![];
        for num in nums.iter() {
            buffer.push(product / num);
        }
        buffer
    }
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
