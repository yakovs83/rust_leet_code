use leet_code::insert_interval57::insert;

fn main() {
    let input: Vec<Vec<i32>> = [[0, 0]].iter().map(|inner| inner.into()).collect();
    let res = insert(input, vec![1, 5]);
    println!("{:?}", res);
}
