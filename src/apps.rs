//! Detects a handful of well-known GUI/CLI programs on `PATH` so the
//! "open with" submenu can list what is actually installed.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// (program, friendly label). Order is roughly "most generally useful first".
const CANDIDATES: &[(&str, &str)] = &[
    ("xdg-open", "default app"),
    ("nvim", "neovim"),
    ("vim", "vim"),
    ("nano", "nano"),
    ("micro", "micro"),
    ("code", "vs code"),
    ("codium", "vscodium"),
    ("mpv", "mpv"),
    ("vlc", "vlc"),
    ("ffplay", "ffplay"),
    ("imv", "imv"),
    ("feh", "feh"),
    ("sxiv", "sxiv"),
    ("eog", "eye of gnome"),
    ("gimp", "gimp"),
    ("zathura", "zathura"),
    ("evince", "evince"),
    ("okular", "okular"),
    ("libreoffice", "libreoffice"),
    ("file-roller", "file roller"),
    ("xarchiver", "xarchiver"),
];

pub fn detect() -> Vec<(&'static str, &'static str)> {
    CANDIDATES
        .iter()
        .copied()
        .filter(|(program, _)| find_on_path(program))
        .collect()
}

fn find_on_path(program: &str) -> bool {
    let Some(paths) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&paths).any(|dir| {
        let candidate = dir.join(program);
        is_executable(&candidate)
    })
}

fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
