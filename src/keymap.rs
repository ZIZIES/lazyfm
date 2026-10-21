use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;

pub fn map(key: KeyEvent) -> Option<Action> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl {
        return match key.code {
            KeyCode::Char('c') => Some(Action::Quit),
            KeyCode::Char('d') => Some(Action::PageDown),
            KeyCode::Char('u') => Some(Action::PageUp),
            KeyCode::Char('t') => Some(Action::NewTab),
            _ => None,
        };
    }

    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('j') | KeyCode::Down => Some(Action::Down),
        KeyCode::Char('k') | KeyCode::Up => Some(Action::Up),
        KeyCode::Char('g') | KeyCode::Home => Some(Action::Top),
        KeyCode::Char('G') | KeyCode::End => Some(Action::Bottom),
        KeyCode::PageDown => Some(Action::PageDown),
        KeyCode::PageUp => Some(Action::PageUp),
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => Some(Action::Enter),
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => Some(Action::Parent),
        KeyCode::Char('.') => Some(Action::ToggleHidden),
        KeyCode::Char('r') => Some(Action::Refresh),
        KeyCode::Tab => Some(Action::FocusNext),
        KeyCode::BackTab => Some(Action::FocusPrev),
        KeyCode::Char('n') => Some(Action::NewTab),
        KeyCode::Char('w') => Some(Action::CloseTab),
        KeyCode::Char(']') => Some(Action::NextTab),
        KeyCode::Char('[') => Some(Action::PrevTab),
        KeyCode::Char('i') => Some(Action::CycleIconTheme),
        KeyCode::Char('?') => Some(Action::Help),
        _ => None,
    }
}
