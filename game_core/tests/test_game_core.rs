#[cfg(test)]
use game_core;

#[test]
fn test_game_core_new() {
    // 使用的数据
    // {
    //     "row_size": 5,
    //     "col_size": 5,
    //     "row_setting": [[3], [1, 1], [3], [1], [2]],
    //     "col_setting": [[0], [3], [1, 3], [3, 1], [0]]
    // }
    let game = game_core::GameCore::new(
        5,
        5,
        vec![vec![3], vec![1, 1], vec![3], vec![1], vec![2]],
        vec![vec![0], vec![3], vec![1, 3], vec![3, 1], vec![0]],
    );
    let game_info_str = format!("{}", game.get_info());
    assert_eq!(
        game_info_str,
        r#"Game Info:
    row size: 5
    col size: 5
    row_constraint: [[3], [1, 1], [3], [1], [2]]
    col_constraint: [[0], [3], [1, 3], [3, 1], [0]]
"#
    );
    println!("{}", game_info_str);
    println!("{:?}", game);
}
