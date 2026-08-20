//! Runtime localization for the Grok Build TUI.
//!
//! The catalog deliberately starts small and grows with each localization
//! iteration. User-facing strings should be moved here instead of being
//! translated inline in view code.

use std::borrow::Cow;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Russian,
}

impl Language {
    pub fn detect() -> Self {
        let explicit = env::var("GROK_LANGUAGE")
            .ok()
            .or_else(|| env::var("GROK_LANG").ok());

        match explicit.as_deref().map(str::trim).map(str::to_ascii_lowercase).as_deref() {
            Some("ru") | Some("ru-ru") | Some("russian") | Some("русский") => Self::Russian,
            Some("en") | Some("en-us") | Some("en-gb") | Some("english") => Self::English,
            Some(_) => Self::English,
            None => Self::from_system_locale(),
        }
    }

    fn from_system_locale() -> Self {
        let locale = env::var("LC_ALL")
            .ok()
            .or_else(|| env::var("LC_MESSAGES").ok())
            .or_else(|| env::var("LANG").ok());

        match locale.as_deref() {
            Some(value)
                if value.to_ascii_lowercase().starts_with("ru") => Self::Russian,
            _ => Self::English,
        }
    }
}

static LANGUAGE: OnceLock<Language> = OnceLock::new();

pub fn language() -> Language {
    *LANGUAGE.get_or_init(Language::detect)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextKey {
    AuthBrowserHeader,
    AuthDeviceHeader,
    AuthDeviceCaption,
    AuthCopyPrefix,
    AuthCopyHere,
    AuthCopySuffix,
    AuthFallback,
    ClipboardCopied,
    ClipboardCopyUnverified,
    ClipboardCopyFailed,
}

pub fn tr(key: TextKey) -> Cow<'static, str> {
    let text = match (language(), key) {
        (Language::English, TextKey::AuthBrowserHeader) => "A browser window will open for authentication.",
        (Language::Russian, TextKey::AuthBrowserHeader) => "Для авторизации откроется окно браузера.",

        (Language::English, TextKey::AuthDeviceHeader) => {
            "Approve in your browser to finish signing in."
        }
        (Language::Russian, TextKey::AuthDeviceHeader) => {
            "Подтвердите вход в браузере, чтобы завершить авторизацию."
        }

        (Language::English, TextKey::AuthDeviceCaption) => {
            "Make sure your browser shows this code."
        }
        (Language::Russian, TextKey::AuthDeviceCaption) => {
            "Убедитесь, что в браузере отображается этот код."
        }

        (Language::English, TextKey::AuthCopyPrefix) => "If it doesn't open, click ",
        (Language::Russian, TextKey::AuthCopyPrefix) => "Если окно не открылось, нажмите ",
        (Language::English, TextKey::AuthCopyHere) => "here",
        (Language::Russian, TextKey::AuthCopyHere) => "здесь",
        (Language::English, TextKey::AuthCopySuffix) => " to copy.",
        (Language::Russian, TextKey::AuthCopySuffix) => ", чтобы скопировать ссылку.",

        (Language::English, TextKey::AuthFallback) => {
            "Copying not working? Click here to show full URL."
        }
        (Language::Russian, TextKey::AuthFallback) => {
            "Не удаётся скопировать? Нажмите здесь, чтобы показать полный URL."
        }

        (Language::English, TextKey::ClipboardCopied) => "copied!",
        (Language::Russian, TextKey::ClipboardCopied) => "скопировано!",
        (Language::English, TextKey::ClipboardCopyUnverified) => "copy sent—verify paste",
        (Language::Russian, TextKey::ClipboardCopyUnverified) => {
            "команда копирования отправлена — проверьте вставку"
        }
        (Language::English, TextKey::ClipboardCopyFailed) => "copy failed",
        (Language::Russian, TextKey::ClipboardCopyFailed) => "не удалось скопировать",
    };

    Cow::Borrowed(text)
}

pub fn language_name() -> &'static str {
    match language() {
        Language::English => "English",
        Language::Russian => "Русский",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn russian_catalog_is_non_empty() {
        let values = [
            TextKey::AuthBrowserHeader,
            TextKey::AuthDeviceHeader,
            TextKey::AuthDeviceCaption,
            TextKey::AuthCopyPrefix,
            TextKey::AuthCopyHere,
            TextKey::AuthCopySuffix,
            TextKey::AuthFallback,
            TextKey::ClipboardCopied,
            TextKey::ClipboardCopyUnverified,
            TextKey::ClipboardCopyFailed,
        ];

        for key in values {
            assert!(!tr(key).is_empty());
        }
    }
}
