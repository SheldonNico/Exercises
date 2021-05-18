fn hamming_distance_bitcount(mut value: u64) -> u64 {
    let mut count = 0;
    while value > 0 {
        if (value & 1) > 0 {
            count += 1;
        }
        value >>= 1;
    }
    count
}

pub fn hamming_distance(a: u64, b: u64) -> u64 {
    return hamming_distance_bitcount(a ^ b)
}

pub fn hamming_distance_str(a: &str, b: &str) -> u64 {
    assert_eq!(a.len(), b.len());
    let mut count = 0;
    for (a, b) in a.bytes().zip(b.bytes()) {
        if a != b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_sample() {
        assert_eq!(2, hamming_distance(11, 2));
        assert_eq!(1, hamming_distance(0, 2));
        assert_eq!(3, hamming_distance(11, 0));
        assert_eq!(1, hamming_distance_str("1101", "1111"));
        assert_eq!(0, hamming_distance_str("1111", "1111"));
        assert_eq!(1, hamming_distance_str("alpha", "alphb"));
        assert_eq!(0, hamming_distance_str("abcd", "abcd"));
        assert_eq!(4, hamming_distance_str("dcba", "abcd"));
    }
}
