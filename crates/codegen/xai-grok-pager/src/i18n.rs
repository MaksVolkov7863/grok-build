//! Runtime localization for the Grok Build TUI.
//!
//! Iteration 1 establishes the localization foundation and the shared strings
//! used by the authentication / clipboard flow. View code should use `tr`
//! instead of embedding translations directly in UI text.

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

        match explicit
            .as_deref()
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
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
            Some(value) if value.to_ascii_lowercase().starts_with("ru") => Self::Russian,
            _ => Self::English,
        }
    }
}

static LANGUAGE: OnceLock<Language> = OnceLock::new();

/// Return the process-wide UI language.
///
/// `GROK_LANGUAGE=ru` forces Russian and `GROK_LANGUAGE=en` forces English.
/// When neither is supplied, the process locale is used as a fallback.
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
    Quit,
    GoBack,
    LoginWith,
    QuitLabel,
    SwitchAccount,
    YesProceed,
    NoQuit,
    TrustDirectory,
    SecurityWarningLine1,
    SecurityWarningLine2,
    WaitingForLogin,
    WaitingForApproval,
    SelectUrlManual,
}

/// Translate a static UI string using the current application language.
pub fn tr(key: TextKey) -> Cow<'static, str> {
    tr_for(language(), key)
}

/// Translate a static UI string for a specific language.
///
/// Kept public so tests and future settings UI can render a preview without
/// changing the process-wide language.
pub fn tr_for(language: Language, key: TextKey) -> Cow<'static, str> {
    let text = match (language, key) {
        (Language::English, TextKey::AuthBrowserHeader) => {
            "A browser window will open for authentication."
        }
        (Language::Russian, TextKey::AuthBrowserHeader) => {
            "Для авторизации откроется окно браузера."
        }

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

        (Language::English, TextKey::Quit) => "quit",
        (Language::Russian, TextKey::Quit) => "выход",
        (Language::English, TextKey::GoBack) => "go back",
        (Language::Russian, TextKey::GoBack) => "назад",
        (Language::English, TextKey::LoginWith) => "Login with",
        (Language::Russian, TextKey::LoginWith) => "Войти через",
        (Language::English, TextKey::QuitLabel) => "Quit",
        (Language::Russian, TextKey::QuitLabel) => "Выйти",
        (Language::English, TextKey::SwitchAccount) => "Switch account",
        (Language::Russian, TextKey::SwitchAccount) => "Сменить аккаунт",
        (Language::English, TextKey::YesProceed) => "Yes, proceed",
        (Language::Russian, TextKey::YesProceed) => "Да, продолжить",
        (Language::English, TextKey::NoQuit) => "No, quit",
        (Language::Russian, TextKey::NoQuit) => "Нет, выйти",
        (Language::English, TextKey::TrustDirectory) => {
            "Do you trust the contents of this directory?"
        }
        (Language::Russian, TextKey::TrustDirectory) => {
            "Вы доверяете содержимому этого каталога?"
        }
        (Language::English, TextKey::SecurityWarningLine1) => {
            "Grok Build may run or modify contents in this directory,"
        }
        (Language::Russian, TextKey::SecurityWarningLine1) => {
            "Grok Build может запускать команды и изменять содержимое этого каталога,"
        }
        (Language::English, TextKey::SecurityWarningLine2) => "posing security risks.",
        (Language::Russian, TextKey::SecurityWarningLine2) => "что создаёт угрозу безопасности.",
        (Language::English, TextKey::WaitingForLogin) => "Waiting for login to complete...",
        (Language::Russian, TextKey::WaitingForLogin) => "Ожидание завершения входа...",
        (Language::English, TextKey::WaitingForApproval) => "Waiting for approval...",
        (Language::Russian, TextKey::WaitingForApproval) => "Ожидание подтверждения...",
        (Language::English, TextKey::SelectUrlManual) => {
            "Select the URL below with your mouse and copy manually."
        }
        (Language::Russian, TextKey::SelectUrlManual) => {
            "Выделите URL ниже мышью и скопируйте его вручную."
        }
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
    fn every_iteration_one_key_has_both_translations() {
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
            TextKey::Quit,
            TextKey::GoBack,
            TextKey::LoginWith,
            TextKey::QuitLabel,
            TextKey::SwitchAccount,
            TextKey::YesProceed,
            TextKey::NoQuit,
            TextKey::TrustDirectory,
            TextKey::SecurityWarningLine1,
            TextKey::SecurityWarningLine2,
            TextKey::WaitingForLogin,
            TextKey::WaitingForApproval,
            TextKey::SelectUrlManual,
        ];

        for key in values {
            assert!(!tr_for(Language::English, key).is_empty());
            assert!(!tr_for(Language::Russian, key).is_empty());
        }
    }
}
