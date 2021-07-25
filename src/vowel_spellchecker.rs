use std::collections::HashMap;
use std::collections::HashSet;
pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    fn hash(s: &String, vowels: Option<&HashSet<char>>) -> String {
        let sub = |c: char| {
            let mut res = c.to_ascii_lowercase();
            if let Some(vs) = vowels {
                res = if vs.contains(&res) { '_' } else { res };
            }
            res
        };
        s.chars().map(sub).collect::<String>()
    }
    let default = String::from("");
    let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
    //this set is for exact word lookup
    let exact_lookup: HashSet<&String> = wordlist.iter().collect();
    //this map is for capitalization lookup
    let cap_lookup: HashMap<String, &String> =
        wordlist.iter().fold(HashMap::new(), |mut acc, e| {
            let key = hash(e, None);
            if !acc.contains_key(&key) {
                acc.insert(key, e);
            }
            acc
        });
    //this map is for general lookup, including vowel substitution
    let subst_lookup: HashMap<String, &String> =
        wordlist.iter().fold(HashMap::new(), |mut acc, e| {
            let key = hash(e, Some(&vowels));
            if !acc.contains_key(&key) {
                acc.insert(key, e);
            }
            acc
        });
    let res: Vec<String> = queries
        .iter()
        .map(|w| {
            exact_lookup
                .get(&w)
                .or_else(|| cap_lookup.get(&hash(&w, None)))
                .or_else(|| subst_lookup.get(&hash(&w, Some(&vowels))))
                .copied()
                .unwrap_or(&default)
        })
        .cloned()
        .collect();
    res
}
