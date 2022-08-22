#[allow(dead_code)]
fn hamilton(w: Vec<Vec<i32>>) -> i32 {
    let n = w.len();

    let mut f = vec![vec![0x3f3f3f3f; n]; 1 << n];

    f[1][0] = 0;

    for i in 1..1 << n {
        for j in 0..n {
            if i >> j & 1 == 1 {
                for k in 0..n {
                    if (i - (1 << j)) >> k & 1 == 1 {
                        f[i][j] = f[i][j].min(f[i - (1 << j)][k] + w[k][j] as usize);
                    }
                }
            }
        }
    }

    f[(1 << n) - 1][n - 1] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamilton() {
        let w = vec![
            vec![0, 2, 4, 5, 1],
            vec![2, 0, 6, 5, 3],
            vec![4, 6, 0, 8, 3],
            vec![5, 5, 8, 0, 5],
            vec![1, 3, 3, 5, 0],
        ];
        assert_eq!(18, hamilton(w));
    }
}
