pub fn is_valid(s: String) -> bool {
    // Optional optimization
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            // Opening parentheses are free
            '{' | '[' | '(' => stack.push(c),
            // Closing parentheses require checks
            '}' => {
                if stack.last().is_some() && stack.last().unwrap() == &'{' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if stack.last().is_some() && stack.last().unwrap() == &'[' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if stack.last().is_some() && stack.last().unwrap() == &'(' {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => return false,
        };
    }

    // Stack should be empty after all closing parentheses
    stack.len() == 0
}

#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(is_valid("()".to_string()), true);
}

#[test]
fn test_2() {
    assert_eq!(is_valid("()[]{}".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(is_valid("(]".to_string()), false);
}

#[test]
fn test_4() {
    assert_eq!(is_valid("(".to_string()), false);
}

#[test]
fn test_5() {
    assert_eq!(is_valid("([)]".to_string()), false);
}
