# lazyfm spec

oct 21, 2026 · @zion resende

a lazygit-shaped tui file manager for linux that also flashes isos to usb sticks,
burns/rips/mounts optical discs, and previews your files like it's got something
to prove. ranger walked so lazyfm could... also walk, but smugger.

the pitch: one tui for "where's that file", "dd this iso to a stick", and
"what the hell is on this dvd". no tabs in your terminal, tabs in the app.

## stack

- rust + ratatui + crossterm. cargo's already here and you've shipped rust in npd.
- deps stay tiny: `ratatui`, `crossterm`, `libc`. that's the whole list. seriously.
- everything else (optical, archives, image preview) shells out behind a runtime
  check, so the core builds and runs even if half the box is missing.
- explicit over clever: one `Action` enum, one keymap table, plain structs for
  state. no macro magic, no framework hiding control flow.

## architecture

```
              +-- tabbar (tabs, each = own cwd) ------------+
              | sidebar (places) | filelist  | preview       |
              |  bookmarks       | entries   | text / hex /  |
              |  mounted devices | cursor    | image / info  |
              +------------------+-----------+---------------+
              | statusbar: clipboard, selection, hints        |
              +-----------------------------------------------+
                     overlay layer: confirm | input | progress | help | error
```

- `app.rs` holds the state: `tabs: Vec<Tab>`, `sidebar`, `focus`,
  `clipboard: Option<Cut|Copy(Vec<PathBuf>)>`, `overlay: Option<Overlay>`,
  `drag: Option<DragState>`, `settings`.
- a `Tab` = `cwd`, `entries`, `cursor`, `scroll`, `marked: HashSet<PathBuf>`.
- every keybinding maps to an `Action`; `handle_action(&mut App, Action)` mutates
  state. this is lazygit's context -> action split, minus the go.
- one event loop: crossterm `EventStream`, render each tick, dispatch key/mouse.
- mouse dnd: `Down` on a row starts `DragState{src}`; `Drag` draws a ghost and
  highlights the drop target; `Up` over a sidebar place / other tab / dir asks
  copy-vs-move, then runs it.

## feature modules

| module    | does                                                                   | backed by        |
|-----------|------------------------------------------------------------------------|------------------|
| `fs_ops`  | copy/cut/paste/move/rename/mkdir/delete; cross-device move = copy+unlink; progress for big ops | std::fs |
| `devices` | enumerate block devs (size, removable, ro, mountpoint, backing `/`)     | `/sys/block` parse, `lsblk -P` fallback |
| `flash`   | iso -> usb in-process: `open(O_WRONLY)`, 1-4 mib chunks, fsync, bytes/speed progress | libc |
| `optical` | burn (growisofs/xorriso), mount/eject (udisksctl/eject), browse iso (`xorriso -osirrox`), rip (`readom`) | shell-out |
| `archive` | generic extract by extension                                            | `unzip` / `7z`   |
| `preview` | text, hex, image, file-type icon                                        | see below        |
| `icons`   | theme -> (extension -> glyph) tables, switchable in settings            | in-tree tables   |

## preview + icons

preview pane, in priority order:

1. text: first ~200 lines, tab-expanded, no wrap (horizontal scroll later).
2. hex: for binaries, 16 bytes/row, offset + ascii gutter. the classic.
3. image: try the kitty graphics protocol when `KITTY_WINDOW_ID` or `GHOSTTY_*`
   is set; else `chafa` if installed; else a `[w]x[h] <format> <size>` info box.
   no hard dep on any renderer. alacritty does not speak kitty graphics, so on
   this box you'll get the info box unless you switch terminals. that's fine.
4. info: perms, owner, size, mtime, symlink target, whatever else we scraped.

icons: a `theme` is a map from extension to a glyph. ship at least:

- `ascii`  — `[C]`, `[RS]`, `[PY]`, `[SH]`, `[M]`, `[<>]` for dirs. works anywhere.
- `nerd`   — nerd font glyphs. you've got nerd fonts installed, so this is the daily driver.
- `emoji`  — for terminals with no glyph font, so your .c file can be a little
  c-shaped badge instead of tofu.

the .c file getting an actual "C" icon is non-negotiable and frankly the reason
this project exists.

## safety (flashing)

- refuse if the target is mounted, is the disk backing `/`, or is smaller than the iso.
- type-to-confirm the device name before a single byte moves.
- euid != 0 -> offer `sudo` re-exec with a clear prompt. browse stays unprivileged,
  only flashing wants the big hat.

## milestones

one at a time. do not start the next until the previous "done when" has been
*run*, not assumed. after m0 the thing already browses files, so it degrades
gracefully if nothing else ever ships.

1. **m0 skeleton** — layout, tabbar, sidebar, file list, navigate, select, quit,
   resize. *done when:* it browses `~` and `~/dvds` cleanly and survives an
   aggressive window resize.
2. **m1 file ops** — clipboard, copy/cut/paste/move, rename, mkdir, delete,
   confirm + input overlays. *done when:* it does daily file juggling without
   leaving the tui for a shell.
3. **m2 tabs + sidebar + dnd** — multiple tabs, bookmark places, mounted-devices
   section, full mouse drag-and-drop. *done when:* dragging a file onto a sidebar
   place moves it there.
4. **m3 usb flash** — device enumeration, guards, in-process writer, progress
   overlay. *done when:* a flashed usb stick actually boots.
5. **m4 optical** — burn iso to disc, mount/eject/browse, browse iso read-only,
   rip disc -> iso. *done when:* you burn a real dvd and mount it back.
6. **m5 polish** — archive extract, preview pane, icon themes, search/filter,
   help overlay, `lazyfm.toml`, and a `STATUS.md` that stops lying to us.

## rules for claude code

- one milestone at a time; the "done when" gets run before moving on.
- `STATUS.md` tracks each milestone as "attempted, not finished" or "finished",
  and only flips to finished after the check passes for real.
- deps stay minimal. optical/archive/image are optional runtime paths, never
  compile-time requirements.
- explicit over clever: fixed structs, visible state, no hidden control flow.
- linux only. block-device code lives in one module so a port only replaces it.
- prose, ui strings, and docs are lowercase. rust types stay `CamelCase` because
  the compiler and clippy have opinions and life's too short.

## open questions

- [x] root escalation: `sudo` re-exec with a clear prompt.
- [x] device list source: hand-parse `/sys/block`, `lsblk -P` as fallback.
- [ ] preview pane images: kitty protocol + chafa detect is the plan. revisit if
      you ever daily-drive alacritty and want real pixels there.
- [ ] config: hand-rolled `key=value` (no dep) vs pull in `toml`. leaning hand-rolled
      on principle, since the whole point is a tiny dep tree.
