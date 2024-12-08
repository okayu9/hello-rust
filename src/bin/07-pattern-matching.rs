enum GameState{
    Playing(u32),
    Paused,
    GameOver(String),
}

fn main(){
    let state = GameState::Playing(42);
    let _paused_state = GameState::Paused; // _をつけると未使用変数として警告が出ない
    let _game_over_state = GameState::GameOver(String::from("Lost all lives"));

    match state {
        GameState::Playing(turn) => println!("Game in progress at turn: {}", turn),
        GameState::Paused => println!("Game is paused"),
        // GameState::GameOver(reason) => println!("Game over because: {}", reason), // reasonの所有権が移動するので、以降でreasonやstateを使えない
        GameState::GameOver(ref reason) => println!("Game over because: {}", reason), // 参照を使うことで所有権を移動しない
    }

    if let GameState::Playing(turn) = state {
        println!("if let: Game in progress at turn: {}", turn);
    }
}
