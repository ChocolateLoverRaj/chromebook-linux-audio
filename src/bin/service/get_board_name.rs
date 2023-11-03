use std::io;

pub async fn get_board_name() -> Result<String, io::Error> {
    match async_fs::read_to_string("/sys/devices/virtual/dmi/id/product_name").await {
        Ok(s) => Ok(s.trim().to_lowercase()),
        Err(e) => Err(e),
    }
}
