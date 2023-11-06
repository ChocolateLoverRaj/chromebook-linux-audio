use fuser::{spawn_mount2, FileAttr, FileType, Filesystem, MountOption};
use libc::ENOENT;
use std::{
    fs::{self, File},
    process::Command,
    time::{Duration, UNIX_EPOCH},
};

struct MyFS {
    file_contents: String,
}
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
            let config = &self.file_contents[..];
            let end = ((offset + (size as i64)) as usize).min(config.len());
            reply.data(&config.as_bytes()[offset as usize..end]);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &fuser::Request, ino: u64, reply: fuser::ReplyAttr) {
        match ino {
            1 => reply.attr(&Duration::from_nanos(0), &self.get_file_attr()),
            _ => reply.error(ENOENT),
        }
    }

    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        _parent: u64,
        _name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        reply.entry(&Duration::from_nanos(0), &self.get_file_attr(), 0);
    }
}

impl MyFS {
    fn get_file_attr(&self) -> FileAttr {
        FileAttr {
            ino: 1,
            size: self.file_contents.len() as u64,
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
            blksize: 512,
        }
    }
}

fn main() {
    let path = "/var/lib/extensions/chromebook-linux-audio/usr/lib/extension-release.d/extension-release.chromebook-linux-audio";
    let file_contents = fs::read_to_string("/var/lib/extensions/chromebook-linux-audio/usr/lib/extension-release.d/.extension-release.chromebook-linux-audio").unwrap();
    let fs = MyFS { file_contents };
    File::create(path).unwrap();
    let handle = spawn_mount2(
        fs,
        path,
        &[
            MountOption::RO,
            MountOption::AllowOther,
            MountOption::AutoUnmount,
        ],
    )
    .unwrap();
    Command::new("systemctl")
        .args(&["restart", "systemd-sysext"])
        .output()
        .unwrap();
    handle.join();
}
