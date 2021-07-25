use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    value: String,
    children: HashMap<char, usize>,
    end: bool,
}

impl TrieNode {
    fn new(v: String, e: bool) -> Self {
        TrieNode {
            value: v,
            children: HashMap::new(),
            end: e,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    //nodes[0] is always a root
    nodes: Vec<TrieNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Trie {
            nodes: vec![TrieNode::new(String::from(""), false)],
        }
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {
        let mut acc = String::from("");
        let mut cur = 0;
        for c in word.chars() {
            if let Some(&child) = self.nodes[cur].children.get(&c) {
                acc.push(c);
                cur = child;
                if acc == word {
                    self.nodes[cur].end = true;
                };
            } else {
                acc.push(c);
                self.nodes.push(TrieNode::new(acc.clone(), acc == word));
                //println!("Current nodes: {:#?}", self.nodes);
                let child = self.nodes.len() - 1;
                self.nodes[cur].children.insert(c, child);
                cur = child;
            }
        }
    }

    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        let mut cur = 0;
        let mut res = false;
        for c in word.chars() {
            if let Some(&child) = self.nodes[cur].children.get(&c) {
                cur = child;
                let n = &self.nodes[child];
                res = n.value == word && n.end;
            }
        }
        res
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut cur = 0;
        let mut res = false;
        for c in prefix.chars() {
            if let Some(&child) = self.nodes[cur].children.get(&c) {
                cur = child;
                res = self.nodes[child].value == prefix;
            }
        }
        res
    }
}

#[test]
fn test_trie() {
    let mut tr = Trie::new();
    let words = vec!["dotty", "dot", "drone"];
    let not_there = vec!["do", "drones", "dots"];
    let prefs = vec!["do", "dott", "dron", "drone", "dot"];
    let not_prefs = vec!["r", "gr", "wh"];
    //insert the words into Trie
    words.iter().for_each(|&w| tr.insert(String::from(w)));
    //check that all the inserted words are present
    assert!(words
        .iter()
        .fold(true, |acc, &w| acc && tr.search(String::from(w))));
    //check that the words that are not there are missing
    assert!(!not_there
        .iter()
        .fold(false, |acc, &w| acc || tr.search(String::from(w))));
    //positive and negative tests for prefixes
    assert!(prefs
        .iter()
        .fold(true, |acc, &w| acc && tr.starts_with(String::from(w))));
    assert!(!not_prefs
        .iter()
        .fold(false, |acc, &w| acc || tr.starts_with(String::from(w))));
}
