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

    let game_str = format!("{:?}", game);
    assert_eq!(
        game_str,
        r#"Game Debug Info:
    row size: 5
    col size: 5
    row_constraint: [[3], [1, 1], [3], [1], [2]]
    col_constraint: [[0], [3], [1, 3], [3, 1], [0]]
    row_lines:
        0. - - - - -
        1. - - - - -
        2. - - - - -
        3. - - - - -
        4. - - - - -
    col_lines:
        0. - - - - -
        1. - - - - -
        2. - - - - -
        3. - - - - -
        4. - - - - -
"#
    );
    println!("{}", game_str);
}

#[test]
fn test_game_core_from_json_file() {
    use std::fs;
    use std::path;

    let mut json_path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    json_path.push("../demo_game/s1.normal.p006.json");

    let json_str = fs::read_to_string(json_path).expect("打开文件失败");

    let game = game_core::GameCore::from_json(&json_str);
    let game_str = format!("{:?}", game);

    println!("{}", game_str);
}
