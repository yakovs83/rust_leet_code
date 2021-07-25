use std::collections::HashMap;
use std::convert::TryInto;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        0
    } else {
        let mut m: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut longest = 1;
        let mut start = 0;
        let mut end = 1;
        let str_len = chars.len();
        m.insert(chars[0], start);
        while end < str_len {
            let opt_rep_pos = m.get(&chars[end]).cloned();
            let len = match (opt_rep_pos, end) {
                //last character & a repeated one
                //compute the length, exit while loop
                (Some(_), l) if l == str_len - 1 => {
                    end += 1;
                    m.len()
                }
                //repeated character - compute the length
                //update pointers and char map
                (Some(rep_pos), _) => {
                    for x in start..=rep_pos {
                        m.remove(&chars[x]);
                    }
                    m.insert(chars[end], end);
                    start = rep_pos + 1;
                    end += 1;
                    m.len()
                }
                //not a repeated character - compute the length
                //keep going
                (None, _) => {
                    m.insert(chars[end], end);
                    end += 1;
                    m.len()
                }
            };
            longest = if len > longest { len } else { longest };
        }
        longest.try_into().unwrap()
    }
}
