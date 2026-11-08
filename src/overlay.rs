//! Modal overlays: the entry action menu, text prompts, and yes/no confirms.
//!
//! Overlays live on `App.overlay` and, while present, swallow all key input.
//! A menu may carry a `parent`, which turns Esc/Left into "go back one level"
//! instead of "close everything" (the open-with and more submenus rely on this).

pub enum Overlay {
    Menu(Menu),
    Prompt(Prompt),
    Confirm(Confirm),
}

#[derive(Clone, PartialEq, Eq)]
pub enum MenuAction {
    Open,
    OpenWith,
    OpenWithApp(String),
    OpenWithCustom,
    Rename,
    Delete,
    Copy,
    Cut,
    More,
    Paste,
    New,
    NewFile,
    NewFolder,
}

pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

impl MenuItem {
    pub fn new(label: impl Into<String>, action: MenuAction) -> MenuItem {
        MenuItem {
            label: label.into(),
            action,
        }
    }
}

pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
    pub cursor: usize,
    /// set for submenus: the level Esc pops back to.
    pub parent: Option<Box<Menu>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    Rename,
    OpenWith,
    NewFile,
    NewFolder,
}

pub struct Prompt {
    pub kind: PromptKind,
    pub title: String,
    pub label: String,
    pub input: Vec<char>,
    pub cursor: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmKind {
    Delete,
}

pub struct Confirm {
    pub kind: ConfirmKind,
    pub title: String,
    pub message: String,
}
