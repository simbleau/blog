// O(n)

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut buffer = String::new();
    for c in s.chars() {
        // If string contains the character, move sliding window
        if buffer.contains(c) {
            // Keep moving window until character is found
            let mut chars = buffer.chars();
            while let Some(ci) = chars.next() {
                if ci == c {
                    break;
                }
            }
            buffer = String::from(chars.as_str());
        }
        // Push character
        buffer.push(c);
        // Update max length
        if buffer.len() > max_length {
            max_length = buffer.len();
        }
    }
    max_length as i32
}

#[cfg(test)]
#[test]
fn test_1() {
    let input = "bbbbb".to_string();
    assert_eq!(length_of_longest_substring(input), 1);
}
#[test]
fn test_2() {
    let input = "pwwkew".to_string();
    assert_eq!(length_of_longest_substring(input), 3);
}
#[test]
fn test_3() {
    let input = "abcabcbb".to_string();
    assert_eq!(length_of_longest_substring(input), 3);
}
