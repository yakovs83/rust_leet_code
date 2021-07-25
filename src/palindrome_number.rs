pub fn is_palindrome(x: i32) -> bool {
    match x {
        x if x < 0 => false,
        x if x < 10 => true,
        x => {
            let mut digits = Vec::new();
            let mut y = x;
            while y > 0 {
                digits.push(y % 10);
                y = y / 10;
            }
            digits
                .iter()
                .zip(digits.iter().rev())
                .map(|(d1, d2)| d1 == d2)
                .fold(true, |acc, el| acc && el)
        }
    }
}
