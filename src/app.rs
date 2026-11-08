use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

use crate::action::Action;
use crate::apps;
use crate::entry::{self, Entry};
use crate::fsops;
use crate::icons::IconTheme;
use crate::overlay::{
    Confirm, ConfirmKind, Menu, MenuAction, MenuItem, Overlay, Prompt, PromptKind,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Left,
    Main,
    Right,
}

impl Focus {
    fn next(self) -> Self {
        match self {
            Focus::Left => Focus::Main,
            Focus::Main => Focus::Right,
            Focus::Right => Focus::Left,
        }
    }

    fn prev(self) -> Self {
        match self {
            Focus::Left => Focus::Right,
            Focus::Main => Focus::Left,
            Focus::Right => Focus::Main,
        }
    }
}

pub struct Tab {
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub cursor: usize,
}

impl Tab {
    fn load(cwd: PathBuf, show_hidden: bool) -> Tab {
        let entries = entry::read_dir(&cwd, show_hidden).unwrap_or_default();
        Tab {
            cwd,
            entries,
            cursor: 0,
        }
    }

    fn refresh(&mut self, show_hidden: bool) {
        self.entries = entry::read_dir(&self.cwd, show_hidden).unwrap_or_default();
        if self.cursor >= self.entries.len() {
            self.cursor = self.entries.len().saturating_sub(1);
        }
    }

    pub fn selected(&self) -> Option<&Entry> {
        self.entries.get(self.cursor)
    }

    pub fn counts(&self) -> (usize, usize) {
        let dirs = self.entries.iter().filter(|e| e.is_dir).count();
        (dirs, self.entries.len() - dirs)
    }
}

pub struct Place {
    pub name: String,
    pub path: PathBuf,
}

pub struct Settings {
    pub show_hidden: bool,
    pub icons: IconTheme,
}

#[derive(Clone)]
pub struct Clipboard {
    pub path: PathBuf,
    pub cut: bool,
}

pub struct App {
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub focus: Focus,
    pub sidebar: Vec<Place>,
    pub sidebar_cursor: usize,
    pub settings: Settings,
    pub should_quit: bool,
    pub overlay: Option<Overlay>,
    pub clipboard: Option<Clipboard>,
    pub status: String,
    pub log: Vec<String>,
}

impl App {
    pub fn new() -> io::Result<App> {
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let start = env::current_dir().unwrap_or_else(|_| PathBuf::from(&home));
        let settings = Settings {
            show_hidden: false,
            icons: IconTheme::Nerd,
        };
        let sidebar = build_sidebar(&home);
        let mut app = App {
            tabs: vec![Tab::load(start, settings.show_hidden)],
            active: 0,
            focus: Focus::Main,
            sidebar,
            sidebar_cursor: 0,
            settings,
            should_quit: false,
            overlay: None,
            clipboard: None,
            status: String::new(),
            log: Vec::new(),
        };
        app.set_status("welcome to lazyfm. the keys are down at the bottom.".to_string());
        Ok(app)
    }

    pub fn tab(&self) -> &Tab {
        &self.tabs[self.active]
    }

    pub fn tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }

    pub fn left_shows_places(&self) -> bool {
        match self.tab().selected() {
            Some(e) => e.is_dir && !e.is_symlink,
            None => true,
        }
    }

    pub fn set_status(&mut self, msg: String) {
        self.status = msg.clone();
        self.log.push(msg);
        if self.log.len() > 200 {
            self.log.drain(0..self.log.len() - 200);
        }
    }

    pub fn handle(&mut self, action: Action) {
        let action = match (self.focus, action) {
            (Focus::Left, Action::Down) if self.left_shows_places() => Action::SidebarDown,
            (Focus::Left, Action::Up) if self.left_shows_places() => Action::SidebarUp,
            (Focus::Left, Action::Enter) if self.left_shows_places() => Action::SidebarSelect,
            _ => action,
        };
        match action {
            Action::Quit => self.should_quit = true,
            Action::Down => self.move_cursor(1),
            Action::Up => self.move_cursor(-1),
            Action::PageDown => self.move_cursor(20),
            Action::PageUp => self.move_cursor(-20),
            Action::Top => self.tab_mut().cursor = 0,
            Action::Bottom => {
                let last = self.tab().entries.len().saturating_sub(1);
                self.tab_mut().cursor = last;
            }
            Action::Enter => self.enter(),
            Action::Menu => self.open_menu(),
            Action::Parent => self.go_parent(),
            Action::ToggleHidden => {
                self.settings.show_hidden = !self.settings.show_hidden;
                let hidden = self.settings.show_hidden;
                self.tab_mut().refresh(hidden);
                let msg = if hidden {
                    "hidden files: revealed. the skeletons are out."
                } else {
                    "hidden files: hidden again. tidy."
                };
                self.set_status(msg.to_string());
            }
            Action::Refresh => {
                let hidden = self.settings.show_hidden;
                self.tab_mut().refresh(hidden);
                self.set_status("refreshed. fresh off the disk.".to_string());
            }
            Action::FocusNext => self.focus = self.focus.next(),
            Action::FocusPrev => self.focus = self.focus.prev(),
            Action::SidebarDown => {
                if !self.sidebar.is_empty() {
                    self.sidebar_cursor = (self.sidebar_cursor + 1).min(self.sidebar.len() - 1);
                }
            }
            Action::SidebarUp => self.sidebar_cursor = self.sidebar_cursor.saturating_sub(1),
            Action::SidebarSelect => self.goto_place(),
            Action::NewTab => {
                let cwd = self.tab().cwd.clone();
                let hidden = self.settings.show_hidden;
                self.tabs.push(Tab::load(cwd, hidden));
                self.active = self.tabs.len() - 1;
                let msg = format!(
                    "tab {} opened. tabs in a terminal, very meta.",
                    self.active + 1
                );
                self.set_status(msg);
            }
            Action::CloseTab => {
                if self.tabs.len() > 1 {
                    self.tabs.remove(self.active);
                    if self.active >= self.tabs.len() {
                        self.active = self.tabs.len() - 1;
                    }
                    self.set_status("tab closed. no hard feelings.".to_string());
                } else {
                    self.set_status("last tab. can't close the only child.".to_string());
                }
            }
            Action::NextTab => {
                if !self.tabs.is_empty() {
                    self.active = (self.active + 1) % self.tabs.len();
                }
            }
            Action::PrevTab => {
                if !self.tabs.is_empty() {
                    self.active = (self.active + self.tabs.len() - 1) % self.tabs.len();
                }
            }
            Action::CycleIconTheme => {
                self.settings.icons = self.settings.icons.next();
                let msg = format!("icon theme: {}", self.settings.icons.label());
                self.set_status(msg);
            }
        }
    }

    /// Single entry point for key presses. Overlays are modal, so they get
    /// first crack before the normal keymap runs.
    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.overlay.is_some() {
            self.handle_overlay_key(key);
            return;
        }
        if let Some(action) = crate::keymap::map(key) {
            self.handle(action);
        }
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
        if self.overlay.is_some() {
            return;
        }
        match mouse.kind {
            MouseEventKind::ScrollDown => self.move_cursor(3),
            MouseEventKind::ScrollUp => self.move_cursor(-3),
            _ => {}
        }
    }

    fn move_cursor(&mut self, delta: i32) {
        let len = self.tab().entries.len();
        if len == 0 {
            return;
        }
        let cur = self.tab().cursor as i32;
        self.tab_mut().cursor = (cur + delta).clamp(0, len as i32 - 1) as usize;
    }

    fn enter(&mut self) {
        match self.tab().selected() {
            Some(e) if e.is_dir => {
                let target = e.path.clone();
                self.chdir(target);
            }
            Some(_) => self.open_menu(),
            None => {}
        }
    }

    // ---- overlays --------------------------------------------------------

    fn handle_overlay_key(&mut self, key: KeyEvent) {
        match self.overlay {
            Some(Overlay::Menu(_)) => self.handle_menu_key(key),
            Some(Overlay::Prompt(_)) => self.handle_prompt_key(key),
            Some(Overlay::Confirm(_)) => self.handle_confirm_key(key),
            None => {}
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        enum Nav {
            None,
            Pop,
            Choose(MenuAction),
        }
        let mut nav = Nav::None;
        if let Some(Overlay::Menu(menu)) = self.overlay.as_mut() {
            match key.code {
                KeyCode::Esc | KeyCode::Left | KeyCode::Backspace | KeyCode::Char('h') => {
                    nav = Nav::Pop;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if menu.cursor + 1 < menu.items.len() {
                        menu.cursor += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    menu.cursor = menu.cursor.saturating_sub(1);
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                    if let Some(item) = menu.items.get(menu.cursor) {
                        nav = Nav::Choose(item.action.clone());
                    }
                }
                _ => {}
            }
        }
        match nav {
            Nav::None => {}
            Nav::Pop => self.pop_menu(),
            Nav::Choose(action) => self.do_menu_action(action),
        }
    }

    fn pop_menu(&mut self) {
        let parent = match self.overlay.as_mut() {
            Some(Overlay::Menu(menu)) => menu.parent.take(),
            _ => None,
        };
        match parent {
            Some(parent) => self.overlay = Some(Overlay::Menu(*parent)),
            None => self.overlay = None,
        }
    }

    fn do_menu_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::Open => {
                self.overlay = None;
                self.open_selected();
            }
            MenuAction::OpenWith => self.open_with_menu(),
            MenuAction::OpenWithApp(program) => {
                self.overlay = None;
                self.open_with_program(&program);
            }
            MenuAction::OpenWithCustom => {
                self.overlay = None;
                self.start_prompt(PromptKind::OpenWith);
            }
            MenuAction::Rename => {
                self.overlay = None;
                self.start_prompt(PromptKind::Rename);
            }
            MenuAction::Delete => {
                self.overlay = None;
                self.start_confirm();
            }
            MenuAction::Copy => {
                self.overlay = None;
                self.set_clipboard(false);
            }
            MenuAction::Cut => {
                self.overlay = None;
                self.set_clipboard(true);
            }
            MenuAction::More => self.open_more_menu(),
            MenuAction::Paste => {
                self.overlay = None;
                self.paste();
            }
            MenuAction::New => self.open_new_menu(),
            MenuAction::NewFile => {
                self.overlay = None;
                self.start_prompt(PromptKind::NewFile);
            }
            MenuAction::NewFolder => {
                self.overlay = None;
                self.start_prompt(PromptKind::NewFolder);
            }
        }
    }

    fn open_menu(&mut self) {
        let selected = self
            .tab()
            .selected()
            .filter(|e| e.name != "..")
            .map(|e| e.name.clone());
        let Some(name) = selected else {
            self.open_dir_menu();
            return;
        };
        let items = vec![
            MenuItem::new("open", MenuAction::Open),
            MenuItem::new("open with...", MenuAction::OpenWith),
            MenuItem::new("rename", MenuAction::Rename),
            MenuItem::new("delete", MenuAction::Delete),
            MenuItem::new("copy", MenuAction::Copy),
            MenuItem::new("cut", MenuAction::Cut),
            MenuItem::new("new...", MenuAction::New),
            MenuItem::new("more...", MenuAction::More),
        ];
        self.overlay = Some(Overlay::Menu(Menu {
            title: format!(" {name} "),
            items,
            cursor: 0,
            parent: None,
        }));
    }

    fn open_with_menu(&mut self) {
        let mut items: Vec<MenuItem> = apps::detect()
            .into_iter()
            .map(|(program, label)| {
                MenuItem::new(
                    format!("{label} ({program})"),
                    MenuAction::OpenWithApp(program.to_string()),
                )
            })
            .collect();
        items.push(MenuItem::new(
            "custom command...",
            MenuAction::OpenWithCustom,
        ));
        self.push_submenu(" open with ".to_string(), items);
    }

    /// The menu for the folder itself, shown on ".." (which is all an empty
    /// folder has): no entry to act on, but you can still make things or paste.
    fn open_dir_menu(&mut self) {
        let name = self
            .tab()
            .cwd
            .file_name()
            .map(|n| format!("{}/", n.to_string_lossy()))
            .unwrap_or_else(|| "/".to_string());
        let items = vec![
            MenuItem::new("new file...", MenuAction::NewFile),
            MenuItem::new("new folder...", MenuAction::NewFolder),
            MenuItem::new(self.paste_label(), MenuAction::Paste),
        ];
        self.overlay = Some(Overlay::Menu(Menu {
            title: format!(" {name} "),
            items,
            cursor: 0,
            parent: None,
        }));
    }

    fn open_new_menu(&mut self) {
        let items = vec![
            MenuItem::new("file...", MenuAction::NewFile),
            MenuItem::new("folder...", MenuAction::NewFolder),
        ];
        self.push_submenu(" new ".to_string(), items);
    }

    fn open_more_menu(&mut self) {
        let items = vec![MenuItem::new(self.paste_label(), MenuAction::Paste)];
        self.push_submenu(" more ".to_string(), items);
    }

    fn paste_label(&self) -> &'static str {
        match &self.clipboard {
            Some(clip) if clip.cut => "paste (move here)",
            Some(_) => "paste (copy here)",
            None => "paste",
        }
    }

    fn push_submenu(&mut self, title: String, items: Vec<MenuItem>) {
        let parent = match self.overlay.take() {
            Some(Overlay::Menu(menu)) => Some(Box::new(menu)),
            other => {
                self.overlay = other;
                None
            }
        };
        self.overlay = Some(Overlay::Menu(Menu {
            title,
            items,
            cursor: 0,
            parent,
        }));
    }

    fn start_prompt(&mut self, kind: PromptKind) {
        // rename/open-with act on the selection; the "new" prompts only need the cwd.
        let selected = self.tab().selected().map(|e| e.name.clone());
        let (title, label, input) = match (kind, selected) {
            (PromptKind::Rename, Some(name)) => (
                " rename ".to_string(),
                format!("rename \"{name}\" to:"),
                name.chars().collect::<Vec<char>>(),
            ),
            (PromptKind::OpenWith, Some(name)) => (
                " open with ".to_string(),
                format!("command to open \"{name}\" with:"),
                Vec::new(),
            ),
            (PromptKind::Rename | PromptKind::OpenWith, None) => return,
            (PromptKind::NewFile, _) => (
                " new file ".to_string(),
                "file name:".to_string(),
                Vec::new(),
            ),
            (PromptKind::NewFolder, _) => (
                " new folder ".to_string(),
                "folder name:".to_string(),
                Vec::new(),
            ),
        };
        let cursor = input.len();
        self.overlay = Some(Overlay::Prompt(Prompt {
            kind,
            title,
            label,
            input,
            cursor,
        }));
    }

    fn handle_prompt_key(&mut self, key: KeyEvent) {
        enum Act {
            None,
            Cancel,
            Submit,
        }
        let mut act = Act::None;
        if let Some(Overlay::Prompt(prompt)) = self.overlay.as_mut() {
            match key.code {
                KeyCode::Esc => act = Act::Cancel,
                KeyCode::Enter => act = Act::Submit,
                KeyCode::Backspace => {
                    if prompt.cursor > 0 {
                        prompt.cursor -= 1;
                        prompt.input.remove(prompt.cursor);
                    }
                }
                KeyCode::Delete => {
                    if prompt.cursor < prompt.input.len() {
                        prompt.input.remove(prompt.cursor);
                    }
                }
                KeyCode::Left => prompt.cursor = prompt.cursor.saturating_sub(1),
                KeyCode::Right => prompt.cursor = (prompt.cursor + 1).min(prompt.input.len()),
                KeyCode::Home => prompt.cursor = 0,
                KeyCode::End => prompt.cursor = prompt.input.len(),
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    prompt.input.insert(prompt.cursor, c);
                    prompt.cursor += 1;
                }
                _ => {}
            }
        }
        match act {
            Act::None => {}
            Act::Cancel => self.overlay = None,
            Act::Submit => self.submit_prompt(),
        }
    }

    fn submit_prompt(&mut self) {
        let Some(Overlay::Prompt(prompt)) = self.overlay.take() else {
            return;
        };
        let text: String = prompt.input.iter().collect();
        let text = text.trim().to_string();
        if text.is_empty() {
            self.set_status("empty input. nothing to do.".to_string());
            return;
        }
        match prompt.kind {
            PromptKind::Rename => self.do_rename(text),
            PromptKind::OpenWith => self.open_with_command(&text),
            PromptKind::NewFile => self.do_new(text, false),
            PromptKind::NewFolder => self.do_new(text, true),
        }
    }

    fn start_confirm(&mut self) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        let what = if entry.is_dir && !entry.is_symlink {
            "directory"
        } else {
            "file"
        };
        self.overlay = Some(Overlay::Confirm(Confirm {
            kind: ConfirmKind::Delete,
            title: " delete ".to_string(),
            message: format!("delete {what} \"{}\"? cannot be undone.", entry.name),
        }));
    }

    fn handle_confirm_key(&mut self, key: KeyEvent) {
        // only an explicit 'y' destroys anything; everything else is "no".
        let yes = match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => true,
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Enter => false,
            _ => return,
        };
        let Some(Overlay::Confirm(confirm)) = self.overlay.take() else {
            return;
        };
        if yes {
            match confirm.kind {
                ConfirmKind::Delete => self.delete_selected(),
            }
        } else {
            self.set_status("no harm done.".to_string());
        }
    }

    // ---- file operations -------------------------------------------------

    fn open_selected(&mut self) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        if entry.is_dir {
            self.chdir(entry.path);
        } else {
            self.launch("xdg-open", &[], &entry.path);
        }
    }

    fn open_with_program(&mut self, program: &str) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        self.launch(program, &[], &entry.path);
    }

    fn open_with_command(&mut self, command: &str) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        let mut parts = command.split_whitespace();
        let Some(program) = parts.next() else {
            return;
        };
        let args: Vec<String> = parts.map(str::to_string).collect();
        self.launch(program, &args, &entry.path);
    }

    fn launch(&mut self, program: &str, args: &[String], path: &Path) {
        let result = Command::new(program)
            .args(args)
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        match result {
            Ok(_) => {
                let msg = format!("launched {} on {}.", program, path.display());
                self.set_status(msg);
            }
            Err(err) => {
                let msg = format!("couldn't launch {program}: {err}");
                self.set_status(msg);
            }
        }
    }

    fn do_rename(&mut self, new_name: String) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        if new_name.contains('/') {
            self.set_status("names can't contain '/'.".to_string());
            return;
        }
        let Some(parent) = entry.path.parent().map(Path::to_path_buf) else {
            return;
        };
        let dest = parent.join(&new_name);
        if dest == entry.path {
            self.set_status("same name. nothing changed.".to_string());
            return;
        }
        if dest.exists() {
            self.set_status(format!("\"{new_name}\" already exists."));
            return;
        }
        match std::fs::rename(&entry.path, &dest) {
            Ok(_) => {
                self.set_status(format!("renamed to \"{new_name}\"."));
                self.refresh_keep(&dest);
            }
            Err(err) => self.set_status(format!("rename failed: {err}")),
        }
    }

    fn delete_selected(&mut self) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        let result = if entry.is_dir && !entry.is_symlink {
            std::fs::remove_dir_all(&entry.path)
        } else {
            std::fs::remove_file(&entry.path)
        };
        match result {
            Ok(_) => {
                self.set_status(format!("deleted \"{}\". gone.", entry.name));
                let hidden = self.settings.show_hidden;
                self.tab_mut().refresh(hidden);
            }
            Err(err) => self.set_status(format!("delete failed: {err}")),
        }
    }

    fn set_clipboard(&mut self, cut: bool) {
        let Some(entry) = self.tab().selected().cloned() else {
            return;
        };
        let verb = if cut { "cut" } else { "copied" };
        self.clipboard = Some(Clipboard {
            path: entry.path.clone(),
            cut,
        });
        let msg = format!("{verb} \"{}\". paste it from the more menu.", entry.name);
        self.set_status(msg);
    }

    fn paste(&mut self) {
        let Some(clip) = self.clipboard.clone() else {
            self.set_status("clipboard is empty. copy or cut something first.".to_string());
            return;
        };
        let dir = self.tab().cwd.clone();
        let Some(name) = clip
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
        else {
            self.set_status("can't paste that.".to_string());
            return;
        };
        let dest = fsops::unique_path(&dir, &name);
        let result = if clip.cut {
            fsops::move_path(&clip.path, &dest)
        } else {
            fsops::copy_path(&clip.path, &dest)
        };
        match result {
            Ok(_) => {
                let verb = if clip.cut { "moved" } else { "copied" };
                self.set_status(format!("{verb} \"{name}\" here."));
                if clip.cut {
                    self.clipboard = None;
                }
                self.refresh_keep(&dest);
            }
            Err(err) => self.set_status(format!("paste failed: {err}")),
        }
    }

    /// Create an empty file or folder named `name` in the current directory.
    fn do_new(&mut self, name: String, folder: bool) {
        if name.contains('/') {
            self.set_status("names can't contain '/'.".to_string());
            return;
        }
        let what = if folder { "folder" } else { "file" };
        let path = self.tab().cwd.join(&name);
        if path.exists() {
            self.set_status(format!("\"{name}\" already exists."));
            return;
        }
        let result = if folder {
            std::fs::create_dir(&path)
        } else {
            // create_new: never clobber something that appeared since the check.
            std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .map(|_| ())
        };
        match result {
            Ok(_) => {
                let msg = if name.starts_with('.') && !self.settings.show_hidden {
                    format!("created {what} \"{name}\". it's hidden, press . to see it.")
                } else {
                    format!("created {what} \"{name}\".")
                };
                self.set_status(msg);
                self.refresh_keep(&path);
            }
            Err(err) => self.set_status(format!("couldn't create {what}: {err}")),
        }
    }

    /// Refresh entries, then park the cursor on `path` when it still exists.
    fn refresh_keep(&mut self, path: &Path) {
        let hidden = self.settings.show_hidden;
        self.tab_mut().refresh(hidden);
        let tab = self.tab_mut();
        if let Some(i) = tab.entries.iter().position(|e| e.path == path) {
            tab.cursor = i;
        }
    }

    fn go_parent(&mut self) {
        if let Some(parent) = self.tab().cwd.parent().map(Path::to_path_buf) {
            self.chdir(parent);
        } else {
            self.set_status("this is / . above here is just vibes.".to_string());
        }
    }

    fn goto_place(&mut self) {
        if let Some(place) = self.sidebar.get(self.sidebar_cursor) {
            let path = place.path.clone();
            self.chdir(path);
        }
    }

    fn chdir(&mut self, path: PathBuf) {
        if !path.is_dir() {
            let msg = format!("{} is not a directory.", path.display());
            self.set_status(msg);
            return;
        }
        let hidden = self.settings.show_hidden;
        let tab = self.tab_mut();
        tab.cwd = path;
        tab.cursor = 0;
        tab.refresh(hidden);
    }
}

fn build_sidebar(home: &str) -> Vec<Place> {
    let home = PathBuf::from(home);
    let candidates = [
        ("home", home.clone()),
        ("root", PathBuf::from("/")),
        ("downloads", home.join("Downloads")),
        ("documents", home.join("Documents")),
        ("projects", home.join("Projects")),
        ("dvds", home.join("dvds")),
        ("media", PathBuf::from("/run/media").join(user_name())),
    ];
    candidates
        .into_iter()
        .filter(|(_, p)| p.is_dir())
        .map(|(name, path)| Place {
            name: name.to_string(),
            path,
        })
        .collect()
}

fn user_name() -> String {
    env::var("USER").unwrap_or_else(|_| "user".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn scratch(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("lazyfm-app-{name}-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn select(app: &mut App, name: &str) {
        let i = app
            .tab()
            .entries
            .iter()
            .position(|e| e.name == name)
            .unwrap_or_else(|| panic!("no entry {name}"));
        app.tab_mut().cursor = i;
    }

    fn menu(app: &App) -> &Menu {
        match &app.overlay {
            Some(Overlay::Menu(m)) => m,
            _ => panic!("expected a menu"),
        }
    }

    #[test]
    fn enter_on_a_file_opens_the_menu() {
        let dir = scratch("enter");
        std::fs::write(dir.join("note.txt"), b"hi").unwrap();
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        select(&mut app, "note.txt");

        app.handle_key(key(KeyCode::Enter));
        assert_eq!(menu(&app).items.len(), 8);
        assert!(menu(&app).parent.is_none());

        // j past "open"/"open with" onto "rename"; choosing it swaps the menu
        // for a prefilled prompt.
        app.handle_key(key(KeyCode::Char('j')));
        app.handle_key(key(KeyCode::Char('j')));
        app.handle_key(key(KeyCode::Enter));
        match &app.overlay {
            Some(Overlay::Prompt(p)) => assert_eq!(p.input.iter().collect::<String>(), "note.txt"),
            _ => panic!("expected a rename prompt"),
        }

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn enter_on_a_directory_still_navigates() {
        let dir = scratch("nav");
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        select(&mut app, "sub");

        app.handle_key(key(KeyCode::Enter));
        assert!(app.overlay.is_none());
        assert_eq!(app.tab().cwd, dir.join("sub"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn submenu_pops_back_to_its_parent_then_closes() {
        let dir = scratch("sub");
        std::fs::write(dir.join("f.txt"), b"x").unwrap();
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        select(&mut app, "f.txt");

        app.handle_key(key(KeyCode::Char('m')));
        assert!(menu(&app).parent.is_none());
        app.handle_key(key(KeyCode::Char('j'))); // "open with..."
        app.handle_key(key(KeyCode::Enter));
        assert!(menu(&app).parent.is_some());

        app.handle_key(key(KeyCode::Esc)); // back to the main menu
        assert!(menu(&app).parent.is_none());
        app.handle_key(key(KeyCode::Esc)); // and gone
        assert!(app.overlay.is_none());

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn copy_then_paste_duplicates_a_file() {
        let dir = scratch("paste");
        std::fs::write(dir.join("a.txt"), b"data").unwrap();
        std::fs::create_dir_all(dir.join("dest")).unwrap();
        std::fs::write(dir.join("dest/keep.txt"), b"k").unwrap();
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        select(&mut app, "a.txt");

        // m -> down to "copy" -> enter.
        app.handle_key(key(KeyCode::Char('m')));
        for _ in 0..4 {
            app.handle_key(key(KeyCode::Char('j')));
        }
        app.handle_key(key(KeyCode::Enter));
        assert!(app.clipboard.is_some());

        // into dest, then m -> down to "more..." -> paste.
        app.chdir(dir.join("dest"));
        select(&mut app, "keep.txt");
        app.handle_key(key(KeyCode::Char('m')));
        for _ in 0..7 {
            app.handle_key(key(KeyCode::Char('j')));
        }
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Enter));
        assert_eq!(std::fs::read(dir.join("dest/a.txt")).unwrap(), b"data");
        assert_eq!(std::fs::read(dir.join("a.txt")).unwrap(), b"data");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn type_text(app: &mut App, text: &str) {
        for c in text.chars() {
            app.handle_key(key(KeyCode::Char(c)));
        }
    }

    #[test]
    fn new_file_and_folder_from_the_file_menu() {
        let dir = scratch("new");
        std::fs::write(dir.join("a.txt"), b"x").unwrap();
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        select(&mut app, "a.txt");

        // m -> down to "new..." -> "file..." -> type a name.
        app.handle_key(key(KeyCode::Char('m')));
        for _ in 0..6 {
            app.handle_key(key(KeyCode::Char('j')));
        }
        app.handle_key(key(KeyCode::Enter));
        assert_eq!(menu(&app).items.len(), 2);
        assert!(menu(&app).parent.is_some());
        app.handle_key(key(KeyCode::Enter));
        type_text(&mut app, "b.txt");
        app.handle_key(key(KeyCode::Enter));
        assert!(dir.join("b.txt").is_file());
        assert_eq!(app.tab().selected().unwrap().name, "b.txt");

        // same again, but "folder...".
        app.handle_key(key(KeyCode::Char('m')));
        for _ in 0..6 {
            app.handle_key(key(KeyCode::Char('j')));
        }
        app.handle_key(key(KeyCode::Enter));
        app.handle_key(key(KeyCode::Char('j')));
        app.handle_key(key(KeyCode::Enter));
        type_text(&mut app, "sub");
        app.handle_key(key(KeyCode::Enter));
        assert!(dir.join("sub").is_dir());

        // an existing name is refused, not overwritten.
        std::fs::write(dir.join("b.txt"), b"keep").unwrap();
        app.do_new("b.txt".to_string(), false);
        assert_eq!(std::fs::read(dir.join("b.txt")).unwrap(), b"keep");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn menu_on_parent_entry_can_create_in_an_empty_folder() {
        let dir = scratch("empty");
        let mut app = App::new().unwrap();
        app.chdir(dir.clone());
        assert_eq!(app.tab().selected().unwrap().name, "..");

        app.handle_key(key(KeyCode::Char('m')));
        assert_eq!(menu(&app).items.len(), 3);
        app.handle_key(key(KeyCode::Enter)); // "new file..."
        type_text(&mut app, "first.txt");
        app.handle_key(key(KeyCode::Enter));
        assert!(dir.join("first.txt").is_file());

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
