use leet_code::swapping_nodes::*;
fn main() {
    let l = from_vec(vec![1, 2]);
    // let l = from_vec(vec![1, 2, 3, 4, 5, 6]);
    let res = swap_nodes(l, 1);
    println!("the list is {:?}", res);
}
