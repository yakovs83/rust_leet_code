use std::collections::HashSet;
type Range = (usize, usize);
type NPS = HashSet<Range>;
enum Either {
    Pali(Range),
    NPali(Vec<Range>),
}

/*
This implementation is based on "shrinking" potential palindromes,
starting from the largest palindrome candidate (first and last
characters are the same). We memoize the (start, end) indices for
sustrings that are not palindromes to check against them later.
*/
pub fn longest_palindrome_v1(s: String) -> String {
    if s.is_empty() {
        s
    } else {
        let chars: Vec<char> = s.chars().collect();
        let mut not_pali: NPS = HashSet::new();
        fn check_pali(v: &Vec<char>, r: Range, nps: &NPS) -> Either {
            let mut np = Vec::new();
            let (mut i, mut j) = r;
            while i < j && v[i] == v[j] && !nps.contains(&(i, j)) {
                np.push((i, j));
                i += 1;
                j -= 1;
            }
            match j as i32 - i as i32 {
                -1 | 0 => Either::Pali(r),
                _ => Either::NPali(np),
            }
        }
        let (left, right) = (0, chars.len() - 1);
        let mut lp: Range = (0, 0);
        for i in left..right {
            for j in i + 1..=right {
                println!("({},{})", i, j);
                if chars[i] == chars[j] {
                    match check_pali(&chars, (i, j), &not_pali) {
                        Either::Pali((a, b)) => {
                            if (a as i32 - b as i32).abs() > (lp.0 as i32 - lp.1 as i32).abs() {
                                lp = (a, b);
                                println!("Longest palindrome is {:?}", lp);
                            }
                        }
                        Either::NPali(vnp) => vnp.iter().for_each(|x| {
                            not_pali.insert(*x);
                        }),
                    }
                }
            }
            if lp.1 - lp.0 > right - i {
                break;
            }
        }
        println!("Assemling result");
        chars[lp.0..=lp.1].iter().collect()
    }
}

use std::cmp;
pub fn longest_palindrome_v2(s: String) -> String {
    /*
    assumptions:
    1) cs will be at least of length 2
    2) x will never equal right
    */
    fn largest_pali(cs: &Vec<char>, x: usize, left: usize, right: usize) -> (usize, usize) {
        let max_dist = (
            cmp::min(x - left, right - x),
            cmp::min(x - left, right - x - 1),
        );
        println!(
            "x is {}, left is {}, right is {}, max_dist is {:?}",
            x, left, right, max_dist
        );
        let mut d = 1;
        while d <= max_dist.0 && cs[x - d] == cs[x + d] {
            d += 1;
        }
        let odd = (x + 1 - d, x + d - 1);
        d = 1;
        let even = if cs[x] == cs[x + 1] {
            while d <= max_dist.1 && cs[x - d] == cs[x + d + 1] {
                d += 1;
            }
            (x + 1 - d, x + d)
        } else {
            (x, x)
        };
        println!("odd is {:?}, even is {:?}", odd, even);
        if even.1 - even.0 > odd.1 - odd.0 {
            println!("Returning even - {:?}", even);
            even
        } else {
            println!("Returning odd - {:?}", odd);
            odd
        }
    }
    let chars: Vec<char> = s.chars().collect();
    match chars.len() {
        0 | 1 => s,
        _ => {
            let (left, right) = (0, chars.len() - 1);
            let res = (left..right)
                .map(|i| largest_pali(&chars, i, left, right))
                .fold(
                    (0, 0),
                    |acc, (a, b)| if acc.1 - acc.0 >= b - a { acc } else { (a, b) },
                );
            chars[res.0..=res.1].iter().collect()
        }
    }
}
