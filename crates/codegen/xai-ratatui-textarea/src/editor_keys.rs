use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{EditCommand, Movement, WordStyle};

/// Resolve a key event into a cursor [`Movement`]; `None` for non-movement keys.
pub(crate) fn resolve_movement(event: &KeyEvent) -> Option<Movement> {
    match event.code {
        // Super matches by `contains` so a stray reported bit (META/HYPER) can't degrade the chord.
        KeyCode::Left if event.modifiers.contains(KeyModifiers::SUPER) => {
            return Some(Movement::VisualRowStart);
        }
        KeyCode::Right if event.modifiers.contains(KeyModifiers::SUPER) => {
            return Some(Movement::VisualRowEnd);
        }
        KeyCode::Home => return Some(Movement::LogicalLineStart),
        KeyCode::End => return Some(Movement::LogicalLineEnd),
        KeyCode::Up => return Some(Movement::VisualRowUp),
        KeyCode::Down => return Some(Movement::VisualRowDown),
        KeyCode::Char('p' | 'з') if event.modifiers == KeyModifiers::CONTROL => {
            return Some(Movement::VisualRowUp);
        }
        KeyCode::Char('n' | 'т') if event.modifiers == KeyModifiers::CONTROL => {
            return Some(Movement::VisualRowDown);
        }
        _ => {}
    }
    // Directional (Navigation) commands are exactly the ones with a collapse edge.
    let command = classify_key_event(event)?;
    let edge = command.selection_collapse_edge()?;
    Some(Movement::Command(command, edge))
}

pub fn classify_key_event(event: &KeyEvent) -> Option<EditCommand> {
    match event {
        // Some terminals encode Ctrl-B/Ctrl-F as bare C0 characters.
        KeyEvent {
            code: KeyCode::Char('\u{0002}'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(EditCommand::MoveGraphemeLeft),
        KeyEvent {
            code: KeyCode::Char('\u{0006}'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Some(EditCommand::MoveGraphemeRight),
        KeyEvent {
            code: KeyCode::Char('h' | 'р'),
            modifiers,
            ..
        } if *modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) => {
            Some(EditCommand::DeleteWordBackward(WordStyle::Small))
        }
        KeyEvent {
            code: KeyCode::Char('\u{0008}' | '\u{007f}'),
            ..
        } => Some(EditCommand::DeleteGraphemeBackward),
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers,
            ..
        } => Some(backspace_command(*modifiers)),
        KeyEvent {
            code: KeyCode::Delete,
            modifiers,
            ..
        } => Some(delete_command(*modifiers)),
        // Ctrl+W / Ctrl+Ц — the latter is Ctrl+W on the Russian layout.
        KeyEvent {
            code: KeyCode::Char('w' | 'ц'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::DeleteWordBackward(
            WordStyle::WhitespaceDelimited,
        )),
        KeyEvent {
            code: KeyCode::Left,
            modifiers,
            ..
        } if modifiers.intersects(KeyModifiers::ALT | KeyModifiers::CONTROL) => {
            Some(EditCommand::MoveWordLeft(WordStyle::Small))
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers,
            ..
        } if modifiers.intersects(KeyModifiers::ALT | KeyModifiers::CONTROL) => {
            Some(EditCommand::MoveWordRight(WordStyle::Small))
        }
        // Ctrl+A / Ctrl+Ф
        KeyEvent {
            code: KeyCode::Char('a' | 'ф'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::MoveLogicalLineStart),
        // Ctrl+E / Ctrl+У
        KeyEvent {
            code: KeyCode::Char('e' | 'у'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::MoveLogicalLineEnd),
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            ..
        }
        | KeyEvent {
            code: KeyCode::Char('b' | 'и'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::MoveGraphemeLeft),
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            ..
        }
        | KeyEvent {
            code: KeyCode::Char('f' | 'а'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::MoveGraphemeRight),
        KeyEvent {
            code: KeyCode::Char('b' | 'и'),
            modifiers: KeyModifiers::ALT,
            ..
        } => Some(EditCommand::MoveWordLeft(WordStyle::Small)),
        KeyEvent {
            code: KeyCode::Char('f' | 'а'),
            modifiers: KeyModifiers::ALT,
            ..
        } => Some(EditCommand::MoveWordRight(WordStyle::Small)),
        // Ctrl+U / Ctrl+Г
        KeyEvent {
            code: KeyCode::Char('u' | 'г'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::DeleteToLineStart),
        // Ctrl+K / Ctrl+Л
        KeyEvent {
            code: KeyCode::Char('k' | 'л'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::DeleteToLineEnd),
        // Ctrl+H / Ctrl+Р
        KeyEvent {
            code: KeyCode::Char('h' | 'р'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::DeleteGraphemeBackward),
        // Ctrl+D / Ctrl+В
        KeyEvent {
            code: KeyCode::Char('d' | 'в'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Some(EditCommand::DeleteGraphemeForward),
        KeyEvent {
            code: KeyCode::Char('d' | 'в'),
            modifiers,
            ..
        } if modifiers.intersects(KeyModifiers::ALT | KeyModifiers::SUPER) => {
            Some(EditCommand::DeleteWordForward(WordStyle::Small))
        }
        // Ctrl-modified command keys are layout-independent. Windows terminals can
        // report the character produced by the active keyboard layout, so accept
        // both the US key and its Russian keyboard equivalent. This is deliberately
        // limited to command chords; ordinary Russian text remains untouched.
        KeyEvent {
            code: KeyCode::Char('c' | 'с'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('v' | 'м'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('x' | 'ч'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('y' | 'н'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('z' | 'я'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('r' | 'к'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char('j' | 'о' | 'm' | 'ь'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => None,
        KeyEvent {
            code: KeyCode::Char(character),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            ..
        } if !character.is_control() => {
            let character = if event.modifiers.contains(KeyModifiers::SHIFT) {
                shifted_char(*character)
            } else {
                *character
            };
            Some(EditCommand::Insert(character))
        }
        KeyEvent {
            code: KeyCode::Char(character),
            modifiers,
            ..
        } if crate::is_altgr(*modifiers) && !character.is_control() => {
            Some(EditCommand::Insert(*character))
        }
        _ => None,
    }
}

fn shifted_char(character: char) -> char {
    if character.is_ascii_lowercase() {
        character.to_ascii_uppercase()
    } else {
        character
    }
}

fn backspace_command(modifiers: KeyModifiers) -> EditCommand {
    match modifiers {
        KeyModifiers::ALT | KeyModifiers::CONTROL => {
            EditCommand::DeleteWordBackward(WordStyle::Small)
        }
        KeyModifiers::SUPER => EditCommand::DeleteToLineStart,
        _ => EditCommand::DeleteGraphemeBackward,
    }
}

fn delete_command(modifiers: KeyModifiers) -> EditCommand {
    if modifiers.intersects(KeyModifiers::ALT | KeyModifiers::CONTROL | KeyModifiers::SUPER) {
        EditCommand::DeleteWordForward(WordStyle::Small)
    } else {
        EditCommand::DeleteGraphemeForward
    }
}
