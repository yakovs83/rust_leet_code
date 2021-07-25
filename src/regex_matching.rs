use std::collections::HashSet;
#[derive(Debug, Copy, Clone)]
enum RE {
    RepC(char), //zero of more of char
    RepAny,     //zero of more of any char
    C(char),    //specific char
    Any,        //any char
}
type Pos = (usize, usize);

fn process(p: String) -> Vec<RE> {
    p.chars().fold(Vec::new(), |mut acc, el| match el {
        '.' => {
            acc.push(RE::Any);
            acc
        }
        '*' => {
            match acc.pop() {
                Some(RE::C(ch)) => acc.push(RE::RepC(ch)),
                Some(RE::Any) => acc.push(RE::RepAny),
                ch => panic!("Something went really wrong, we popped {:?}", ch),
            };
            acc
        }
        ch => {
            acc.push(RE::C(ch));
            acc
        }
    })
}

// pub fn is_match(s: String, p: String) -> bool {
//     let mut lookup: HashSet<Pos> = HashSet::new();
//     let s: Vec<char> = s.chars().collect();
//     let p: Vec<RE> = process(p);
//     false
// }

pub fn is_match(s: String, p: String) -> bool {
    let mut lookup: HashSet<Pos> = HashSet::new();
    let s: Vec<char> = s.chars().collect();
    let p: Vec<RE> = process(p);
    //TODO: handle empty inputs here
    fn rec_match(s: &Vec<char>, p: &Vec<RE>, pos: Pos, l: &mut HashSet<Pos>) -> bool {
        let (i, j) = pos;
        let (s_ended, p_ended) = (i == s.len(), j == p.len());
        //if we have a lookup table hit, we can return
        if l.contains(&pos) {
            false
        } else {
            match (s_ended, p_ended) {
                //if we ran out of pattern and the string - we have a match
                (true, true) => true,
                //if we ran out of pattern but we still have characters - no match
                (false, true) => false,
                //if we ran out of characters and pattern ends in RepC / RepAny it's a match, otherwise no-match
                (true, false) => match p[j] {
                    RE::RepC(_) | RE::RepAny => rec_match(s, p, (i, j + 1), l),
                    _ => false,
                },
                _ => {
                    println!(
                        "In advance branch, pos is {:?}, values are {:?}",
                        pos,
                        (s[i], p[j])
                    );
                    let res = match (s[i], p[j]) {
                        //if there is no char to char match, it's false
                        (c, RE::C(pc)) if pc != c => false,
                        //if repeating char does not match we advance the pattern position
                        (c, RE::RepC(pc)) if pc != c => rec_match(s, p, (i, j + 1), l),
                        //if we have repeat (any) char, we can advance either - two recursion branches
                        (_, RE::RepAny) | (_, RE::RepC(_)) => {
                            rec_match(s, p, (i + 1, j), l) || rec_match(s, p, (i, j + 1), l)
                        }
                        //default branch is
                        //1) (c,RE::C(pc)) if c == pc
                        //2) (_,RE::Any)
                        _ => rec_match(s, p, (i + 1, j + 1), l),
                    };
                    if !res {
                        l.insert((i, j));
                    }
                    println!("{:?} -> {:?}\nLookup table is {:?}", pos, res, l);
                    res
                }
            }
        }
    }
    rec_match(&s, &p, (0, 0), &mut lookup)
}

#[test]
fn basic_tests() {
    let tests = vec![
        ("", "", true, "Empty string, empty pattern"),
        ("", "a*", true, "Empty string, char wildcard"),
        ("", ".*", true, "Empty string, any char wildcard"),
        ("", ".*c*", true, "Empty string, multiple any* wildcards"),
        ("a", "a", true, "Single char match"),
        ("a", ".", true, "Any char match"),
        ("a", "a*", true, "Wildcard match"),
        ("a", ".*", true, "Non-empty string any char wildcard"),
        ("abc", "abc", true, "Exact match, multiple chars"),
    ];
    tests.iter().for_each(|(s, p, r, d)| {
        assert_eq!(is_match(String::from(*s), String::from(*p)), *r, "{}", d);
    });
}
