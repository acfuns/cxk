#[allow(dead_code)]
fn mul(mut a: i32, mut b: i32, p: i32) -> i32 {
    let mut res = 0;

    while b != 0 {
        if b & 1 == 1 {
            res = (res + a) % p;
        }
        b >>= 1;
        a = (2 * a) % p;
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mul() {
        const P: i32 = 100000009;
        assert_eq!(20, mul(10, 2, P));
        assert_eq!(6, mul(2, 3, P));
        assert_eq!(0, mul(2, 0, P));
    }
}
