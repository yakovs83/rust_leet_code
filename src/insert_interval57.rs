use std::i32;

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    fn before(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[1] < b[0]
    }
    fn after(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        a[0] > b[1]
    }
    fn overlaps(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        !before(a, b) && !after(a, b)
    }

    if intervals.is_empty() {
        return vec![new_interval];
    };
    let mut res: Vec<Vec<i32>> = Vec::new();
    //now we need to consider the case where new interval does have an overlap
    let mut i = 0;
    //while new interval is after the interval in the list, we add those to the restult
    while i < intervals.len() && after(&new_interval, &intervals[i]) {
        res.push(intervals[i].clone());
        i += 1;
    }
    //handle any overlaps (if it does not overlap, we'll just insert the new interval as is)
    let (mut left, mut right) = (new_interval[0], new_interval[1]);
    while i < intervals.len() && overlaps(&new_interval, &intervals[i]) {
        (left, right) = (left.min(intervals[i][0]), right.max(intervals[i][1]));
        i += 1;
    }
    res.push(vec![left, right]);
    while i < intervals.len() {
        res.push(intervals[i].clone());
        i += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::insert_interval57::insert;
    #[test]
    fn empty_interval_list() {
        let res = insert(vec![], vec![-2, -1]);
        assert_eq!(res, vec![vec![-2, -1]]);
    }

    #[test]
    fn new_interval_to_the_left() {
        let res = insert(vec![vec![0, 0]], vec![-2, -1]);
        assert_eq!(res, vec![vec![-2, -1], vec![0, 0]]);
    }

    #[test]
    fn new_interval_to_the_right() {
        let res = insert(vec![vec![0, 0]], vec![1, 2]);
        assert_eq!(res, vec![vec![0, 0], vec![1, 2]]);
    }

    #[test]
    fn overlap() {
        let res = insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        );
        assert_eq!(res, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn no_overlap() {
        let res = insert(vec![vec![3, 5], vec![12, 15]], vec![6, 6]);
        assert_eq!(res, vec![vec![3, 5], vec![6, 6], vec![12, 15]]);
    }
}
