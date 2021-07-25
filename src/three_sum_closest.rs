pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut n = nums;
    n.sort();
    let last = n.len() - 1;
    let mut i = 0;
    let mut res: i32 = 0;
    let mut dist: i32 = i32::max_value();
    while i < last - 1 {
        //init two other pointers
        let (mut j, mut k) = (i + 1, last);
        while j < k {
            println!(
                "(i,j,k) is ({},{},{}), dist is {}, res is {}",
                i, j, k, dist, res
            );
            let s = n[i] + n[j] + n[k];
            let d = i32::abs(s - target);
            if dist > d {
                dist = d;
                res = s;
            }
            match s - target {
                //we are to the right of target, need to make sum smaller -> move k left
                x if x > 0 => k = k - 1,
                //we are to the left of target, need to make sum larger -> move j right
                x if x < 0 => j = j + 1,
                _ => {
                    //zero distance, we got our answer
                    res = s;
                    break;
                }
            };
        }
        i = i + 1;
    }
    res
}
