pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = nums
        .iter()
        .scan(1, |acc, &el| {
            *acc = *acc * el;
            Some(*acc)
        })
        .collect::<Vec<i32>>();
    let mut right_prod = 1;
    for i in (0..nums.len()).rev() {
        let left_prod = if i > 0 { res[i - 1] } else { 1 };
        res[i] = left_prod * right_prod;
        right_prod *= nums[i];
    }
    res
}
