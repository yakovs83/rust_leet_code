pub fn my_atoi(str: String) -> i32 {
    //skipt the space characters
    let mut no_space = str.chars().skip_while(|&c| c == ' ').peekable();
    //check for sign
    let (rest, sign) = match no_space.peek() {
        Some('-') => (no_space.skip(1), -1),
        Some('+') => (no_space.skip(1), 1),
        _ => (no_space.skip(0), 1),
    };
    //take digits only, flip sign if negative
    let digits: Vec<i32> = rest
        .take_while(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32 * sign)
        .collect();
    //fold with saturating operations to hit max / min value per the problem statement
    digits
        .iter()
        .fold(0i32, |acc, &el| acc.saturating_mul(10).saturating_add(el))
}

#[test]
fn basic() {
    assert_eq!(my_atoi("42".to_owned()), 42, "Basic case - just the number");
}
#[test]
fn neg_with_space() {
    assert_eq!(
        my_atoi("    -42".to_owned()),
        -42,
        "Negative number with some preceeding spaces"
    );
}

#[test]
fn with_words_after() {
    assert_eq!(
        my_atoi("4193 with words".to_owned()),
        4193,
        "A number with some words after"
    );
}

#[test]
fn with_words_before() {
    assert_eq!(
        my_atoi("with words 987".to_owned()),
        0,
        "A number with some preceeding words"
    )
}

#[test]
fn min_max_values() {
    assert_eq!(
        my_atoi("-91283472332".to_owned()),
        i32::min_value(),
        "Testing saturating behavior near min value"
    );
    assert_eq!(
        my_atoi("91283472332".to_owned()),
        i32::max_value(),
        "Testing saturating behavior near max value"
    );
}
