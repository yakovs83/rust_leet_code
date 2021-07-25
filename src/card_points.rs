pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let size = card_points.len();
    let n = k as usize;
    if size == n {
        card_points.iter().sum()
    } else {
        let first_k = &card_points[..n];
        let last_k = &card_points[size - n..];
        let initial_sum = first_k.iter().sum();
        let max_delta = first_k
            .iter()
            .rev()
            .zip(last_k.iter().rev())
            .map(|(f, l)| l - f)
            .scan(0i32, |acc, x| {
                *acc = *acc + x;
                Some(*acc)
            })
            .max()
            .expect("Iterator should not be empty");
        if max_delta > 0 {
            initial_sum + max_delta
        } else {
            initial_sum
        }
    }
}
