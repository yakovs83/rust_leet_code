pub fn reverse(x: i32) -> i32 {
    let sign = if x < 0 { -1 } else { 1 };
    let mut x = x.abs();
    let mut digits = Vec::new();
    while x > 0 {
        digits.push(x % 10);
        x = x / 10;
    }
    println!("Digits are {:?}", digits);
    match digits.iter().fold(Some(0i32), |acc, el| match acc {
        Some(acc) => acc.checked_mul(10).and_then(|z| z.checked_add(*el)),
        None => None,
    }) {
        Some(z) => z * sign,
        None => 0,
    }
}
