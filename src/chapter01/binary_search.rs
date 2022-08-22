#[allow(dead_code)]
fn binary_search(vec: Vec<i32>, x: i32) -> usize {
    let (mut l, mut r) = (-1, vec.len() as i32);

    while l + 1 != r {
        let mid = (l + r) / 2;

        if vec[mid as usize] < x {
            l = mid;
        } else {
            r = mid;
        }
    }

    r as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let vec = vec![1, 2, 2, 3, 3, 3, 5];
        let eg = vec.partition_point(|&x| x < 3);
        assert_eq!(eg, binary_search(vec, 3));
    }
}
