//! Key shortcut types and the `key!()` macro.

use std::fmt;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// A key + modifiers pair for matching against crossterm events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyShortcut {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyShortcut {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }.normalize_case()
    }

    pub fn key(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::NONE)
    }

    pub fn ctrl(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::CONTROL)
    }

    fn normalize_case(mut self) -> Self {
        let c = match self.code {
            KeyCode::Char(c) => c,
            _ => return self,
        };
        if c.is_ascii_uppercase() {
            self.modifiers.insert(KeyModifiers::SHIFT);
        } else if self.modifiers.contains(KeyModifiers::SHIFT) && c.is_ascii_lowercase() {
            self.code = KeyCode::Char(c.to_ascii_uppercase());
        }
        self
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        if event.kind == KeyEventKind::Release {
            return false;
        }
        let normalized = Self::new(event.code, event.modifiers);
        if self.code == normalized.code && self.modifiers == normalized.modifiers {
            return true;
        }
        // Support standard Russian keyboard layout: map Cyrillic character to Latin equivalent.
        if let KeyCode::Char(c) = event.code
            && let Some(mapped) = cyrillic_to_latin(c)
        {
            let mapped_normalized = Self::new(KeyCode::Char(mapped), event.modifiers);
            if self.code == mapped_normalized.code && self.modifiers == mapped_normalized.modifiers {
                return true;
            }
        }
        false
    }

    pub fn to_key_event(self) -> KeyEvent {
        KeyEvent::new(self.code, self.modifiers)
    }

    pub fn display(&self) -> String {
        self.to_string()
    }

    pub fn modifiers_prefix(&self) -> String {
        let mut s = String::new();
        let _ = self.write_modifiers_prefix(&mut s);
        s
    }

    pub fn code_display(&self) -> String {
        let mut s = String::new();
        let _ = self.write_code_display(&mut s);
        s
    }

    fn write_modifiers_prefix(&self, f: &mut impl fmt::Write) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::SUPER) {
            f.write_str(if cfg!(target_os = "macos") { "Cmd+" } else { "Super+" })?;
        }
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            f.write_str("Ctrl+")?;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            f.write_str(if cfg!(target_os = "macos") { "Opt+" } else { "Alt+" })?;
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            f.write_str("Shift+")?;
        }
        Ok(())
    }

    fn write_code_display(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self.code {
            KeyCode::Char(' ') => f.write_str("Space"),
            KeyCode::Char(c) => write!(f, "{}", c.to_ascii_lowercase()),
            KeyCode::Enter => f.write_str("Enter"),
            KeyCode::Esc => f.write_str("Esc"),
            KeyCode::Tab => f.write_str("Tab"),
            KeyCode::BackTab => f.write_str("Shift+Tab"),
            KeyCode::Backspace => f.write_str("Bsp"),
            KeyCode::Delete => f.write_str("Del"),
            KeyCode::Up => f.write_str("↑"),
            KeyCode::Down => f.write_str("↓"),
            KeyCode::Left => f.write_str("←"),
            KeyCode::Right => f.write_str("→"),
            KeyCode::Home => f.write_str("Home"),
            KeyCode::End => f.write_str("End"),
            KeyCode::PageUp => f.write_str("PgUp"),
            KeyCode::PageDown => f.write_str("PgDn"),
            KeyCode::F(n) => write!(f, "F{n}"),
            other => write!(f, "{other:?}"),
        }
    }

    pub fn display_pretty(&self) -> String {
        self.to_string()
    }

    pub fn display_telemetry(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.contains(KeyModifiers::SUPER) {
            parts.push("Super".to_string());
        }
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt".to_string());
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift".to_string());
        }
        let code = match self.code {
            KeyCode::Char(' ') => "Space".to_string(),
            KeyCode::Char(c) => c.to_ascii_uppercase().to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab | KeyCode::BackTab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::F(n) => format!("F{n}"),
            other => format!("{other:?}"),
        };
        parts.push(code);
        parts.join("+")
    }

    pub fn is_letter_or_shift_letter(&self) -> bool {
        let KeyCode::Char(c) = self.code else { return false };
        if !c.is_ascii_alphabetic() { return false; }
        self.modifiers.is_empty() || self.modifiers == KeyModifiers::SHIFT
    }
}

/// Map a Cyrillic character from the standard Russian JCUKEN / QWERTY layout
/// to its corresponding Latin key character on the physical keyboard.
pub fn cyrillic_to_latin(c: char) -> Option<char> {
    let lower = c.to_lowercase().next()?;
    let mapped = match lower {
        'й' => 'q', 'ц' => 'w', 'у' => 'e', 'к' => 'r', 'е' => 't', 'н' => 'y',
        'г' => 'u', 'ш' => 'i', 'щ' => 'o', 'з' => 'p', 'х' => '[', 'ъ' => ']',
        'ф' => 'a', 'ы' => 's', 'в' => 'd', 'а' => 'f', 'п' => 'g', 'р' => 'h',
        'о' => 'j', 'л' => 'k', 'д' => 'l', 'ж' => ';', 'э' => '\'',
        'я' => 'z', 'ч' => 'x', 'с' => 'c', 'м' => 'v', 'и' => 'b', 'т' => 'n',
        'ь' => 'm', 'б' => ',', 'ю' => '.', 'ё' => '`',
        _ => return None,
    };
    if c.is_uppercase() {
        Some(mapped.to_ascii_uppercase())
    } else {
        Some(mapped)
    }
}

pub fn is_paste_key(key: &KeyEvent) -> bool {
    key!('v', CONTROL).matches(key)
        || key!('v', SUPER).matches(key)
        || {
            #[cfg(target_os = "windows")]
            {
                key!('v', ALT).matches(key)
            }
            #[cfg(not(target_os = "windows"))]
            {
                false
            }
        }
}

pub fn is_inline_paste_key(key: &KeyEvent) -> bool {
    key!('v', CONTROL | SHIFT).matches(key)
        || key!('v', SUPER | SHIFT).matches(key)
}

/// Ctrl+Z / Cmd+Z — the textarea's undo binding.
pub fn is_undo_key(key: &KeyEvent) -> bool {
    xai_ratatui_textarea::is_undo_input(key)
}

#[cfg(target_os = "windows")]
#[inline]
pub fn is_altgr(modifiers: KeyModifiers) -> bool {
    modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT)
}

#[cfg(not(target_os = "windows"))]
#[inline]
pub fn is_altgr(_modifiers: KeyModifiers) -> bool { false }

pub fn shift_tab_keys() -> [KeyShortcut; 3] {
    [
        KeyShortcut::key(KeyCode::BackTab),
        KeyShortcut::new(KeyCode::BackTab, KeyModifiers::SHIFT),
        KeyShortcut::new(KeyCode::Tab, KeyModifiers::SHIFT),
    ]
}

pub fn is_shift_tab(key: &KeyEvent) -> bool {
    shift_tab_keys().iter().any(|k| k.matches(key))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowWalk { Forward, Backward }

impl RowWalk {
    pub fn from_key(key: &KeyEvent) -> Option<Self> {
        if is_shift_tab(key) {
            Some(Self::Backward)
        } else if KeyShortcut::key(KeyCode::Tab).matches(key) {
            Some(Self::Forward)
        } else {
            None
        }
    }

    #[must_use]
    pub fn step(self, idx: usize, len: usize) -> usize {
        let Some(last) = len.checked_sub(1) else { return 0 };
        let cur = idx.min(last);
        match self {
            Self::Forward => if cur == last { 0 } else { cur + 1 },
            Self::Backward => if cur == 0 { last } else { cur - 1 },
        }
    }
}

pub fn is_text_input_key(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char(_))
        && (key.modifiers.is_empty()
            || key.modifiers == KeyModifiers::SHIFT
            || is_altgr(key.modifiers))
}

impl From<KeyEvent> for KeyShortcut {
    fn from(key: KeyEvent) -> Self { Self::new(key.code, key.modifiers) }
}

impl fmt::Display for KeyShortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_modifiers_prefix(f)?;
        self.write_code_display(f)
    }
}

#[macro_export]
macro_rules! key {
    ($char:literal $(, $($mod:ident)|+)? $(,)?) => {
        $crate::input::key::KeyShortcut::new(
            ::crossterm::event::KeyCode::Char($char),
            ::crossterm::event::KeyModifiers::NONE
                $($(| ::crossterm::event::KeyModifiers::$mod)+)?,
        )
    };
    ($code:ident $(, $($mod:ident)|+)? $(,)?) => {
        $crate::input::key::KeyShortcut::new(
            ::crossterm::event::KeyCode::$code,
            ::crossterm::event::KeyModifiers::NONE
                $($(| ::crossterm::event::KeyModifiers::$mod)+)?,
        )
    };
    ($code:ident ($($arg:tt)*) $(, $($mod:ident)|+)? $(,)?) => {
        $crate::input::key::KeyShortcut::new(
            ::crossterm::event::KeyCode::$code($($arg)*),
            ::crossterm::event::KeyModifiers::NONE
                $($(| ::crossterm::event::KeyModifiers::$mod)+)?,
        )
    };
}

pub use key;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn russian_ctrl_v_is_paste() {
        assert!(is_paste_key(&KeyEvent::new(KeyCode::Char('м'), KeyModifiers::CONTROL)));
        assert!(is_paste_key(&KeyEvent::new(KeyCode::Char('М'), KeyModifiers::CONTROL)));
    }

    #[test]
    fn russian_ctrl_shift_v_is_inline_paste() {
        assert!(is_inline_paste_key(&KeyEvent::new(
            KeyCode::Char('м'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        )));
    }

    #[test]
    fn unmodified_russian_m_is_not_paste() {
        assert!(!is_paste_key(&KeyEvent::new(KeyCode::Char('м'), KeyModifiers::NONE)));
    }

    #[test]
    fn altgr_russian_m_is_not_paste() {
        assert!(!is_paste_key(&KeyEvent::new(
            KeyCode::Char('м'),
            KeyModifiers::CONTROL | KeyModifiers::ALT,
        )));
    }

    #[test]
    fn russian_shortcuts_match_all_ctrl_chords() {
        // Ctrl+C (С)
        assert!(key!('c', CONTROL).matches(&KeyEvent::new(KeyCode::Char('с'), KeyModifiers::CONTROL)));
        assert!(key!('c', CONTROL).matches(&KeyEvent::new(KeyCode::Char('С'), KeyModifiers::CONTROL)));

        // Ctrl+O (Щ)
        assert!(key!('o', CONTROL).matches(&KeyEvent::new(KeyCode::Char('щ'), KeyModifiers::CONTROL)));

        // Ctrl+G (П)
        assert!(key!('g', CONTROL).matches(&KeyEvent::new(KeyCode::Char('п'), KeyModifiers::CONTROL)));

        // Ctrl+Z (Я)
        assert!(key!('z', CONTROL).matches(&KeyEvent::new(KeyCode::Char('я'), KeyModifiers::CONTROL)));

        // Alt+X (Ч)
        assert!(key!('x', ALT).matches(&KeyEvent::new(KeyCode::Char('ч'), KeyModifiers::ALT)));

        // Ctrl+Shift+N (Т)
        assert!(key!('n', CONTROL | SHIFT).matches(&KeyEvent::new(KeyCode::Char('т'), KeyModifiers::CONTROL | KeyModifiers::SHIFT)));
    }
}
