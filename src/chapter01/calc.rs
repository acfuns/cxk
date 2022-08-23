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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_exp() {
        let res = calc_exp(2);
        println!("{:?}", res);
        assert!(res.contains(&vec![]));
        assert!(res.contains(&vec![1, 2]));
        assert!(res.contains(&vec![1]));
        assert!(res.contains(&vec![2]));
    }
}
