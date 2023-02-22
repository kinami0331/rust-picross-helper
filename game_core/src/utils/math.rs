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

//* 将 k 个球按顺序放进 s 个盒子中，每个盒子至少一个的所有放法 */
pub fn calc_balls_in_boxes_combination(k: u8, s: u8) -> Vec<Vec<u8>> {
    assert!(k >= s);
    let total_num = count_combinations((k - 1) as u64, (s - 1) as u64);
    let mut rst: Vec<Vec<u8>> = Vec::with_capacity(total_num as usize);
    let mut buffer: Vec<u8> = vec![0; s as usize];

    fn inner(
        current_box: u8,
        total_box_num: u8,
        remaining_ball: u8,
        total_ball: u8,
        buffer: &mut Vec<u8>,
        rst: &mut Vec<Vec<u8>>,
    ) {
        if current_box == total_box_num - 1 {
            buffer[current_box as usize] = remaining_ball;
            rst.push(buffer.clone());
            return;
        }
        // 除了当前的盒子之外， 还有 total_box_num - current_box - 1 个盒子
        // 当前盒子最多装 remaining_ball - (total_box_num - current_box - 1) 个球
        for i in 1..=(remaining_ball - (total_box_num - current_box - 1)) {
            buffer[current_box as usize] = i;
            inner(
                current_box + 1,
                total_box_num,
                remaining_ball - i,
                total_ball,
                buffer,
                rst,
            );
        }
    }

    inner(0, s, k, k, &mut buffer, &mut rst);
    println!("{} {}", total_num, rst.len());
    rst
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

    #[test]
    fn test_calc_balls_in_boxes_combination() {
        let rst = calc_balls_in_boxes_combination(9, 6);
        let total_num = count_combinations(8, 5);
        println!("{:?}", rst);
        assert_eq!(rst.len(), total_num as usize);
    }
}
