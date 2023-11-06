use std::collections::HashMap;

use async_fs::read_to_string;

pub async fn get_board_generations() -> HashMap<String, String> {
    let board_generations = read_to_string(format!("/usr/share/chromebook-audio/boards.json"))
        .await
        .unwrap();
    let board_generations: HashMap<String, String> =
        serde_json::from_str(&board_generations).unwrap();
    board_generations
}
