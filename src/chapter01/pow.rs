#[allow(dead_code)]
fn power(mut a: i32, mut b: i32, p: i32) -> i32 {
    let mut res = 1 % p;

    while b != 0 {
        if b & 1 == 1 {
            res = res * a;
        }
        b >>= 1;
        a = a * a;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        const P: i32 = 100000009;
        assert_eq!(100, power(10, 2, P));
        assert_eq!(8, power(2, 3, P));
        assert_eq!(1, power(2, 0, P));
    }
}
