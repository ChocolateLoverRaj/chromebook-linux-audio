pub const EXTENSION_NAME: &str = "chromebook-audio";

pub fn get_extension_dir() -> String {
    format!("/var/lib/extensions/{EXTENSION_NAME}")
}
