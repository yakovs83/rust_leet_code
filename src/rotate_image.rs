pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    fn nex_pos((i, j): (usize, usize), n: usize) -> (usize, usize) {
        (j, n - i - 1)
    }
    let n = matrix.len();
    for i in 0..=(n / 2) {
        for j in i..(n - i - 1) {
            let (mut ci, mut cj) = (i, j);
            let mut cv = matrix[i][j];
            for _ in 0..4 {
                let (ni, nj) = nex_pos((ci, cj), n);
                let nv = matrix[ni][nj];
                matrix[ni][nj] = cv;
                cv = nv;
                ci = ni;
                cj = nj;
            }
        }
    }
}
