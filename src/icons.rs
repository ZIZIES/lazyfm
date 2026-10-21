use crate::entry::Entry;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconTheme {
    Ascii,
    Nerd,
    Emoji,
}

impl IconTheme {
    pub fn next(self) -> Self {
        match self {
            IconTheme::Ascii => IconTheme::Nerd,
            IconTheme::Nerd => IconTheme::Emoji,
            IconTheme::Emoji => IconTheme::Ascii,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            IconTheme::Ascii => "ascii",
            IconTheme::Nerd => "nerd",
            IconTheme::Emoji => "emoji",
        }
    }
}

pub fn kind_key(entry: &Entry) -> &'static str {
    let ext = entry.extension().unwrap_or_default();
    let named = entry.name.to_ascii_lowercase();
    if entry.is_symlink {
        "symlink"
    } else if entry.is_dir {
        "dir"
    } else if named == "makefile"
        || named.starts_with("makefile.")
        || named == "gnu makefile"
        || named == "cmakelists.txt"
        || named == "justfile"
    {
        "make"
    } else if named == "dockerfile"
        || named == "containerfile"
        || named.starts_with("docker-compose.")
    {
        "docker"
    } else if named == "cargo.toml" {
        "cargo"
    } else if named == ".gitignore"
        || named == ".gitattributes"
        || named == ".gitmodules"
        || named == ".gitconfig"
    {
        "git"
    } else if named == "license" || named.starts_with("license.") || named.starts_with("copying") {
        "license"
    } else if named == "readme" || named == "readme.txt" {
        "markdown"
    } else if named == ".env" || named.starts_with(".env.") {
        "config"
    } else if named.ends_with(".pkg.tar.zst") {
        "package"
    } else if ext.is_empty() && entry.is_exec {
        "exec"
    } else {
        classify(&ext)
    }
}

pub fn glyph(theme: IconTheme, entry: &Entry) -> String {
    let key = kind_key(entry);

    match theme {
        IconTheme::Ascii => ascii(key),
        IconTheme::Nerd => nerd(key),
        IconTheme::Emoji => emoji(key),
    }
    .to_string()
}

fn classify(ext: &str) -> &'static str {
    match ext {
        "c" => "c",
        "h" => "h",
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => "cpp",
        "cs" | "csx" => "csharp",
        "rs" => "rust",
        "py" => "python",
        "sh" | "bash" | "zsh" | "fish" => "shell",
        "js" | "mjs" | "cjs" => "js",
        "ts" => "ts",
        "jsx" | "tsx" => "react",
        "html" | "htm" | "xhtml" => "html",
        "css" | "scss" | "sass" | "less" => "css",
        "java" => "java",
        "kt" | "kts" => "kotlin",
        "scala" => "scala",
        "rb" | "gemspec" => "ruby",
        "php" => "php",
        "lua" => "lua",
        "swift" => "swift",
        "zig" => "zig",
        "s" | "asm" => "asm",
        "wasm" => "wasm",
        "go" => "go",
        "json" => "json",
        "toml" | "ini" | "conf" | "cfg" | "yaml" | "yml" | "properties" | "env" => "config",
        "xml" | "xsd" | "xsl" => "xml",
        "md" | "markdown" => "markdown",
        "txt" | "log" => "text",
        "lock" => "lock",
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "ico" => "image",
        "mp3" | "flac" | "wav" | "ogg" | "m4a" => "audio",
        "mp4" | "mkv" | "webm" | "mov" | "avi" => "video",
        "zip" | "tar" | "gz" | "xz" | "bz2" | "7z" | "rar" | "zst" => "archive",
        "deb" | "rpm" | "apk" | "appimage" => "package",
        "sql" => "sql",
        "db" | "sqlite" | "sqlite3" => "database",
        "csv" | "tsv" | "xls" | "xlsx" | "ods" => "spreadsheet",
        "doc" | "docx" | "odt" | "rtf" => "document",
        "ppt" | "pptx" | "odp" => "presentation",
        "epub" | "mobi" | "azw" | "azw3" => "ebook",
        "ttf" | "otf" | "woff" | "woff2" => "font",
        "torrent" => "torrent",
        "srt" | "ass" | "ssa" | "vtt" | "sub" => "subtitle",
        "stl" | "obj" | "fbx" | "blend" | "gltf" | "glb" | "step" | "stp" => "model",
        "pem" | "crt" | "cer" | "p12" | "pfx" => "certificate",
        "pdf" => "pdf",
        "iso" | "img" => "iso",
        _ => "file",
    }
}

fn ascii(key: &str) -> &'static str {
    match key {
        "dir" => "[<>]",
        "symlink" => "[->]",
        "c" | "h" => "[C]",
        "cpp" => "[C+]",
        "csharp" => "[C#]",
        "rust" => "[RS]",
        "python" => "[PY]",
        "shell" => "[SH]",
        "js" => "[JS]",
        "ts" => "[TS]",
        "react" => "[JSX]",
        "html" => "[HTML]",
        "css" => "[CSS]",
        "java" => "[JAVA]",
        "kotlin" => "[KT]",
        "scala" => "[SC]",
        "ruby" => "[RB]",
        "php" => "[PHP]",
        "lua" => "[LUA]",
        "swift" => "[SW]",
        "zig" => "[ZIG]",
        "asm" => "[ASM]",
        "wasm" => "[WASM]",
        "go" => "[GO]",
        "json" => "{ }",
        "config" => "[CFG]",
        "xml" => "[XML]",
        "markdown" => "[MD]",
        "text" => "[TX]",
        "make" => "[MK]",
        "lock" => "[LK]",
        "git" => "[GIT]",
        "docker" => "[DOCK]",
        "cargo" => "[CARGO]",
        "license" => "[LIC]",
        "image" => "[IM]",
        "audio" => "[AU]",
        "video" => "[VD]",
        "archive" => "[AR]",
        "package" => "[PKG]",
        "sql" => "[SQL]",
        "database" => "[DB]",
        "spreadsheet" => "[XLS]",
        "document" => "[DOC]",
        "presentation" => "[PPT]",
        "ebook" => "[BOOK]",
        "font" => "[FONT]",
        "torrent" => "[TOR]",
        "subtitle" => "[SUB]",
        "model" => "[3D]",
        "certificate" => "[CERT]",
        "pdf" => "[PDF]",
        "iso" => "[ISO]",
        "exec" => "[X]",
        _ => "[ ]",
    }
}

fn nerd(key: &str) -> &'static str {
    match key {
        "dir" => "\u{f07b}",
        "symlink" => "\u{f0c1}",
        "c" | "h" => "\u{e61e}",
        "cpp" => "\u{e61d}",
        "csharp" => "\u{e648}",
        "rust" => "\u{e7a8}",
        "python" => "\u{e73c}",
        "shell" => "\u{f489}",
        "js" => "\u{e74e}",
        "ts" => "\u{e628}",
        "react" => "\u{e7ba}",
        "html" => "\u{e736}",
        "css" => "\u{e749}",
        "java" => "\u{e738}",
        "kotlin" => "\u{e634}",
        "scala" => "\u{e737}",
        "ruby" => "\u{e739}",
        "php" => "\u{e73d}",
        "lua" => "\u{e620}",
        "swift" => "\u{e755}",
        "zig" => "\u{e6a9}",
        "asm" => "\u{f471}",
        "wasm" => "\u{e6a1}",
        "go" => "\u{e626}",
        "json" => "\u{e60b}",
        "config" => "\u{e615}",
        "xml" => "\u{f1c9}",
        "markdown" => "\u{e73e}",
        "text" => "\u{f15c}",
        "make" => "\u{e673}",
        "lock" => "\u{f023}",
        "git" => "\u{e702}",
        "docker" => "\u{e7b0}",
        "cargo" => "\u{e7a8}",
        "license" => "\u{f0a3}",
        "image" => "\u{f1c5}",
        "audio" => "\u{f001}",
        "video" => "\u{f03d}",
        "archive" => "\u{f410}",
        "package" => "\u{f187}",
        "sql" => "\u{e706}",
        "database" => "\u{f1c0}",
        "spreadsheet" => "\u{f0ce}",
        "document" => "\u{f15c}",
        "presentation" => "\u{f1c4}",
        "ebook" => "\u{f02d}",
        "font" => "\u{f031}",
        "torrent" => "\u{f0ed}",
        "subtitle" => "\u{f1c8}",
        "model" => "\u{f1b2}",
        "certificate" => "\u{f0a3}",
        "pdf" => "\u{f1c1}",
        "iso" => "\u{f0a0}",
        "exec" => "\u{f489}",
        _ => "\u{f15b}",
    }
}

fn emoji(key: &str) -> &'static str {
    match key {
        "dir" => "📁",
        "symlink" => "🔗",
        "c" | "h" => "🅲",
        "cpp" => "🅲",
        "csharp" => "♯",
        "rust" => "🦀",
        "python" => "🐍",
        "shell" => "🐚",
        "js" => "🟨",
        "ts" => "🔷",
        "react" => "⚛️",
        "html" => "🌐",
        "css" => "🎨",
        "java" => "☕",
        "kotlin" => "🟪",
        "scala" => "🔴",
        "ruby" => "💎",
        "php" => "🐘",
        "lua" => "🌙",
        "swift" => "🐦",
        "zig" => "⚡",
        "asm" => "🔩",
        "wasm" => "🕸️",
        "go" => "🐹",
        "json" => "🧩",
        "config" => "⚙️",
        "xml" => "🧾",
        "markdown" => "📝",
        "text" => "📄",
        "make" => "🔨",
        "lock" => "🔒",
        "git" => "🌿",
        "docker" => "🐳",
        "cargo" => "🦀",
        "license" => "📜",
        "image" => "🖼️",
        "audio" => "🎵",
        "video" => "🎬",
        "archive" => "📦",
        "package" => "📦",
        "sql" => "🗃️",
        "database" => "🗄️",
        "spreadsheet" => "📊",
        "document" => "📃",
        "presentation" => "📽️",
        "ebook" => "📚",
        "font" => "🔤",
        "torrent" => "⬇️",
        "subtitle" => "💬",
        "model" => "🧊",
        "certificate" => "📜",
        "pdf" => "📕",
        "iso" => "💿",
        "exec" => "⚡",
        _ => "📄",
    }
}

pub fn label(entry: &Entry) -> &'static str {
    match kind_key(entry) {
        "dir" => "directory",
        "symlink" => "symbolic link",
        "c" => "c source",
        "h" => "c header",
        "cpp" => "c++",
        "csharp" => "c#",
        "rust" => "rust",
        "python" => "python",
        "shell" => "shell",
        "js" => "javascript",
        "ts" => "typescript",
        "react" => "react/jsx",
        "html" => "html",
        "css" => "css",
        "java" => "java",
        "kotlin" => "kotlin",
        "scala" => "scala",
        "ruby" => "ruby",
        "php" => "php",
        "lua" => "lua",
        "swift" => "swift",
        "zig" => "zig",
        "asm" => "assembly",
        "wasm" => "webassembly",
        "go" => "go",
        "json" => "json",
        "config" => "config",
        "xml" => "xml",
        "markdown" => "markdown",
        "text" => "text",
        "make" => "makefile",
        "lock" => "lockfile",
        "git" => "git config",
        "docker" => "docker",
        "cargo" => "cargo manifest",
        "license" => "license",
        "image" => "image",
        "audio" => "audio",
        "video" => "video",
        "archive" => "archive",
        "package" => "package",
        "sql" => "sql",
        "database" => "database",
        "spreadsheet" => "spreadsheet",
        "document" => "document",
        "presentation" => "presentation",
        "ebook" => "ebook",
        "font" => "font",
        "torrent" => "torrent",
        "subtitle" => "subtitle",
        "model" => "3d model",
        "certificate" => "certificate",
        "pdf" => "pdf",
        "iso" => "disk image",
        "exec" => "executable",
        _ => "file",
    }
}

pub const SPRITE_W: usize = 16;
pub const SPRITE_H: usize = 16;

/// 16x16 pixel art, one char per pixel. '.' is transparent, every other char
/// is a key into PALETTE. ui draws two pixel rows per terminal row with
/// half blocks, so each pixel comes out roughly square.
pub type Sprite = [&'static str; SPRITE_H];

/// pixel char -> rgb. colors are from the endesga-32 palette plus violet and
/// lavender for the languages that insisted.
pub const PALETTE: [(char, [u8; 3]); 27] = [
    ('k', [0x18, 0x14, 0x25]), // black
    ('S', [0x26, 0x2b, 0x44]), // night
    ('s', [0x3a, 0x44, 0x66]), // slate
    ('G', [0x5a, 0x69, 0x88]), // steel
    ('g', [0x8b, 0x9b, 0xb4]), // gray
    ('W', [0xc0, 0xcb, 0xdc]), // silver
    ('w', [0xff, 0xff, 0xff]), // white
    ('R', [0xa2, 0x26, 0x33]), // maroon
    ('r', [0xe4, 0x3b, 0x44]), // red
    ('m', [0xf6, 0x75, 0x7a]), // pink
    ('O', [0xbe, 0x4a, 0x2f]), // rust
    ('o', [0xf7, 0x76, 0x22]), // orange
    ('a', [0xfe, 0xae, 0x34]), // amber
    ('y', [0xfe, 0xe7, 0x61]), // yellow
    ('e', [0xea, 0xd4, 0xaa]), // beige
    ('t', [0xe4, 0xa6, 0x72]), // tan
    ('T', [0xb8, 0x6f, 0x50]), // brown
    ('D', [0x73, 0x3e, 0x39]), // umber
    ('N', [0x3e, 0x89, 0x48]), // forest
    ('n', [0x63, 0xc7, 0x4d]), // green
    ('B', [0x12, 0x4e, 0x89]), // navy
    ('b', [0x00, 0x99, 0xdb]), // blue
    ('c', [0x2c, 0xe8, 0xf5]), // cyan
    ('P', [0x68, 0x38, 0x6c]), // plum
    ('p', [0xb5, 0x50, 0x88]), // magenta
    ('v', [0x7a, 0x5c, 0xf0]), // violet
    ('l', [0x88, 0x92, 0xbf]), // lavender
];

pub fn pixel_rgb(c: char) -> Option<[u8; 3]> {
    PALETTE.iter().find(|(k, _)| *k == c).map(|(_, rgb)| *rgb)
}

pub fn art(entry: &Entry) -> &'static Sprite {
    sprite(kind_key(entry))
}

fn sprite(key: &str) -> &'static Sprite {
    match key {
        "dir" => &DIR,
        "symlink" => &LINK,
        "c" => &C,
        "h" => &H,
        "cpp" => &CPP,
        "csharp" => &CSHARP,
        "rust" => &RUST,
        "python" => &PYTHON,
        "shell" => &SHELL,
        "js" => &JS,
        "ts" => &TS,
        "react" => &REACT,
        "html" => &HTML,
        "css" => &CSS,
        "java" => &JAVA,
        "kotlin" => &KOTLIN,
        "scala" => &SCALA,
        "ruby" => &RUBY,
        "php" => &PHP,
        "lua" => &LUA,
        "swift" => &SWIFT,
        "zig" => &ZIG,
        "asm" => &ASM,
        "wasm" => &WASM,
        "go" => &GO,
        "json" => &JSON,
        "config" => &CONFIG,
        "xml" => &XML,
        "markdown" => &MARKDOWN,
        "text" => &TEXT,
        "make" => &MAKE,
        "lock" => &LOCK,
        "git" => &GIT,
        "docker" => &DOCKER,
        "cargo" => &CARGO,
        "license" => &LICENSE,
        "image" => &IMAGE,
        "audio" => &AUDIO,
        "video" => &VIDEO,
        "archive" => &ARCHIVE,
        "package" => &PACKAGE,
        "sql" => &SQL,
        "database" => &DATABASE,
        "spreadsheet" => &SPREADSHEET,
        "document" => &DOCUMENT,
        "presentation" => &PRESENTATION,
        "ebook" => &EBOOK,
        "font" => &FONT,
        "torrent" => &TORRENT,
        "subtitle" => &SUBTITLE,
        "model" => &MODEL,
        "certificate" => &CERTIFICATE,
        "pdf" => &PDF,
        "iso" => &ISO,
        "exec" => &EXEC,
        _ => &FILE,
    }
}

const FILE: Sprite = [
    "..GGGGGGGG......",
    "..GwwwwwwGG.....",
    "..GwwwwwwGWG....",
    "..GwwwwwwGWWG...",
    "..GwwwwwwGGGGG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GWWWWWWWWWWG..",
    "..GGGGGGGGGGGG..",
];

const TEXT: Sprite = [
    "..GGGGGGGG......",
    "..GwwwwwwGG.....",
    "..GwggggwGWG....",
    "..GwwwwwwGWWG...",
    "..GwwwwwwGGGGG..",
    "..GwwwwwwwwwwG..",
    "..GwggggggggwG..",
    "..GwwwwwwwwwwG..",
    "..GwggggggwwwG..",
    "..GwwwwwwwwwwG..",
    "..GwggggggggwG..",
    "..GwwwwwwwwwwG..",
    "..GwgggggwwwwG..",
    "..GwwwwwwwwwwG..",
    "..GWWWWWWWWWWG..",
    "..GGGGGGGGGGGG..",
];

const DOCUMENT: Sprite = [
    "..BBBBBBBB......",
    "..BbbbbbbBB.....",
    "..BbwwwwbBcB....",
    "..BbbbbbbBccB...",
    "..BbbbbbbBBBBB..",
    "..BbbbbbbbbbbB..",
    "..BbwwwwwwwwbB..",
    "..BbbbbbbbbbbB..",
    "..BbwwwwwwwwbB..",
    "..BbbbbbbbbbbB..",
    "..BbwwwwwwbbbB..",
    "..BbbbbbbbbbbB..",
    "..BbwwwwwwwbbB..",
    "..BbbbbbbbbbbB..",
    "..BBBBBBBBBBBB..",
    "..BBBBBBBBBBBB..",
];

const SPREADSHEET: Sprite = [
    "..NNNNNNNN......",
    "..NnnnnnnNN.....",
    "..NnnnnnnNWN....",
    "..NnnnnnnNWWN...",
    "..NnnnnnnNNNNN..",
    "..NnnnnnnnnnnN..",
    "..NnNNNNNNNNnN..",
    "..NnNyyNwwwNnN..",
    "..NnNyyNwwwNnN..",
    "..NnNNNNNNNNnN..",
    "..NnNwwNwwwNnN..",
    "..NnNwwNwwwNnN..",
    "..NnNNNNNNNNnN..",
    "..NnnnnnnnnnnN..",
    "..NNNNNNNNNNNN..",
    "..NNNNNNNNNNNN..",
];

const PDF: Sprite = [
    "..GGGGGGGG......",
    "..GwwwwwwGG.....",
    "..GwggggwGWG....",
    "..GwwwwwwGWWG...",
    "..GwwwwwwGGGGG..",
    "..GwggggggggwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "rrrrrrrrrrrrrG..",
    "rwwrrwwrrwwwrG..",
    "rwrwrwrwrwrrrG..",
    "rwwrrwrwrwwrrG..",
    "rwrrrwrwrwrrrG..",
    "rwrrrwwrrwrrrG..",
    "RRRRRRRRRRRRRG..",
    "..GGGGGGGGGGGG..",
];

const SQL: Sprite = [
    "..GGGGGGGG......",
    "..GwwwwwwGG.....",
    "..GwggggwGWG....",
    "..GwwwwwwGWWG...",
    "..GwwwwwwGGGGG..",
    "..GwggggggggwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "bbbbbbbbbbbbbG..",
    "bbwwbbwbbwbbbG..",
    "bwbbbwbwbwbbbG..",
    "bbwbbwbwbwbbbG..",
    "bbbwbwbwbwbbbG..",
    "bwwbbbwwbwwwbG..",
    "BBBBBBBBBBBBBG..",
    "..GGGGGGGGGGGG..",
];

const XML: Sprite = [
    "..GGGGGGGG......",
    "..GwwwwwwGG.....",
    "..GwggggwGWG....",
    "..GwwwwwwGWWG...",
    "..GwwwwwwGGGGG..",
    "..GwggggggggwG..",
    "..GwwwwwwwwwwG..",
    "..GwwwwwwwwwwG..",
    "oooooooooooooG..",
    "ooowooowowoooG..",
    "oowoooowoowooG..",
    "owoooowoooowoG..",
    "oowoowoooowooG..",
    "ooowowooowoooG..",
    "OOOOOOOOOOOOOG..",
    "..GGGGGGGGGGGG..",
];

const ARCHIVE: Sprite = [
    "..GGGGGGgG......",
    "..GwwwwgGGG.....",
    "..GwwwwGgGWG....",
    "..GwwwwgGGWWG...",
    "..GwwwwGgGGGGG..",
    "..GwwwwgGwwwwG..",
    "..GwwwwGgwwwwG..",
    "..GwwwwgGwwwwG..",
    "..GwwwwGgwwwwG..",
    "..GwwwwgGwwwwG..",
    "..GwwwGGGGwwwG..",
    "..GwwwGwwGwwwG..",
    "..GwwwGwwGwwwG..",
    "..GwwwGGGGwwwG..",
    "..GWWWWWWWWWWG..",
    "..GGGGGGGGGGGG..",
];

const DIR: Sprite = [
    "................",
    "................",
    ".TTTTTT.........",
    "TaaaaaaT........",
    "TaaaaaaaTTTTTTT.",
    "TaaaaaaaaaaaaaaT",
    "TaaaaaaaaaaaaaaT",
    "TTTTTTTTTTTTTTTT",
    "TyyyyyyyyyyyyyyT",
    "TyyyyyyyyyyyyyyT",
    "TyyyyyyyyyyyyyyT",
    "TyyyyyyyyyyyyyyT",
    "TyyyyyyyyyyyyyyT",
    "TaaaaaaaaaaaaaaT",
    ".TTTTTTTTTTTTTT.",
    "................",
];

const LINK: Sprite = [
    "................",
    "................",
    "..........c.....",
    "..........cc....",
    "..........ccc...",
    ".....ccccccccc..",
    "...cccccccccccc.",
    "..cbbbbbbbbbbb..",
    "..cbb.....bbb...",
    "..cbb.....bb....",
    "..cbb.....b.....",
    "..cbb...........",
    "..cbb...........",
    "..cbb...........",
    "..cbb...........",
    "................",
];

const C: Sprite = [
    ".......bb.......",
    ".....bbbbbb.....",
    "...bbbbbbbbbb...",
    ".bbbbbbbbbbbbbb.",
    "bbbbbbwwwwwwbbbb",
    "bbbbbwwwwwwwbbbb",
    "bbbbwwwbbbbbbbbb",
    "bbbbwwbbbbbbbbbb",
    "bbbbwwbbBBBBBBBB",
    "bbbbwwwbBBBBBBBB",
    "bbbbbwwwwwwwBBBB",
    "bbbbbbwwwwwwBBBB",
    ".bbbbbbbBBBBBBB.",
    "...bbbbbBBBBB...",
    ".....bbbBBB.....",
    ".......bB.......",
];

const H: Sprite = [
    ".......bb.......",
    ".....bbbbbb.....",
    "...bbbbbbbbbb...",
    ".bbbbbbbbbbbbbb.",
    "bbbbbwwbbwwbbbbb",
    "bbbbbwwbbwwbbbbb",
    "bbbbbwwbbwwbbbbb",
    "bbbbbwwwwwwbbbbb",
    "bbbbbwwwwwwBBBBB",
    "bbbbbwwbBwwBBBBB",
    "bbbbbwwbBwwBBBBB",
    "bbbbbwwbBwwBBBBB",
    ".bbbbbbbBBBBBBB.",
    "...bbbbbBBBBB...",
    ".....bbbBBB.....",
    ".......bB.......",
];

const CPP: Sprite = [
    ".......bb.......",
    ".....bbbbbb.....",
    "...bbbbbbbbbb...",
    ".bbbbbbbbbbbbbb.",
    "bbbbbbbbbbbbbbbb",
    "bbwwwwwbbbbbbbbb",
    "bwwbbbbbbbbbbbbb",
    "bwwbbbbbbwbbbwbb",
    "bwwbbbbbwwwBwwwB",
    "bwwbbbbbBwBBBwBB",
    "bwwbbbbbBBBBBBBB",
    "bbwwwwwbBBBBBBBB",
    ".bbbbbbbBBBBBBB.",
    "...bbbbbBBBBB...",
    ".....bbbBBB.....",
    ".......bB.......",
];

const CSHARP: Sprite = [
    ".......pp.......",
    ".....pppppp.....",
    "...pppppppppp...",
    ".pppppppppppppp.",
    "pppppppppppppppp",
    "pppwwwwwpppppppp",
    "ppwwppppppwpwppp",
    "ppwwpppppwwwwwpp",
    "ppwwppppPPwPwPPP",
    "ppwwppppPwwwwwPP",
    "ppwwppppPPwPwPPP",
    "pppwwwwwPPPPPPPP",
    ".pppppppPPPPPPP.",
    "...pppppPPPPP...",
    ".....pppPPP.....",
    ".......pP.......",
];

const RUST: Sprite = [
    "................",
    "................",
    ".o.o........o.o.",
    ".ooo........ooo.",
    "..o..........o..",
    "..o..oooooo..o..",
    "..o.oooooooo.o..",
    "..oookwookwooo..",
    "..oookkookkooo..",
    ".oooooooooooooo.",
    ".ooooooOOoooooo.",
    "..oooooooooooo..",
    "..O.O.O..O.O.O..",
    ".O.O.O....O.O.O.",
    "................",
    "................",
];

const PYTHON: Sprite = [
    "................",
    ".....bbbbb......",
    "....bwbbbbb.....",
    "....bbbbbbb.....",
    "....bbbbbbb.....",
    ".bbbbbbbbbb.yyy.",
    "bbbbbbbbbbb.yyyy",
    "bbbb........yyyy",
    "bbbb........yyyy",
    "bbbb.yyyyyyyyyyy",
    ".bbb.yyyyyyyyyy.",
    ".....yyyyyyy....",
    ".....yyyyyyy....",
    ".....yyyyywy....",
    "......yyyyy.....",
    "................",
];

const SHELL: Sprite = [
    "................",
    "................",
    ".gggggggggggggg.",
    ".grgagngggggggg.",
    ".gkkkkkkkkkkkkg.",
    ".gknkkkkkkkkkkg.",
    ".gkknkkkkkkkkkg.",
    ".gkkknkkkkkkkkg.",
    ".gkknkkkkkkkkkg.",
    ".gknkkknnnnkkkg.",
    ".gkkkkkkkkkkkkg.",
    ".gkkkkkkkkkkkkg.",
    ".gggggggggggggg.",
    "................",
    "................",
    "................",
];

const JS: Sprite = [
    "................",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyyyyyyyyyy.",
    ".yyyyyykkyykkky.",
    ".yyyyyykkykyyyy.",
    ".yyyyyykkykyyyy.",
    ".yyyyyykkyykkyy.",
    ".yyyyyykkyyyyky.",
    ".yyyykykkyyyyky.",
    ".yyyyykkyykkkyy.",
    ".yyyyyyyyyyyyyy.",
    "................",
];

const TS: Sprite = [
    "................",
    ".bbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    ".bbbwwwwwbbwwwb.",
    ".bbbbbwbbbwbbbb.",
    ".bbbbbwbbbwbbbb.",
    ".bbbbbwbbbbwwbb.",
    ".bbbbbwbbbbbbwb.",
    ".bbbbbwbbbbbbwb.",
    ".bbbbbwbbbwwwbb.",
    ".bbbbbbbbbbbbbb.",
    "................",
];

const REACT: Sprite = [
    "................",
    "....cc....cc....",
    "...cccc..cccc...",
    "...cc..cc..cc...",
    "...c...cc...c...",
    "....cccccccc....",
    ".cccc..cc..cccc.",
    "cc...cccccc...cc",
    "cc...cccccc...cc",
    ".cccc..cc..cccc.",
    "....cccccccc....",
    "...c...cc...c...",
    "...cc..cc..cc...",
    "...cccc..cccc...",
    "....cc....cc....",
    "................",
];

const HTML: Sprite = [
    "................",
    ".OOOOOOOooooooo.",
    ".OOOOOOOooooooo.",
    ".OOOOwwwwwooooo.",
    ".OOOOwOOooooooo.",
    ".OOOOwwwwoooooo.",
    ".OOOOOOOowooooo.",
    ".OOOOOOOowooooo.",
    ".OOOOwOOowooooo.",
    ".OOOOOwwwoooooo.",
    "..OOOOOOoooooo..",
    "..OOOOOOoooooo..",
    "...OOOOOooooo...",
    "....OOOOoooo....",
    "......OOoo......",
    "................",
];

const CSS: Sprite = [
    "................",
    ".BBBBBBBbbbbbbb.",
    ".BBBBBBBbbbbbbb.",
    ".BBBBwwwwbbbbbb.",
    ".BBBBBBBbwbbbbb.",
    ".BBBBBBwwbbbbbb.",
    ".BBBBBBBbwbbbbb.",
    ".BBBBBBBbwbbbbb.",
    ".BBBBwBBbwbbbbb.",
    ".BBBBBwwwbbbbbb.",
    "..BBBBBBbbbbbb..",
    "..BBBBBBbbbbbb..",
    "...BBBBBbbbbb...",
    "....BBBBbbbb....",
    "......BBbb......",
    "................",
];

const JAVA: Sprite = [
    "................",
    ".......r...r....",
    "......r...r.....",
    ".......r...r....",
    "......r...r.....",
    "................",
    "..WWWWWWWWWWW...",
    "..bDDDDDDDDDbbb.",
    "..bbbbbbbbbbb.b.",
    "..bbbbbbbbbbb.b.",
    "..bbbbbbbbbbbbb.",
    "...bbbbbbbbb....",
    "....BBBBBBB.....",
    ".BBBBBBBBBBBBB..",
    "................",
    "................",
];

const KOTLIN: Sprite = [
    "................",
    ".ppppoooooooooo.",
    ".pppppoooooooo..",
    ".ppppppoooooo...",
    ".vppppppoooo....",
    ".vvppppppoo.....",
    ".vvvpppppp......",
    ".vvvvpppp.......",
    ".vvvvvppp.......",
    ".vvvvvvppp......",
    ".vvvvvvvppp.....",
    ".vvvvvvvvppp....",
    ".vvvvvvvvvppp...",
    ".vvvvvvvvvvppp..",
    ".vvvvvvvvvvvppp.",
    "................",
];

const SCALA: Sprite = [
    "................",
    "................",
    ".....rrrrrrrrr..",
    "....rrrrrrrrr...",
    "...RRRRRRRRR....",
    "................",
    ".....rrrrrrrrr..",
    "....rrrrrrrrr...",
    "...RRRRRRRRR....",
    "................",
    ".....rrrrrrrrr..",
    "....rrrrrrrrr...",
    "...RRRRRRRRR....",
    "................",
    "................",
    "................",
];

const RUBY: Sprite = [
    "................",
    "................",
    "...RRRRRRRRRR...",
    "..RrmmrrrrmmrR..",
    ".RrmmrrrrrrmmrR.",
    "RRRRRRRRRRRRRRRR",
    ".RrrrrrmmrrrrrR.",
    "..RrrrrmmrrrrR..",
    "...RrrrmmrrrR...",
    "....RrrmmrrR....",
    ".....RrmmrR.....",
    "......RmmR......",
    ".......RR.......",
    "................",
    "................",
    "................",
];

const PHP: Sprite = [
    "................",
    "................",
    "................",
    ".....llllll.....",
    "..llllllllllll..",
    ".llllllllllllll.",
    "llSSllSlllSSllll",
    "llSlSlSlllSlSlll",
    "llSSllSSllSSllll",
    "llSlllSlSlSlllll",
    ".lSlllSlSlSllll.",
    "..llllllllllll..",
    ".....llllll.....",
    "................",
    "................",
    "................",
];

const LUA: Sprite = [
    ".............BB.",
    "............BBBB",
    "............BBBB",
    "....BBBBBB...BB.",
    "..BBBBBBBBBB....",
    "..BBBBBBwwwB....",
    ".BBBBBBBwwwBB...",
    ".BBBBBBBwwwBB...",
    ".BBBBBBBBBBBB...",
    ".BBBBBBBBBBBB...",
    ".BBBBBBBBBBBB...",
    ".BBBBBBBBBBBB...",
    "..BBBBBBBBBB....",
    "..BBBBBBBBBB....",
    "....BBBBBB......",
    "................",
];

const SWIFT: Sprite = [
    "................",
    "..oooooooooooo..",
    ".oooooooooooooo.",
    ".oooooooowooooo.",
    ".roooooooowwooo.",
    ".rrwooooooowwoo.",
    ".rrrwwoooooowoo.",
    ".rrrrwwwoooowwo.",
    ".rrrrrwwwwwowwo.",
    ".rrrrrrowwwwwwo.",
    ".rwrrrrrwwwwwwo.",
    ".rrwwwwwwwwwwoo.",
    ".rrrrwwwwwwoowo.",
    ".rrrrrrrrrroooo.",
    "..rrrrrrrrrroo..",
    "................",
];

const ZIG: Sprite = [
    "................",
    "................",
    "..oooooooooooo..",
    "..OOOOOOOOOOOO..",
    "..........ooo...",
    ".........ooo....",
    "........ooo.....",
    ".......ooo......",
    "......ooo.......",
    ".....ooo........",
    "....ooo.........",
    "...ooo..........",
    "..oooooooooooo..",
    "..OOOOOOOOOOOO..",
    "................",
    "................",
];

const ASM: Sprite = [
    "................",
    "....a.a.a.a.....",
    "....a.a.a.a.....",
    "...sssssssss....",
    ".aasssssssssaa..",
    "...ssgggggss....",
    ".aassgWGGgssaa..",
    "...ssgGGGgss....",
    ".aassgGGGgssaa..",
    "...ssgggggss....",
    ".aasssssssssaa..",
    "...sssssssss....",
    "....a.a.a.a.....",
    "....a.a.a.a.....",
    "................",
    "................",
];

const WASM: Sprite = [
    "................",
    ".vvvvv....vvvvv.",
    ".vvvvvv..vvvvvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvwvvvwvvwwvvv.",
    ".vvwvvvwvwvvwvv.",
    ".vvwvwvwvwwwwvv.",
    ".vvwwwwwvwvvwvv.",
    ".vvvwvwvvwvvwvv.",
    ".vvvvvvvvvvvvvv.",
    ".vvvvvvvvvvvvvv.",
    "................",
];

const GO: Sprite = [
    "................",
    "..bb........bb..",
    ".bcbbbbbbbbbbcb.",
    ".bbwwwbbbbwwwbb.",
    "bbwwwwwbbwwwwwbb",
    "bbwwkkwbbwkkwwbb",
    "bbwwkkwbbwkkwwbb",
    "bbbwwwbbbbwwwbbb",
    "bbbbbttkkttbbbbb",
    "bbbbbttttttbbbbb",
    "bbbbbbbwwbbbbbbb",
    ".bbbbbbwwbbbbbb.",
    ".bbbbbbbbbbbbbb.",
    "..bbbbbbbbbbbb..",
    "....bbbbbbbb....",
    "................",
];

const JSON: Sprite = [
    "................",
    "................",
    "...aaa....aaa...",
    "..aa........aa..",
    "..aa........aa..",
    "..aa........aa..",
    "..aa........aa..",
    ".aa..........aa.",
    "..aa........aa..",
    "..aa........aa..",
    "..aa..a..a..aa..",
    "..aa........aa..",
    "...aaa....aaa...",
    "................",
    "................",
    "................",
];

const CONFIG: Sprite = [
    "......WWWW......",
    "......WWWW......",
    "...WW.WWWW.gg...",
    "..WWWWWWWggggg..",
    "..WWWWWWgggggg..",
    "...WWWWgggggg...",
    "WWWWWW....ggGGGG",
    "WWWWWg....gGGGGG",
    "WWWWgg....GGGGGG",
    "WWWggg....GGGGGG",
    "...gggggGGGGG...",
    "..gggggGGGGGGG..",
    "..ggggGGGGGGGG..",
    "...gg.GGGG.GG...",
    "......GGGG......",
    "......GGGG......",
];

const MARKDOWN: Sprite = [
    "................",
    "................",
    "................",
    ".wwwwwwwwwwwwww.",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwSwwwSwwwwSwwww",
    "wwSSwSSwwwwSwwww",
    "wwSwSwSwwSSSSSww",
    "wwSwwwSwwwSSSwww",
    "wwSwwwSwwwwSwwww",
    "wwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwww.",
    "................",
    "................",
    "................",
];

const MAKE: Sprite = [
    ".........W......",
    "........WWW.....",
    ".......WWWgg....",
    "......WWWgg.....",
    ".....WWWgg......",
    "....WWWgg.......",
    "...WWWgg........",
    "..GWWggTT.......",
    ".GGGgg.tTT......",
    "GGGGG...tTT.....",
    ".GGG.....tTT....",
    "..G.......tTD...",
    "...........DDD..",
    "............DDD.",
    ".............DD.",
    "................",
];

const LOCK: Sprite = [
    "................",
    ".....GGGGGG.....",
    "....GG....GG....",
    "...GG......GG...",
    "...GG......GG...",
    "...GG......GG...",
    "..TTTTTTTTTTTT..",
    "..TyyyyyyyyyyT..",
    "..TaaaaaaaaaaT..",
    "..TaaaaDDaaaaT..",
    "..TaaaaDDaaaaT..",
    "..TaaaaDDaaaaT..",
    "..TaaaaaaaaaaT..",
    "..TOOOOOOOOOOT..",
    "..TTTTTTTTTTTT..",
    "................",
];

const GIT: Sprite = [
    ".......rr.......",
    "......wwrr......",
    ".....rwwrrr.....",
    "....rrwwrrrr....",
    "...rrrwwrrrrr...",
    "..rrrwwwwrrrrr..",
    ".rrrrwwwwwrrrrr.",
    "rrrrrrwwrwwwwrrr",
    "rrrrrrwwrrwwwwrr",
    ".rrrrrwwrrwwwwr.",
    "..rrrrwwrrrwwr..",
    "...rrwwwwrrrr...",
    "....rwwwwrrr....",
    ".....rwwrrr.....",
    "......rrrr......",
    ".......rr.......",
];

const DOCKER: Sprite = [
    "................",
    "..........bb....",
    "..........bb....",
    "................",
    "....bb.bb.bb....",
    "....bb.bb.bb....",
    "..............b.",
    ".bb.bb.bb.bb.bbb",
    ".bb.bb.bb.bb..b.",
    "bbbbbbbbbbbbbbb.",
    "bbwbbbbbbbbbbbb.",
    ".bbbbbbbbbbbbb..",
    "..bbbbbbbbbbb...",
    "....bbbbbbb.....",
    "................",
    "................",
];

const CARGO: Sprite = [
    "................",
    "................",
    ".DTTTTTTTTTTTTD.",
    ".TTTTTTTTTTTTTT.",
    ".TTtttDttDtTTTT.",
    ".TTtttDttDTTtTT.",
    ".TTtttDttTTttTT.",
    ".TTtttDtTTtttTT.",
    ".TTtttDTTDtttTT.",
    ".TTtttTTtDtttTT.",
    ".TTttTTttDtttTT.",
    ".TTTTTDttDtttTT.",
    ".TTTTTTTTTTTTTT.",
    ".DTTTTTTTTTTTTD.",
    "................",
    "................",
];

const LICENSE: Sprite = [
    "................",
    "..TTTTTTTTTTTT..",
    ".TttttttttttttT.",
    "..TTTTTTTTTTTT..",
    "...eeeeeeeeee...",
    "...ettttttttt...",
    "...eeeeeeeeee...",
    "...etttttteee...",
    "...eeeeeeeeee...",
    "...ettttttttt...",
    "...eeeeeeeeee...",
    "...ettttteeee...",
    "..TTTTTTTTTTTT..",
    ".TttttttttttttT.",
    "..TTTTTTTTTTTT..",
    "................",
];

const CERTIFICATE: Sprite = [
    "................",
    "................",
    ".GGGGGGGGGGGGGG.",
    ".GwwwwwwwwwwwwG.",
    ".GwggggggggggwG.",
    ".GwwwwwwwwwwwwG.",
    ".GwggggggwwwwwG.",
    ".GwwwwwwwrrrwwG.",
    ".GwgggggrryrrwG.",
    ".GwwwwwwryyyrwG.",
    ".GGGGGGGrryrrGG.",
    ".........rrr....",
    ".........R..R...",
    ".........R..R...",
    ".........R..R...",
    "................",
];

const IMAGE: Sprite = [
    "................",
    "................",
    "WWWWWWWWWWWWWWWW",
    "WbbbbbbbbbbbbbbW",
    "WbbbbbbbbbyyybbW",
    "WbbbbbbbbbyyybbW",
    "WbbbbNbbbbyyybbW",
    "WbbbNNNbbbbbbbbW",
    "WbbNNNNNbbbnbbbW",
    "WbNNNNNNNbnnnbbW",
    "WNNNNNNNNNnnnnbW",
    "WNNNNNNNNNNnnnnW",
    "WNNNNNNNNNNNnnnW",
    "WWWWWWWWWWWWWWWW",
    "................",
    "................",
];

const AUDIO: Sprite = [
    "................",
    "......pppppppp..",
    "......pppppppp..",
    "......p......p..",
    "......p......p..",
    "......p......p..",
    "......p......p..",
    "......p......p..",
    "......p......p..",
    "...pppp...pppp..",
    "..ppppp..ppppp..",
    "..pmppp..pmppp..",
    "..pppp...pppp...",
    "...pp.....pp....",
    "................",
    "................",
];

const VIDEO: Sprite = [
    "................",
    "................",
    ".wwkkwwkkwwkkww.",
    ".wkkwwkkwwkkwwk.",
    "................",
    ".kkwwkkwwkkwwkk.",
    ".kwwkkwwkkwwkkw.",
    ".ssssssssssssss.",
    ".sSSSSSSSSSSSSs.",
    ".sSGGGGGSSSSSSs.",
    ".sSSSSSSSSSSSSs.",
    ".sSGGGGGGGGSSSs.",
    ".sSSSSSSSSSSSSs.",
    ".ssssssssssssss.",
    "................",
    "................",
];

const ISO: Sprite = [
    ".......gg.......",
    "....ggWWWWgg....",
    "...gpppWWWWWg...",
    "..gccppWWWWWWg..",
    ".gccccpWWWWWWWg.",
    ".gccccpggWWWWWg.",
    ".WWWccggggWWWWW.",
    "gWWWWgg..ggWWWWg",
    "gWWWWgg..ggWWWWg",
    ".WWWWWggggyyWWW.",
    ".gWWWWWggWyyyyg.",
    ".gWWWWWWWWyyyyg.",
    "..gWWWWWWWWyyg..",
    "...gWWWWWWWWg...",
    "....ggWWWWgg....",
    ".......gg.......",
];

const PACKAGE: Sprite = [
    "................",
    ".......tt.......",
    ".....tttttt.....",
    "...tttttteett...",
    ".tttttteetttttt.",
    ".TTtteettttttDD.",
    ".TTTTttttttDDDD.",
    ".TTTTTTttDDDDDD.",
    ".TTTTTTTDDDDDDD.",
    ".TTTTTTTDDDDDDD.",
    ".TTTTTTTDDDDDDD.",
    ".TTTTTTTDDDDDDD.",
    "..TTTTTTDDDDDD..",
    "....TTTTDDDD....",
    "......TTDD......",
    "................",
];

const MODEL: Sprite = [
    "................",
    ".......cc.......",
    ".....cccccc.....",
    "...cccccccccc...",
    ".cccccccccccccc.",
    ".bbccccccccccBB.",
    ".bbbbccccccBBBB.",
    ".bbbbbbccBBBBBB.",
    ".bbbbbbbBBBBBBB.",
    ".bbbbbbbBBBBBBB.",
    ".bbbbbbbBBBBBBB.",
    ".bbbbbbbBBBBBBB.",
    "..bbbbbbBBBBBB..",
    "....bbbbBBBB....",
    "......bbBB......",
    "................",
];

const DATABASE: Sprite = [
    "................",
    "................",
    "...cccccccccc...",
    ".cccccccccccccc.",
    ".bBccccccccccBb.",
    ".bbbBBBBBBBBbbb.",
    ".BbbbbbbbbbbbbB.",
    ".BBbbbbbbbbbbBB.",
    ".bbBBBBBBBBBBbb.",
    ".bbbbbbbbbbbbbb.",
    ".BbbbbbbbbbbbbB.",
    "..BBbbbbbbbbBB..",
    "....BBBBBBBB....",
    "................",
    "................",
    "................",
];

const PRESENTATION: Sprite = [
    "................",
    ".......GG.......",
    "GGGGGGGGGGGGGGGG",
    ".wwwwwwwwwwwwww.",
    ".wwwwwwwwwwaaww.",
    ".wwwwnnwwwwaaww.",
    ".wwwwnnwwwwaaww.",
    ".wwwwnnwwwwaaww.",
    ".wbbwnnwwwwaaww.",
    ".wbbwnnwrrwaaww.",
    ".wbbwnnwrrwaaww.",
    ".WWWWWWWWWWWWWW.",
    "GGGGGGGGGGGGGGGG",
    ".......GG.......",
    ".....GG..GG.....",
    "....GG....GG....",
];

const EBOOK: Sprite = [
    "................",
    "...RRRRRRRRRR...",
    "..RrrrrrrrrrrR..",
    "..RrrrrrrrrrrRe.",
    "..RraaaaaaaarRe.",
    "..RrrrrrrrrrrRe.",
    "..RraaaaaarrrRe.",
    "..RrrrrrrrrrrRe.",
    "..RrrrrrrrrrrRe.",
    "..RrrrrrrrrrrRe.",
    "..RrrrrrrrrrrRe.",
    "..RrrrrrrrrrrRe.",
    "..RRRRRRRRRRRRe.",
    "...eeeeeeeeeeee.",
    "................",
    "................",
];

const FONT: Sprite = [
    "................",
    "................",
    "................",
    "....p...........",
    "...ppp..........",
    "...ppp..........",
    "..pp.pp.........",
    "..pp.pp.........",
    ".pp...pp...mmm..",
    ".ppppppp......m.",
    "pp.....pp..mmmm.",
    "pp.....pp.m...m.",
    "ppp...ppp..mmmm.",
    "................",
    "................",
    "................",
];

const TORRENT: Sprite = [
    "................",
    "......nnnN......",
    "......nnnN......",
    "......nnnN......",
    "......nnnN......",
    "......nnnN......",
    "..nnnnnnnnnnNN..",
    "...nnnnnnnnNN...",
    "....nnnnnnNN....",
    ".....nnnnNN.....",
    "......nnNN......",
    ".......NN.......",
    "g..............g",
    "g..............g",
    "gggggggggggggggg",
    "................",
];

const SUBTITLE: Sprite = [
    "................",
    "................",
    "................",
    ".wwwwwwwwwwwwww.",
    "wwwwwwwwwwwwwwww",
    "wwwwwwwwwwwwwwww",
    "wwwwSSSwwwSSSwww",
    "wwwSwwwwwSwwwwww",
    "wwwSwwwwwSwwwwww",
    "wwwSwwwwwSwwwwww",
    "wwwwSSSwwwSSSwww",
    "wwwwwwwwwwwwwwww",
    ".wwwwwwwwwwwwww.",
    "................",
    "................",
    "................",
];

const EXEC: Sprite = [
    "................",
    "....NNNNNNNN....",
    "...NNnnnnnnNN...",
    "..NnnnnnnnnnnN..",
    ".NNnnnnnnnnnnNN.",
    ".NnnnnwnnnnnnnN.",
    ".NnnnnwwwnnnnnN.",
    ".NnnnnwwwwwnnnN.",
    ".NnnnnwwwwwnnnN.",
    ".NnnnnwwwnnnnnN.",
    ".NnnnnwnnnnnnnN.",
    ".NNnnnnnnnnnnNN.",
    "..NnnnnnnnnnnN..",
    "...NNnnnnnnNN...",
    "....NNNNNNNN....",
    "................",
];

#[cfg(test)]
mod tests {
    use super::{FILE, IconTheme, SPRITE_W, art, glyph, kind_key, pixel_rgb, sprite};
    use crate::entry::Entry;
    use std::path::PathBuf;

    fn file(name: &str) -> Entry {
        Entry {
            name: name.to_string(),
            path: PathBuf::from(name),
            is_dir: false,
            is_symlink: false,
            is_exec: false,
            size: 0,
            mode: 0,
            uid: 0,
            gid: 0,
            mtime: None,
        }
    }

    #[test]
    fn recognizes_more_file_types() {
        let cases = [
            ("main.cs", "csharp"),
            ("app.jsx", "react"),
            ("index.html", "html"),
            ("style.scss", "css"),
            ("Main.java", "java"),
            ("main.kt", "kotlin"),
            ("script.lua", "lua"),
            ("kernel.zig", "zig"),
            ("boot.s", "asm"),
            ("module.wasm", "wasm"),
            ("query.sql", "sql"),
            ("data.sqlite3", "database"),
            ("budget.xlsx", "spreadsheet"),
            ("report.docx", "document"),
            ("slides.pptx", "presentation"),
            ("font.woff2", "font"),
            ("mesh.glb", "model"),
            ("linux.pkg.tar.zst", "package"),
        ];

        for (name, expected) in cases {
            let entry = file(name);
            assert_eq!(kind_key(&entry), expected, "{name}");
            assert!(!std::ptr::eq(art(&entry), &FILE), "{name} has no sprite");
            for theme in [IconTheme::Ascii, IconTheme::Nerd, IconTheme::Emoji] {
                assert!(!glyph(theme, &entry).is_empty(), "{name} has no glyph");
            }
        }
    }

    #[test]
    fn recognizes_project_dotfiles_and_special_names() {
        for (name, expected) in [
            (".gitignore", "git"),
            ("Dockerfile", "docker"),
            ("Cargo.toml", "cargo"),
            ("LICENSE", "license"),
            ("Makefile", "make"),
        ] {
            assert_eq!(kind_key(&file(name)), expected, "{name}");
        }
    }

    #[test]
    fn symlink_to_directory_keeps_its_link_icon() {
        let mut entry = file("shortcut");
        entry.is_symlink = true;
        entry.is_dir = true;
        assert_eq!(kind_key(&entry), "symlink");
        assert_eq!(glyph(IconTheme::Ascii, &entry), "[->]");
    }

    #[test]
    fn every_sprite_is_16x16_and_uses_the_palette() {
        let keys = [
            "dir",
            "symlink",
            "c",
            "h",
            "cpp",
            "csharp",
            "rust",
            "python",
            "shell",
            "js",
            "ts",
            "react",
            "html",
            "css",
            "java",
            "kotlin",
            "scala",
            "ruby",
            "php",
            "lua",
            "swift",
            "zig",
            "asm",
            "wasm",
            "go",
            "json",
            "config",
            "xml",
            "markdown",
            "text",
            "make",
            "lock",
            "git",
            "docker",
            "cargo",
            "license",
            "image",
            "audio",
            "video",
            "archive",
            "package",
            "sql",
            "database",
            "spreadsheet",
            "document",
            "presentation",
            "ebook",
            "font",
            "torrent",
            "subtitle",
            "model",
            "certificate",
            "pdf",
            "iso",
            "exec",
            "file",
        ];
        for key in keys {
            for (y, row) in sprite(key).iter().enumerate() {
                assert_eq!(row.chars().count(), SPRITE_W, "{key} row {y}");
                for c in row.chars() {
                    assert!(c == '.' || pixel_rgb(c).is_some(), "{key} row {y}: '{c}'");
                }
            }
        }
    }
}
