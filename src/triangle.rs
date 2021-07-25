use std::iter::*;
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    fn merge(a: &Vec<i32>, b: &Vec<i32>) -> Option<Vec<i32>> {
        let fst = a.first()?;
        let lst = a.last()?;
        let padded: Vec<&i32> = once(fst).chain(a.iter()).chain(once(lst)).collect();
        let next: Vec<i32> = padded
            .windows(2)
            .map(|w| *(w[0].min(w[1])))
            .zip(b.iter())
            .map(|(x, y)| x + y)
            .collect();
        Some(next)
    }
    let mut tail = triangle.iter();
    let head = tail.next().cloned();
    let res = tail.fold(head, |acc, el| acc.and_then(|a| merge(&a, el)));
    res.and_then(|r| r.iter().min().cloned())
        .expect("Should not be None")
}
