pub fn convert(s: String, num_rows: i32) -> String {
    fn zz_iter(n: usize) -> impl Iterator<Item = usize> {
        match n {
            0 => panic!("Having 0 rows is not supported!"),
            1 => vec![0].into_iter().cycle(),
            n => (0..n)
                .chain((1..n - 1).rev())
                .collect::<Vec<usize>>()
                .into_iter()
                .cycle(),
        }
    }
    if s.len() < 2 {
        s
    } else {
        let n = num_rows as usize;
        let it = zz_iter(n);
        let res = it
            .zip(s.chars())
            .fold(vec![Vec::new(); n], |mut acc, (i, c)| {
                acc[i].push(c);
                acc
            });
        res.iter().flat_map(|it| it.clone()).collect()
    }
}

//with a bit more brevity and using strings instead of char vectors
pub fn convert2(s: String, num_rows: i32) -> String {
    let n = num_rows as usize;
    if s.len() <= n {
        s
    } else {
        (0..n)
            .chain((1..n - 1).rev())
            .cycle()
            .zip(s.chars())
            .fold(vec![String::new(); n], |mut acc, (i, c)| {
                acc[i].push(c);
                acc
            })
            .iter()
            .fold(String::new(), |acc, el| acc + el)
    }
}
