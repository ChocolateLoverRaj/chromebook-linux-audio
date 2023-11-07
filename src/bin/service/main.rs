use chromebook_audio::SOF_BOARD_GENERATIONS;
use fuser::{spawn_mount2, BackgroundSession, FileAttr, FileType, Filesystem, MountOption};
use libc::ENOENT;
use parking::Parker;
use std::{
    fs::{self},
    process::Command,
    time::{Duration, UNIX_EPOCH},
};
use tokio::join;

use crate::{board_generations::get_board_generations, get_board_name::get_board_name};
mod board_generations;
mod get_board_name;

struct MyFS {}
impl Filesystem for MyFS {
    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        if ino == 1 {
            let contents = fs::read("/etc/os-release").unwrap();
            let end = ((offset + (size as i64)) as usize).min(contents.len());
            reply.data(&contents[offset as usize..end]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &fuser::Request, ino: u64, reply: fuser::ReplyAttr) {
        match ino {
            1 => match fs::metadata("/etc/os-release") {
                Ok(stats) => {
                    reply.attr(
                        &Duration::from_nanos(0),
                        &FileAttr {
                            ino: 1,
                            size: stats.len(),
                            blocks: 1,
                            atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                            mtime: UNIX_EPOCH,
                            ctime: UNIX_EPOCH,
                            crtime: UNIX_EPOCH,
                            kind: FileType::RegularFile,
                            perm: 0o644,
                            nlink: 1,
                            uid: 501,
                            gid: 20,
                            rdev: 0,
                            flags: 0,
                            blksize: 0,
                        },
                    )
                }
                Err(_e) => {
                    reply.error(1);
                }
            },
            _ => reply.error(ENOENT),
        }
    }
}

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
        let fs = MyFS {};
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
