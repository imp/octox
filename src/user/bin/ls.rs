#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use ulib::{
    env,
    fs::{self, File},
    path::Path,
    print, println, sys,
};

fn main() {
    let args = env::args();
    if args.len() < 2 {
        ls(".").unwrap();
        return;
    }
    for arg in args.skip(1) {
        ls(arg).unwrap();
    }
}

fn ls(path: &str) -> sys::Result<()> {
    if fs::metadata(path)?.is_dir() {
        fs::read_dir(path)?
            .filter_map(|entry| name_and_attr(entry).ok())
            .for_each(print_entry)
    } else {
        let attr = fs::metadata(path)?;
        print_entry((path, attr))
    }

    Ok(())
}

fn name_and_attr(entry: sys::Result<fs::DirEntry>) -> sys::Result<(String, fs::Metadata)> {
    let entry = entry?;
    let attr = entry.metadata()?;
    Ok((entry.file_name(), attr))
}

fn print_entry((name, attr): (impl AsRef<str>, fs::Metadata)) {
    println!(
        "{:14} {:6} {:3} {}",
        name.as_ref(),
        format_args!("{}", attr.file_type()),
        attr.inum(),
        attr.len(),
    );
}
