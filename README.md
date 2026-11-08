# lazyfm

a lazy tui file manager. no setup, no docs needed. type `lazyfm` in a directory and you're in it.

heads up: most of this was built with ai while i learn rust. code feedback is very welcome and appreciated and welcome from anybody. :)

![demo](demo.gif)

i tried other tui file managers, but i was not really a fan. i wanted to make another option, for people that prefer a carded look.

- **zero config.** just run it
- **card-style layout** instead of three crowded columns
- **file menu on enter.** open, open with, rename, delete, copy, cut, new file/folder
- **keys always shown** at the bottom
- **pretty icons**, with a few themes (`i` to cycle)
- **tabs**, hidden file toggle, mouse scrolling

## install

```
cargo install lazyfm
```

needs rust 1.85+ (edition 2024). unix only (linux/macos), no windows.

## keys

| key | does |
|---|---|
| `j`/`k` or `↓`/`↑` | down/up |
| `h`/`l` or `←`/`→` | parent/enter |
| `enter` | open a dir, or pop the file menu for a file |
| `m` | file menu for any entry: open, open with, rename, delete, copy, cut, new file/folder, more. on `..` it's a folder menu (new file, new folder, paste) |
| `g`/`G` | top/bottom |
| `ctrl-d` / `ctrl-u` | page down/up |
| `n` / `w` | new/close tab |
| `[` / `]` | previous/next tab |
| `.` | toggle hidden files |
| `i` | cycle icon theme |
| `r` | refresh |
| `q` | quit |

`esc` closes a menu. delete asks you to confirm first.

## how it tells `.obj` files apart

`.obj` means two different things: a compiled object file or a wavefront 3d model. lazyfm reads the first 512 bytes. if there's a NUL byte in there it's treated as an object file, otherwise it's treated as a wavefront model, since those are plain text.

it's a heuristic, so a weird file (like UTF-16 text) could get the wrong icon.

## license

apache-2.0
