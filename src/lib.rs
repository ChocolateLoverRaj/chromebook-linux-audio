pub const EXTENSION_NAME: &str = "chromebook-audio";

pub const SOF_BOARD_GENERATIONS: [&str; 6] = ["glk", "apl", "cml", "tgl", "jsl", "adl"];

pub fn get_extension_dir() -> String {
    format!("/var/lib/extensions/{EXTENSION_NAME}")
}
