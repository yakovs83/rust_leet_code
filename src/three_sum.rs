pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        vec![]
    } else {
        let mut n = nums;
        n.sort();
        let last = n.len() - 1;
        let mut i = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        while n[i] <= 0 && i < last - 1 {
            //init two other pointers
            let (mut j, mut k) = (i + 1, last);
            while j < k {
                match n[i] + n[j] + n[k] {
                    x if x > 0 => k = k - 1, //move right boundary
                    x if x < 0 => j = j + 1, //move left boundary
                    _ => {
                        //we have a triple, push it to the result vector and skip repeating elements
                        res.push(vec![n[i], n[j], n[k]]);
                        while j < k && n[j] == n[j + 1] {
                            j = j + 1;
                        }
                        j = j + 1;
                        while k > j && n[k] == n[k - 1] {
                            k = k - 1;
                        }
                        k = k - 1;
                    }
                };
            }
            //move i pointer
            //skip multiple occurences
            while n[i + 1] == n[i] && i < last - 1 {
                i = i + 1;
            }
            i = i + 1;
        }
        res
    }
}

#[test]
fn basic_test() {
    use std::collections::HashSet;
    let input = vec![-1, 0, 1, 2, -1, -4];
    let expected: HashSet<Vec<i32>> = (vec![vec![-1, 0, 1], vec![-1, -1, 2]])
        .into_iter()
        .collect();
    let result: HashSet<Vec<i32>> = three_sum(input.clone()).into_iter().collect();
    assert_eq!(
        result, expected,
        "Checking that {:?} input produces {:?} output",
        input, result
    );
}
