use std::collections::HashMap;

use async_fs::read_to_string;
use chromebook_audio::get_extension_dir;

pub async fn get_board_generations() -> HashMap<String, String> {
    let extension_dir = get_extension_dir();
    let board_generations = read_to_string(format!(
        "{extension_dir}/usr/lib/chromebook-audio/conf/board-generations.json"
    ))
    .await
    .unwrap();
    let board_generations: HashMap<String, String> =
        serde_json::from_str(&board_generations).unwrap();
    board_generations
}
