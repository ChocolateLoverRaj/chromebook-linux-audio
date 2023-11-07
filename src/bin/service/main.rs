use chromebook_audio::SOF_BOARD_GENERATIONS;
use fuser::{spawn_mount2, BackgroundSession, MountOption};
use parking::Parker;
use std::process::Command;
use tokio::join;

use crate::{board_generations::get_board_generations, get_board_name::get_board_name};
mod board_generations;
mod get_board_name;
mod virtual_file;
use virtual_file::VirtualFile;

#[tokio::main]
async fn main() {
    let (board_name, board_generations) = join!(get_board_name(), get_board_generations());
    let board_name = board_name.unwrap();
    // let board_name = String::from("jinlon");
    let board_generation = match board_generations.get(&board_name) {
        Some(v) => Some(v as &str),
        None => None,
    };

    fn enable_overlay(overlay: String) -> BackgroundSession {
        println!("Enabling overlay: {overlay}");
        let path = &format!(
            "/etc/extensions/{overlay}/usr/lib/extension-release.d/extension-release.{overlay}"
        )[..];
        let fs = VirtualFile {
            source_file: String::from("/etc/os-release"),
        };
        return spawn_mount2(
            fs,
            path,
            &[
                MountOption::RO,
                MountOption::AllowOther,
                MountOption::AutoUnmount,
            ],
        )
        .unwrap();
    }

    match board_generation {
        Some(board_generation) => match SOF_BOARD_GENERATIONS.contains(&board_generation) {
            true => {
                let common_fuse = enable_overlay(String::from("chromebook-sof-common"));
                let board_fuse = enable_overlay(format!("chromebook-sof-{board_generation}"));
                let modprobe_fuse = spawn_mount2(
                    VirtualFile {
                        source_file: String::from("/usr/share/chromebook-audio/sof/snd-sof.conf"),
                    },
                    "/etc/modprobe.d/snd-sof.conf",
                    &[
                        MountOption::RO,
                        MountOption::AllowOther,
                        MountOption::AutoUnmount,
                    ],
                );
                Command::new("systemctl")
                    .args(&["restart", "systemd-sysext"])
                    .output()
                    .unwrap();
                let parker = Parker::new();
                let unparker = parker.unparker();
                ctrlc::set_handler(move || {
                    unparker.unpark();
                })
                .unwrap();
                parker.park();
                drop(common_fuse);
                drop(board_fuse);
                drop(modprobe_fuse);
            }
            false => {
                println!("Chromebook doesn't use SOF. Non-SOF is not implemented yet")
            }
        },
        None => {
            println!("Not a chromebook. Not overlaying Chromebook audio.")
        }
    }
}
