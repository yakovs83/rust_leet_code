use std::collections::HashSet;
use std::iter::FromIterator;
pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    if rooms.len() == 1 {
        true
    } else {
        let mut res: HashSet<i32> = HashSet::from_iter(rooms[0].iter().cloned());
        res.insert(0);
        let mut diff: HashSet<i32> = res.clone();
        while diff.len() > 0 {
            let new_keys: HashSet<i32> = diff
                .iter()
                .map(|k| HashSet::from_iter(rooms[*k as usize].clone()))
                .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
            diff = new_keys.difference(&res).cloned().collect();
            res = res.union(&diff).cloned().collect();
            println!(
                "diff is {:?}, res is {:?}, new_keys is {:?}",
                diff, res, new_keys
            );
        }
        res.len() == rooms.len()
    }
}
