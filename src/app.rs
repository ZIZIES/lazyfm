use std::env;
use std::io;
use std::path::{Path, PathBuf};

use crossterm::event::{MouseEvent, MouseEventKind};

use crate::action::Action;
use crate::entry::{self, Entry};
use crate::icons::IconTheme;

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

pub struct App {
    pub tabs: Vec<Tab>,
    pub active: usize,
    pub focus: Focus,
    pub sidebar: Vec<Place>,
    pub sidebar_cursor: usize,
    pub settings: Settings,
    pub should_quit: bool,
    pub show_help: bool,
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
            show_help: false,
            status: String::new(),
            log: Vec::new(),
        };
        app.set_status("welcome to lazyfm. press ? if you get lost.".to_string());
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
            Action::Help => self.show_help = !self.show_help,
        }
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
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
        let target = match self.tab().selected() {
            Some(e) if e.is_dir => e.path.clone(),
            Some(e) => {
                let msg = format!("{} is a file. open it with what, exactly?", e.name);
                self.set_status(msg);
                return;
            }
            None => return,
        };
        self.chdir(target);
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
