use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    // Optional optimization
    if s.len() != t.len() {
        return false;
    }

    let mut letters = HashMap::with_capacity(s.len());
    // Inserting character amounts into a HashMap
    for c in s.chars() {
        match letters.get_mut(&c) {
            Some(amt) => {
                *amt += 1;
            }
            None => {
                letters.insert(c, 1);
            }
        };
    }
    // Drain character amounts into a HashMap
    for c in t.chars() {
        match letters.get_mut(&c) {
            Some(amt) => {
                *amt -= 1;
            }
            None => {
                return false;
            }
        };
    }
    // If every character in the hashmap is not 0, there are more or less
    // characters in t than s.
    for (_, amt) in letters {
        if amt != 0 {
            return false;
        }
    }
    true
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(
        is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
}

#[test]
fn test_2() {
    assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
}
