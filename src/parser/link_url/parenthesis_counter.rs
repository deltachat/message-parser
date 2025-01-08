use nom::Slice;

macro_rules! adjust_balance {
    ($a: expr, $b: expr, $c: expr, $d: expr) => {
        // for opening ones
        {
            $a = $a.saturating_add(1);
            if $d.slice($c..).find($b).is_none() {
                return Some($c);
            }
        }
    };
    ($a: expr, $b: expr) => {
        // for closing ones
        {
            if $a == 0 {
                return Some($b);
            } else {
                $a = $a.saturating_sub(1);
            }
        }
    };
}

/// finds unbalanced closing parenthesesis and returns distance to it.
/// unbalanced means it was closed but not opened before in the given string
pub(super) fn count_chars_in_complete_parenthesis(input: &str) -> Option<usize> {
    let mut parenthes = 0usize; // ()
    let mut curly_bracket = 0usize; // {}
    let mut bracket = 0usize; // []
    let mut angle = 0usize; // <>

    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => {
                adjust_balance!(parenthes, ')', i, input);
            }
            '{' => {
                adjust_balance!(curly_bracket, '}', i, input);
            }
            '[' => {
                adjust_balance!(bracket, ']', i, input);
            }
            '<' => {
                adjust_balance!(angle, '>', i, input);
            }
            ')' => {
                adjust_balance!(parenthes, i);
            }
            ']' => {
                adjust_balance!(bracket, i);
            }
            '}' => {
                adjust_balance!(curly_bracket, i);
            }
            '>' => {
                adjust_balance!(angle, i);
            }
            _ => continue,
        }
    }
    None
}

#[test]
fn test_count_parenthesis() {
    assert_eq!(count_chars_in_complete_parenthesis("{}"), None);
    assert_eq!(count_chars_in_complete_parenthesis("{} test"), None);
    assert_eq!(count_chars_in_complete_parenthesis("(test) test"), None);
    assert_eq!(count_chars_in_complete_parenthesis("(test)) test"), Some(6));
}

#[test]
fn test_count_different_types_invalid() {
    assert_eq!(count_chars_in_complete_parenthesis("(({(})))"), None);
}

#[test]
fn test_count_different_types_invalid2() {
    assert_eq!(count_chars_in_complete_parenthesis("}(({(})))"), Some(0));
}
