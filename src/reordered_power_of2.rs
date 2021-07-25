use std::collections::HashSet;
use std::iter::FromIterator;

pub fn reordered_power_of2(n: i32) -> bool {
    fn to_sorted_string(x: i32) -> String {
        let mut cs: Vec<char> = x.to_string().chars().collect();
        cs.sort();
        String::from_iter(cs)
    }
    let n_str = to_sorted_string(n);
    let p2s: HashSet<String> = (0..)
        .map(|x| to_sorted_string((2 as i32).pow(x)))
        .take_while(|x| x.len() <= n_str.len())
        .collect();
    p2s.contains(&n_str)
}
