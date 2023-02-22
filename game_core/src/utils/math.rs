pub fn count_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

pub fn gen_bits_by_num(n: u8) -> u64 {
    if n == 0 {
        0
    } else {
        (1 << n) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_combinations() {
        let rst = count_combinations(5, 2);
        assert_eq!(rst, 10);
    }

    #[test]
    fn test_gen_bits_by_num() {
        let rst = gen_bits_by_num(9);
        println!("{:#b}", rst);
        assert_eq!(rst, 0b1_1111_1111);
    }
}
