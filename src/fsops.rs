//! Filesystem operations the file menu needs: paste targets, recursive copy,
//! move (with a cross-device fallback), and remove.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A destination path in `dir` based on `name` that does not exist yet.
/// Collisions become "name (2).ext", "name (3).ext", ...
pub fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !first.exists() {
        return first;
    }
    let path = Path::new(name);
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| name.to_string());
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    for n in 2..10_000 {
        let candidate = format!("{stem} ({n}){ext}");
        let candidate = dir.join(candidate);
        if !candidate.exists() {
            return candidate;
        }
    }
    first
}

/// Copy a file, symlink, or directory tree.
pub fn copy_path(src: &Path, dst: &Path) -> io::Result<()> {
    let meta = fs::symlink_metadata(src)?;
    if meta.is_dir() {
        fs::create_dir(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            copy_path(&entry.path(), &dst.join(entry.file_name()))?;
        }
        Ok(())
    } else if meta.file_type().is_symlink() {
        std::os::unix::fs::symlink(fs::read_link(src)?, dst)
    } else {
        fs::copy(src, dst).map(|_| ())
    }
}

/// Rename, falling back to copy + remove when the move crosses filesystems.
pub fn move_path(src: &Path, dst: &Path) -> io::Result<()> {
    match fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(_) => {
            copy_path(src, dst)?;
            remove_path(src)
        }
    }
}

/// Remove a file, symlink, or directory tree.
pub fn remove_path(src: &Path) -> io::Result<()> {
    let meta = fs::symlink_metadata(src)?;
    if meta.is_dir() {
        fs::remove_dir_all(src)
    } else {
        fs::remove_file(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("lazyfm-test-{name}-{nanos}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn unique_path_avoids_collisions() {
        let dir = scratch("unique");
        fs::write(dir.join("a.txt"), b"one").unwrap();
        fs::write(dir.join("a (2).txt"), b"two").unwrap();
        assert_eq!(unique_path(&dir, "a.txt"), dir.join("a (3).txt"));
        assert_eq!(unique_path(&dir, "b.txt"), dir.join("b.txt"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn copy_moves_a_directory_tree() {
        let dir = scratch("copy");
        let src = dir.join("src");
        fs::create_dir_all(src.join("nested")).unwrap();
        fs::write(src.join("nested/file.txt"), b"hi").unwrap();
        let dst = dir.join("dst");
        copy_path(&src, &dst).unwrap();
        assert_eq!(fs::read(dst.join("nested/file.txt")).unwrap(), b"hi");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn move_then_remove_leaves_nothing_behind() {
        let dir = scratch("move");
        let src = dir.join("one.txt");
        fs::write(&src, b"data").unwrap();
        let dst = dir.join("two.txt");
        move_path(&src, &dst).unwrap();
        assert!(!src.exists());
        assert_eq!(fs::read(&dst).unwrap(), b"data");
        remove_path(&dst).unwrap();
        assert!(!dst.exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
