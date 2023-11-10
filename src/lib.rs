pub const EXTENSION_PREFIX: &str = "chromebook-audio";

pub const SOF_BOARD_GENERATIONS: [&str; 6] = ["glk", "apl", "cml", "tgl", "jsl", "adl"];

pub fn get_common_ucm_extension_name() -> String {
    format!("{}-common-ucm", EXTENSION_PREFIX)
}

pub fn get_avs_extension_name() -> String {
    format!("{}-avs", EXTENSION_PREFIX)
}

pub fn get_avs_max98357a_extension_name() -> String {
    format!("{}-avs-max98357a", EXTENSION_PREFIX)
}

pub fn get_mt8138_extension_name() -> String {
    format!("{}-mt8138", EXTENSION_PREFIX)
}
