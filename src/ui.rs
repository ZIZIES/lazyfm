use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Clear, List, ListItem, ListState, Paragraph, Wrap};

use crate::app::{App, Focus};
use crate::entry::Entry;
use crate::icons;

const LEFT_W: u16 = 26;
const RIGHT_W: u16 = 38;
const LOG_H: u16 = 6;
const SHOW_LEFT_AT: u16 = 50;
const SHOW_RIGHT_AT: u16 = 92;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(area);

    draw_tabbar(frame, rows[0], app);

    let body = Layout::vertical([Constraint::Min(6), Constraint::Length(LOG_H)]).split(rows[1]);
    draw_top(frame, body[0], app);
    draw_log(frame, body[1], app);

    draw_status(frame, rows[2], app);

    if app.show_help {
        draw_help(frame, area);
    }
}

fn draw_top(frame: &mut Frame, area: Rect, app: &mut App) {
    let show_left = area.width >= SHOW_LEFT_AT;
    let show_right = area.width >= SHOW_RIGHT_AT;

    let constraints: Vec<Constraint> = match (show_left, show_right) {
        (true, true) => vec![
            Constraint::Length(LEFT_W),
            Constraint::Min(20),
            Constraint::Length(RIGHT_W),
        ],
        (true, false) => vec![Constraint::Length(LEFT_W), Constraint::Min(20)],
        (false, true) => vec![Constraint::Min(20), Constraint::Length(RIGHT_W)],
        (false, false) => vec![Constraint::Min(10)],
    };

    let cols = Layout::horizontal(constraints).split(area);
    let mut idx = 0;

    if show_left {
        draw_left(frame, cols[idx], app);
        idx += 1;
    }
    draw_main(frame, cols[idx], app);
    idx += 1;
    if show_right {
        draw_right(frame, cols[idx], app);
    }
}

fn draw_tabbar(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans: Vec<Span> = vec![Span::styled(
        " lazyfm ",
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )];
    spans.push(Span::raw(" "));
    for (i, tab) in app.tabs.iter().enumerate() {
        let label = format!(" {}:{} ", i + 1, shorten(&tab.cwd));
        if i == app.active {
            spans.push(Span::styled(
                label,
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(label, Style::default().fg(Color::Gray)));
        }
        spans.push(Span::raw(" "));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn draw_left(frame: &mut Frame, area: Rect, app: &App) {
    if app.left_shows_places() {
        draw_places(frame, area, app);
    } else {
        draw_file_info(frame, area, app);
    }
}

fn draw_places(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Left;
    let items: Vec<ListItem> = app
        .sidebar
        .iter()
        .map(|p| {
            let line = Line::from(vec![
                Span::styled(
                    icons::glyph(app.settings.icons, &synth_dir(&p.path)),
                    Style::default().fg(Color::Blue),
                ),
                Span::raw(" "),
                Span::raw(p.name.clone()),
            ]);
            ListItem::new(line)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.sidebar_cursor));

    let list = List::new(items)
        .block(panel("places", focused))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_file_info(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Left;
    let (title, text) = match app.tab().selected() {
        Some(e) => {
            // name, then art, then details: keeps the sprite near the top so long
            // wrapped details can't push it off the bottom of the panel.
            let mut lines = vec![Line::from(Span::styled(
                e.name.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ))];
            lines.extend(sprite_lines(icons::art(e), area.width));
            lines.push(Line::from(""));
            lines.extend(info_text(e));
            (format!(" file: {} ", icons::label(e)), lines)
        }
        None => (" file ".to_string(), vec![Line::from("nothing selected.")]),
    };
    frame.render_widget(
        Paragraph::new(text)
            .block(panel(&title, focused))
            .wrap(Wrap { trim: false }),
        area,
    );
}

/// two sprite rows per terminal row: '▀' with fg = top pixel, bg = bottom pixel.
fn sprite_lines(sprite: &icons::Sprite, width: u16) -> Vec<Line<'static>> {
    let inner = width.saturating_sub(2) as usize;
    let pad = " ".repeat(inner.saturating_sub(icons::SPRITE_W) / 2);
    sprite
        .chunks(2)
        .map(|pair| {
            let mut spans = vec![Span::raw(pad.clone())];
            for (top, bottom) in pair[0].chars().zip(pair[1].chars()) {
                spans.push(half_block(pixel(top), pixel(bottom)));
            }
            Line::from(spans)
        })
        .collect()
}

fn half_block(top: Option<Color>, bottom: Option<Color>) -> Span<'static> {
    match (top, bottom) {
        (None, None) => Span::raw(" "),
        (Some(t), None) => Span::styled("▀", Style::default().fg(t)),
        (None, Some(b)) => Span::styled("▄", Style::default().fg(b)),
        (Some(t), Some(b)) => Span::styled("▀", Style::default().fg(t).bg(b)),
    }
}

fn pixel(c: char) -> Option<Color> {
    icons::pixel_rgb(c).map(|[r, g, b]| Color::Rgb(r, g, b))
}

fn draw_main(frame: &mut Frame, area: Rect, app: &App) {
    let tab = app.tab();
    let focused = app.focus == Focus::Main;

    let items: Vec<ListItem> = tab
        .entries
        .iter()
        .map(|e| {
            let icon = icons::glyph(app.settings.icons, e);
            let line = Line::from(vec![
                Span::styled(format!("{} ", icon), icon_style(e)),
                Span::styled(e.name.clone(), name_style(e)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let title = format!(" {} ", shorten(&tab.cwd));
    let mut state = ListState::default();
    state.select(Some(tab.cursor));

    let list = List::new(items)
        .block(panel(&title, focused))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_right(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Right;
    let tab = app.tab();
    let (dirs, files) = tab.counts();

    let mut lines = vec![
        Line::from(Span::styled(
            shorten(&tab.cwd),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        kv("entries", format!("{} total", tab.entries.len())),
        kv("dirs", dirs.to_string()),
        kv("files", files.to_string()),
    ];

    if let Some((free, total)) = fs_space(&tab.cwd) {
        lines.push(Line::from(""));
        lines.push(kv("disk", format!("{} free", human_size(free))));
        lines.push(kv("of", human_size(total)));
    }

    if let Some(sel) = tab.selected()
        && !sel.is_dir
    {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "selection",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(kv("name", sel.name.clone()));
        lines.push(kv("size", human_size(sel.size)));
    }

    frame.render_widget(Paragraph::new(lines).block(panel("info", focused)), area);
}

fn draw_log(frame: &mut Frame, area: Rect, app: &App) {
    let capacity = area.height.saturating_sub(2) as usize;
    let start = app.log.len().saturating_sub(capacity);
    let lines: Vec<Line> = app.log[start..]
        .iter()
        .map(|m| Line::from(Span::styled(m.clone(), Style::default().fg(Color::Gray))))
        .collect();
    frame.render_widget(Paragraph::new(lines).block(panel("log", false)), area);
}

fn info_text(e: &Entry) -> Vec<Line<'static>> {
    let mut lines = vec![kv("kind", e.kind().to_string())];
    if e.is_symlink
        && let Ok(target) = std::fs::read_link(&e.path)
    {
        lines.push(kv("target", target.display().to_string()));
    }
    if !e.is_dir {
        lines.push(kv(
            "size",
            format!("{} ({} bytes)", human_size(e.size), e.size),
        ));
    }
    lines.push(kv(
        "perm",
        format!("{}{}", file_type_char(e), perm_string(e.mode)),
    ));
    lines.push(kv("owner", format!("uid {} gid {}", e.uid, e.gid)));
    if let Some(t) = e.mtime {
        lines.push(kv("mtime", format_time(t)));
    }
    lines.push(kv("path", e.path.display().to_string()));
    lines
}

fn kv(key: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{:<7}", key), Style::default().fg(Color::DarkGray)),
        Span::raw(value),
    ])
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let left = " j/k move  enter open  tab pane  n tab  . hidden  i icons  ? help ";
    let right = format!(
        "tab {}/{}  hidden:{}  icons:{} ",
        app.active + 1,
        app.tabs.len(),
        if app.settings.show_hidden {
            "on"
        } else {
            "off"
        },
        app.settings.icons.label(),
    );
    let width = area.width as usize;
    let pad = width.saturating_sub(left.chars().count() + right.chars().count());
    let line = Line::from(vec![
        Span::styled(left, Style::default().fg(Color::Green)),
        Span::raw(" ".repeat(pad)),
        Span::styled(right, Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(Span::styled(
            "lazyfm keys",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  j / k          move down / up"),
        Line::from("  enter / l      open dir (or pick a place)"),
        Line::from("  h / backspace  go to parent"),
        Line::from("  g / G          top / bottom"),
        Line::from("  .              toggle hidden files"),
        Line::from("  r              refresh"),
        Line::from("  tab            cycle pane focus"),
        Line::from("  n / w          new / close tab"),
        Line::from("  [ / ]          prev / next tab"),
        Line::from("  i              cycle icon theme"),
        Line::from("  ?              toggle this"),
        Line::from("  q              quit"),
        Line::from(""),
        Line::from(Span::styled(
            "  the rest is coming. patience, grasshopper.",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let popup = centered(area, 50, text.len() as u16 + 2);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(text).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" help ")
                .border_style(Style::default().fg(Color::Green)),
        ),
        popup,
    );
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let [popup] = Layout::horizontal([Constraint::Length(w.min(area.width))])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::vertical([Constraint::Length(h.min(area.height))])
        .flex(Flex::Center)
        .areas(popup);
    popup
}

fn panel(title: &str, focused: bool) -> Block<'_> {
    let style = if focused {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(format!(" {} ", title))
        .border_style(style)
}

fn name_style(e: &Entry) -> Style {
    if e.is_symlink {
        Style::default().fg(Color::Cyan)
    } else if e.is_dir {
        Style::default().fg(Color::Blue)
    } else if e.is_exec {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    }
}

fn icon_style(e: &Entry) -> Style {
    let color = match icons::kind_key(e) {
        "dir" => Color::Blue,
        "symlink" => Color::Cyan,
        "exec" | "git" => Color::Green,
        "c" | "h" | "cpp" | "csharp" | "rust" | "go" | "zig" | "asm" | "wasm" => Color::LightYellow,
        "python" | "java" | "kotlin" | "scala" | "ruby" | "php" | "lua" | "swift" => {
            Color::LightRed
        }
        "js" | "ts" | "react" | "html" | "css" | "xml" | "docker" => Color::LightCyan,
        "image" | "video" | "model" => Color::Magenta,
        "audio" => Color::LightGreen,
        "archive" | "package" | "iso" | "torrent" => Color::Yellow,
        "sql" | "database" => Color::Cyan,
        "lock" | "certificate" | "license" => Color::LightRed,
        "spreadsheet" | "document" | "presentation" | "ebook" | "pdf" | "markdown" => {
            Color::LightBlue
        }
        "font" => Color::LightMagenta,
        _ => Color::Gray,
    };
    Style::default().fg(color)
}

fn shorten(path: &Path) -> String {
    let s = path.display().to_string();
    if let Ok(home) = std::env::var("HOME")
        && let Some(rest) = s.strip_prefix(&home)
    {
        return format!("~{}", rest);
    }
    s
}

fn synth_dir(path: &Path) -> Entry {
    Entry {
        name: path.display().to_string(),
        path: path.to_path_buf(),
        is_dir: true,
        is_symlink: false,
        is_exec: false,
        size: 0,
        mode: 0,
        uid: 0,
        gid: 0,
        mtime: None,
    }
}

fn file_type_char(e: &Entry) -> &'static str {
    if e.is_symlink {
        "l"
    } else if e.is_dir {
        "d"
    } else {
        "-"
    }
}

fn perm_string(mode: u32) -> String {
    let bits = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];
    bits.iter()
        .map(|(bit, ch)| if mode & bit != 0 { *ch } else { '-' })
        .collect()
}

fn human_size(n: u64) -> String {
    const UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    let mut size = n as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} B", n)
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

fn fs_space(path: &Path) -> Option<(u64, u64)> {
    let c = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::statvfs(c.as_ptr(), &mut st) };
    if rc != 0 {
        return None;
    }
    let bsize = st.f_frsize as u64;
    Some((st.f_bavail as u64 * bsize, st.f_blocks as u64 * bsize))
}

fn format_time(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02} utc",
        y,
        m,
        d,
        rem / 3600,
        (rem % 3600) / 60
    )
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}
