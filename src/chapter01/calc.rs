#[allow(dead_code)]
pub fn calc_exp(n: usize) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n]; 1 << n];

    for i in 0..1 << n {
        let mut ans = Vec::new();
        for j in 0..n {
            if i >> j & 1 == 1 {
                ans.push((j + 1) as i32);
            }
        }
        res.push(ans);
    }

    res
}

#[allow(dead_code)]
pub fn calc_combining(u: usize, s: usize, state: i32, n: usize, m: usize) {
    if s == m {
        let mut vec = Vec::new();
        for i in 0..n {
            if state >> i & 1 == 1 {
                vec.push(i + 1);
            }
        }
        println!("{:?}", vec);
    }

    for i in u..n {
        calc_combining(i + 1, s + 1, state | (1 << i), n, m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_exp() {
        let res = calc_exp(2);
        assert!(res.contains(&vec![]));
        assert!(res.contains(&vec![1, 2]));
        assert!(res.contains(&vec![1]));
        assert!(res.contains(&vec![2]));
    }
}
