#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, Arbitrary)]
enum Action {
    CreateFile(String),
    OpenFile(String),
    Exists(String),
    RemoveFile(String),
    CopyFile(String, String),
    MoveFile(String, String),
}
use Action::*;

use vfs::impls::memory::MemoryFS;
use vfs::{FileSystem, VfsResult};

fuzz_target!(|actions: Vec<Action>| {
    let _ = fuzz(actions);
});

fn fuzz(actions: Vec<Action>) -> VfsResult<()> {
    let fs = MemoryFS::new();
    let mut files = Vec::new();

    for action in actions {
        match action {
            CreateFile(path) => {
                fs.create_file(&path)?;
                files.push(path);
            }
            OpenFile(path) => {
                let r = fs.open_file(&path);
                if files.contains(&path) {
                    assert!(r.is_ok(), "file should exist");
                }
            }
            Exists(path) => {
                let exists = fs
                    .exists(&path)
                    .expect("in mem file exists should never fail");
                if !path.is_empty() {
                    assert_eq!(files.contains(&path), exists);
                }
            }
            RemoveFile(path) => {
                let _ = fs.remove_file(&path);
                files.retain(|f| f != &path);
            }
            CopyFile(from, to) => {
                fs.copy_file(&from, &to)?;
                files.push(to);
            }
            MoveFile(from, to) => {
                fs.copy_file(&from, &to)?;
                files.retain(|f| f != &from);
                files.push(to);
            }
        }
    }

    Ok(())
}
