use async_fs::{copy, create_dir_all};
use async_process::Command;
use chromebook_audio::{get_extension_dir, EXTENSION_NAME};

#[tokio::main]
async fn main() {
    let extension_dir = get_extension_dir();
    let extension_release_d_dir = format!("{extension_dir}/usr/lib/extension-release.d");
    let extension_release_file =
        format!("{extension_release_d_dir}/extension-release.{EXTENSION_NAME}");
    println!("Creating systemd-sysext extension");
    create_dir_all(extension_release_d_dir).await.unwrap();
    copy("/etc/os-release", extension_release_file)
        .await
        .unwrap();
    println!("Copying UCM conf");
    // TODO: Async cp_r
    let dest = &format!("{extension_dir}/usr/lib/chromebook-audio/chromebook-ucm-conf")[..];
    create_dir_all(dest).await.unwrap();
    cp_r::CopyOptions::new()
        .copy_tree("chromebook-ucm-conf", dest)
        .unwrap();

    println!("Copying conf");
    // TODO: Async cp_r
    let dest = &format!("{extension_dir}/usr/lib/chromebook-audio/conf")[..];
    create_dir_all(dest).await.unwrap();
    cp_r::CopyOptions::new().copy_tree("conf", dest).unwrap();

    println!("Enabling systemd-sysext");
    Command::new("systemctl")
        .args(&["enable", "--now", "systemd-sysext"])
        .output()
        .await
        .unwrap();
}
