#[cfg(test)]
use game_core;

#[test]
fn test_game_core() {
    let game = game_core::GameCore::new(15, 15);
    let game_info = game.get_info();
    dbg!(game);
    println!("{}", game_info);
    assert_eq!(game_info, "Game Info:\n    row size: 15\n    col size: 15\n"); 
}
