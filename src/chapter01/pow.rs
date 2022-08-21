#[allow(dead_code)]
fn pow(mut a: i32, mut b: i32, p: i32) -> i32 {
    let mut res = 1 % p;

    while b != 0 {
        if b & 1 == 1 {
            res = (res * a) % p;
        }
        b >>= 1;
        a = (a * a) % p;
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pow() {
        const P: i32 = 100000009;
        assert_eq!(100, pow(10, 2, P));
        assert_eq!(8, pow(2, 3, P));
        assert_eq!(1, pow(2, 0, P));
    }
}
