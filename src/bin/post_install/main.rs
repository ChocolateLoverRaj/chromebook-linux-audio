use chromebook_audio::SOF_BOARD_GENERATIONS;

#[tokio::main]
async fn main() {
    async fn copy_os_release(extension_name: &str) {
        async_fs::copy("/etc/os-release", format!("/etc/extensions/{extension_name}/usr/lib/extension-release.d/.extension-release.{extension_name}"))
            .await
            .unwrap();
    }
    copy_os_release("chromebook-sof-common").await;
    for board_generation in SOF_BOARD_GENERATIONS {
        copy_os_release(&format!("chromebook-sof-{board_generation}")[..]).await;
    }
}
