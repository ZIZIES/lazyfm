use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub is_exec: bool,
    pub size: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: Option<SystemTime>,
}

impl Entry {
    pub fn extension(&self) -> Option<String> {
        if self.is_dir {
            return None;
        }
        self.path
            .extension()
            .map(|e| e.to_string_lossy().to_ascii_lowercase())
    }

    pub fn kind(&self) -> &'static str {
        if self.is_symlink {
            "symlink"
        } else if self.is_dir {
            "directory"
        } else if self.is_exec {
            "executable"
        } else {
            "file"
        }
    }
}

fn parent_entry(cwd: &Path) -> Option<Entry> {
    let parent = cwd.parent()?;
    let meta = fs::symlink_metadata(parent).ok();
    Some(Entry {
        name: "..".to_string(),
        path: parent.to_path_buf(),
        is_dir: true,
        is_symlink: false,
        is_exec: false,
        size: meta.as_ref().map(|m| m.len()).unwrap_or(0),
        mode: meta.as_ref().map(|m| m.mode()).unwrap_or(0o755),
        uid: meta.as_ref().map(|m| m.uid()).unwrap_or(0),
        gid: meta.as_ref().map(|m| m.gid()).unwrap_or(0),
        mtime: meta.as_ref().and_then(|m| m.modified().ok()),
    })
}

pub fn read_dir(path: &Path, show_hidden: bool) -> io::Result<Vec<Entry>> {
    let mut entries: Vec<Entry> = Vec::new();

    if let Some(parent) = parent_entry(path) {
        entries.push(parent);
    }

    for dirent in fs::read_dir(path)? {
        let dirent = match dirent {
            Ok(d) => d,
            Err(_) => continue,
        };
        let name = dirent.file_name().to_string_lossy().to_string();
        if !show_hidden && name.starts_with('.') {
            continue;
        }
        let entry_path = dirent.path();
        let meta = match fs::symlink_metadata(&entry_path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let is_symlink = meta.file_type().is_symlink();
        let is_dir = fs::metadata(&entry_path)
            .map(|m| m.is_dir())
            .unwrap_or(false);
        entries.push(Entry {
            name,
            path: entry_path,
            is_dir,
            is_symlink,
            is_exec: meta.mode() & 0o111 != 0,
            size: meta.len(),
            mode: meta.mode(),
            uid: meta.uid(),
            gid: meta.gid(),
            mtime: meta.modified().ok(),
        });
    }

    entries[parent_entry_count(path)..].sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| {
            a.name
                .to_ascii_lowercase()
                .cmp(&b.name.to_ascii_lowercase())
        })
    });

    Ok(entries)
}

fn parent_entry_count(path: &Path) -> usize {
    if path.parent().is_some() { 1 } else { 0 }
}
