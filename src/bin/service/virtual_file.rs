use std::{
    fs,
    time::{Duration, UNIX_EPOCH},
};

use fuser::{FileAttr, FileType, Filesystem};
use libc::ENOENT;

pub struct VirtualFile {
    pub source_file: String,
}
impl Filesystem for VirtualFile {
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
            let contents = fs::read(&self.source_file).unwrap();
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
