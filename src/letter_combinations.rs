use std::collections::HashMap;
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        vec![]
    } else {
        let chars: HashMap<char, Vec<char>> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .map(|(c, s)| (c, s.chars().collect()))
        .collect();

        let count: HashMap<char, usize> = chars.iter().map(|(c, cc)| (*c, cc.len())).collect();
        let nr = digits
            .chars()
            .map(|c| count.get(&c).unwrap_or(&1))
            .product();

        let nc = digits.len();
        let mut res: Vec<Vec<char>> = vec![vec!['a'; nc]; nr];
        let mut rep_cnt = nr;
        digits.chars().enumerate().for_each(|(col, c)| {
            rep_cnt = rep_cnt / count.get(&c).unwrap_or(&1);
            let seq: Vec<char> = chars
                .get(&c)
                .unwrap_or(&vec![])
                .iter()
                .flat_map(|c| vec![*c; rep_cnt])
                .collect();
            let mut it = seq.iter().cycle().enumerate();
            //fill the result
            while let Some((row, c)) = it.next() {
                if row == nr {
                    break;
                }
                res[row][col] = *c;
            }
        });
        res.iter().map(|v| v.into_iter().collect()).collect()
    }
}
