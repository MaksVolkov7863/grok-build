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
        // Standard Unix / POSIX locale variables
        let locale = env::var("LC_ALL")
            .ok()
            .or_else(|| env::var("LC_MESSAGES").ok())
            .or_else(|| env::var("LANG").ok());

        if let Some(value) = locale.as_deref() {
            let lower = value.to_ascii_lowercase();
            if lower.starts_with("ru") || lower.contains("russian") {
                return Self::Russian;
            }
        }

        // On Windows, check Windows-specific locale indicators
        #[cfg(windows)]
        {
            if let Ok(user) = env::var("USERNAME") {
                // If username contains Cyrillic (e.g. "Никита"), it's an unmistakable Russian Windows install
                if user.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)) {
                    return Self::Russian;
                }
            }
            if let Ok(user_profile) = env::var("USERPROFILE") {
                if user_profile.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)) {
                    return Self::Russian;
                }
            }
        }

        Self::English
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

    // Turn activity & status
    Thinking,
    Responding,
    Compacting,
    Verifying,
    Cancelling,
    Running,
    Waiting,
    WaitingForResponse,
    WaitingOnSubagent,
    WaitingOnTaskOutput,
    WaitingOnTasks,
    Sleeping,
    StartingSession,
    SendToBg,
    Stop,
    PressAgainTo,

    // Tool writing status
    WritingSubagentPrompt,
    PreparingMcpTool,
    SearchingMcpTools,
    WritingFile,
    WritingEdit,
    WritingCommand,
    UpdatingTodoList,
    WritingWorkflow,
    WritingImagePrompt,
    WritingVideoPrompt,
    PreparingQuestion,
    PreparingToolCall,

    // Tool execution status prefixes
    SearchPrefix,
    FetchPrefix,
    RunPrefix,
    WaitingOnAnswersFor,
    ActionRequired,

    // Watchers
    StillRunning,
    SendAMessageToInterrupt,
    Queued,
    EnterToSendNow,

    // Welcome menu & dialogs
    NewWorktree,
    ResumeSession,
    Changelog,
    ImportClaudeSettings,
    Logout,
    UpgradeSubscription,
    Save,
    SaveAndSend,
    DiscardChanges,
    DiscardAndSend,
    DeletePrompt,
    Cancel,
    Reset,
    WelcomeHeroSubtitle,
    TypeAMessage,
    EnlargeWindowNotice,
    WindowTooSmall,
    AcceptConsent,
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

        // Turn activity & status
        (Language::English, TextKey::Thinking) => "Thinking…",
        (Language::Russian, TextKey::Thinking) => "Размышляет…",
        (Language::English, TextKey::Responding) => "Responding…",
        (Language::Russian, TextKey::Responding) => "Отвечает…",
        (Language::English, TextKey::Compacting) => "Compacting…",
        (Language::Russian, TextKey::Compacting) => "Сжатие контекста…",
        (Language::English, TextKey::Verifying) => "Verifying…",
        (Language::Russian, TextKey::Verifying) => "Проверка…",
        (Language::English, TextKey::Cancelling) => "Cancelling…",
        (Language::Russian, TextKey::Cancelling) => "Отмена…",
        (Language::English, TextKey::Running) => "Running…",
        (Language::Russian, TextKey::Running) => "Выполнение…",
        (Language::English, TextKey::Waiting) => "Waiting…",
        (Language::Russian, TextKey::Waiting) => "Ожидание…",
        (Language::English, TextKey::WaitingForResponse) => "Waiting for response…",
        (Language::Russian, TextKey::WaitingForResponse) => "Ожидание ответа модели…",
        (Language::English, TextKey::WaitingOnSubagent) => "Waiting on subagent…",
        (Language::Russian, TextKey::WaitingOnSubagent) => "Ожидание субагента…",
        (Language::English, TextKey::WaitingOnTaskOutput) => "Waiting on task output…",
        (Language::Russian, TextKey::WaitingOnTaskOutput) => "Ожидание вывода задачи…",
        (Language::English, TextKey::WaitingOnTasks) => "Waiting on tasks…",
        (Language::Russian, TextKey::WaitingOnTasks) => "Ожидание завершения задач…",
        (Language::English, TextKey::Sleeping) => "Sleeping…",
        (Language::Russian, TextKey::Sleeping) => "Ожидание по таймеру…",
        (Language::English, TextKey::StartingSession) => "Starting session…",
        (Language::Russian, TextKey::StartingSession) => "Запуск сессии…",
        (Language::English, TextKey::SendToBg) => "send to bg",
        (Language::Russian, TextKey::SendToBg) => "в фон",
        (Language::English, TextKey::Stop) => "stop",
        (Language::Russian, TextKey::Stop) => "стоп",
        (Language::English, TextKey::PressAgainTo) => "press again to ",
        (Language::Russian, TextKey::PressAgainTo) => "нажмите ещё раз для ",

        // Tool writing status
        (Language::English, TextKey::WritingSubagentPrompt) => "Writing subagent prompt",
        (Language::Russian, TextKey::WritingSubagentPrompt) => "Формирование запроса субагенту",
        (Language::English, TextKey::PreparingMcpTool) => "Preparing MCP tool",
        (Language::Russian, TextKey::PreparingMcpTool) => "Подготовка инструмента MCP",
        (Language::English, TextKey::SearchingMcpTools) => "Searching MCP tools",
        (Language::Russian, TextKey::SearchingMcpTools) => "Поиск инструментов MCP",
        (Language::English, TextKey::WritingFile) => "Writing file",
        (Language::Russian, TextKey::WritingFile) => "Запись файла",
        (Language::English, TextKey::WritingEdit) => "Writing edit",
        (Language::Russian, TextKey::WritingEdit) => "Подготовка правок",
        (Language::English, TextKey::WritingCommand) => "Writing command",
        (Language::Russian, TextKey::WritingCommand) => "Формирование команды",
        (Language::English, TextKey::UpdatingTodoList) => "Updating todo list",
        (Language::Russian, TextKey::UpdatingTodoList) => "Обновление списка задач",
        (Language::English, TextKey::WritingWorkflow) => "Writing workflow",
        (Language::Russian, TextKey::WritingWorkflow) => "Подготовка воркфлоу",
        (Language::English, TextKey::WritingImagePrompt) => "Writing image prompt",
        (Language::Russian, TextKey::WritingImagePrompt) => "Формирование промпта для изображения",
        (Language::English, TextKey::WritingVideoPrompt) => "Writing video prompt",
        (Language::Russian, TextKey::WritingVideoPrompt) => "Формирование промпта для видео",
        (Language::English, TextKey::PreparingQuestion) => "Preparing question",
        (Language::Russian, TextKey::PreparingQuestion) => "Подготовка вопроса",
        (Language::English, TextKey::PreparingToolCall) => "Preparing tool call",
        (Language::Russian, TextKey::PreparingToolCall) => "Подготовка вызова инструмента",

        // Tool execution status prefixes
        (Language::English, TextKey::SearchPrefix) => "Search ",
        (Language::Russian, TextKey::SearchPrefix) => "Поиск: ",
        (Language::English, TextKey::FetchPrefix) => "Fetch ",
        (Language::Russian, TextKey::FetchPrefix) => "Загрузка: ",
        (Language::English, TextKey::RunPrefix) => "Run ",
        (Language::Russian, TextKey::RunPrefix) => "Запуск: ",
        (Language::English, TextKey::WaitingOnAnswersFor) => "Waiting on answers for ",
        (Language::Russian, TextKey::WaitingOnAnswersFor) => "Ожидание ответа на: ",
        (Language::English, TextKey::ActionRequired) => "Action Required",
        (Language::Russian, TextKey::ActionRequired) => "Требуется действие",

        // Watchers
        (Language::English, TextKey::StillRunning) => "still running",
        (Language::Russian, TextKey::StillRunning) => "работает в фоне",
        (Language::English, TextKey::SendAMessageToInterrupt) => "send a message to interrupt",
        (Language::Russian, TextKey::SendAMessageToInterrupt) => "отправьте сообщение для прерывания",
        (Language::English, TextKey::Queued) => "queued",
        (Language::Russian, TextKey::Queued) => "в очереди",
        (Language::English, TextKey::EnterToSendNow) => "Enter to send now",
        (Language::Russian, TextKey::EnterToSendNow) => "Enter — отправить сейчас",

        // Welcome menu & dialogs
        (Language::English, TextKey::NewWorktree) => "New worktree",
        (Language::Russian, TextKey::NewWorktree) => "Новое рабочее дерево",
        (Language::English, TextKey::ResumeSession) => "Resume session",
        (Language::Russian, TextKey::ResumeSession) => "Возобновить сессию",
        (Language::English, TextKey::Changelog) => "Changelog",
        (Language::Russian, TextKey::Changelog) => "Список изменений",
        (Language::English, TextKey::ImportClaudeSettings) => "Import Claude settings",
        (Language::Russian, TextKey::ImportClaudeSettings) => "Импортировать настройки Claude",
        (Language::English, TextKey::Logout) => "Logout",
        (Language::Russian, TextKey::Logout) => "Выйти из аккаунта",
        (Language::English, TextKey::UpgradeSubscription) => "Upgrade Subscription",
        (Language::Russian, TextKey::UpgradeSubscription) => "Улучшить подписку",
        (Language::English, TextKey::Save) => "save",
        (Language::Russian, TextKey::Save) => "сохранить",
        (Language::English, TextKey::SaveAndSend) => "save & send",
        (Language::Russian, TextKey::SaveAndSend) => "сохранить и отправить",
        (Language::English, TextKey::DiscardChanges) => "discard changes",
        (Language::Russian, TextKey::DiscardChanges) => "отменить изменения",
        (Language::English, TextKey::DiscardAndSend) => "discard & send",
        (Language::Russian, TextKey::DiscardAndSend) => "отменить и отправить",
        (Language::English, TextKey::DeletePrompt) => "delete prompt",
        (Language::Russian, TextKey::DeletePrompt) => "удалить запрос",
        (Language::English, TextKey::Cancel) => "cancel",
        (Language::Russian, TextKey::Cancel) => "отмена",
        (Language::English, TextKey::Reset) => "reset",
        (Language::Russian, TextKey::Reset) => "сбросить",

        (Language::English, TextKey::WelcomeHeroSubtitle) => {
            "Thanks for trying Grok Build, give feedback with /feedback!"
        }
        (Language::Russian, TextKey::WelcomeHeroSubtitle) => {
            "Спасибо, что используете Grok Build! Оставить отзыв: /feedback"
        }

        (Language::English, TextKey::TypeAMessage) => "Type a message...",
        (Language::Russian, TextKey::TypeAMessage) => "Введите сообщение...",

        (Language::English, TextKey::EnlargeWindowNotice) => {
            "Enlarge the window to read this notice"
        }
        (Language::Russian, TextKey::EnlargeWindowNotice) => {
            "Увеличьте окно терминала, чтобы прочитать это уведомление"
        }

        (Language::English, TextKey::WindowTooSmall) => "Window too small",
        (Language::Russian, TextKey::WindowTooSmall) => "Окно слишком мало",

        (Language::English, TextKey::AcceptConsent) => "Accept",
        (Language::Russian, TextKey::AcceptConsent) => "Принять",
    };

    Cow::Borrowed(text)
}

pub fn language_name() -> &'static str {
    match language() {
        Language::English => "English",
        Language::Russian => "Русский",
    }
}

pub fn format_count_noun(count: usize, noun_en: &str) -> String {
    match language() {
        Language::English => {
            let plural = if count == 1 { "" } else { "s" };
            format!("{count} {noun_en}{plural}")
        }
        Language::Russian => {
            let noun_ru = match noun_en {
                "command" => match russian_plural_form(count) {
                    PluralForm::One => "команда",
                    PluralForm::Few => "команды",
                    PluralForm::Many => "команд",
                },
                "monitor" => match russian_plural_form(count) {
                    PluralForm::One => "монитор",
                    PluralForm::Few => "монитора",
                    PluralForm::Many => "мониторов",
                },
                "loop" => match russian_plural_form(count) {
                    PluralForm::One => "цикл",
                    PluralForm::Few => "цикла",
                    PluralForm::Many => "циклов",
                },
                "subagent" => match russian_plural_form(count) {
                    PluralForm::One => "субагент",
                    PluralForm::Few => "субагента",
                    PluralForm::Many => "субагентов",
                },
                "workflow" => match russian_plural_form(count) {
                    PluralForm::One => "воркфлоу",
                    PluralForm::Few => "воркфлоу",
                    PluralForm::Many => "воркфлоу",
                },
                "file" => match russian_plural_form(count) {
                    PluralForm::One => "файл",
                    PluralForm::Few => "файла",
                    PluralForm::Many => "файлов",
                },
                "skill" => match russian_plural_form(count) {
                    PluralForm::One => "навык",
                    PluralForm::Few => "навыка",
                    PluralForm::Many => "навыков",
                },
                "pattern" => match russian_plural_form(count) {
                    PluralForm::One => "шаблон",
                    PluralForm::Few => "шаблона",
                    PluralForm::Many => "шаблонов",
                },
                "dir" => match russian_plural_form(count) {
                    PluralForm::One => "каталог",
                    PluralForm::Few => "каталога",
                    PluralForm::Many => "каталогов",
                },
                "website" => match russian_plural_form(count) {
                    PluralForm::One => "сайт",
                    PluralForm::Few => "сайта",
                    PluralForm::Many => "сайтов",
                },
                "memory" => match russian_plural_form(count) {
                    PluralForm::One => "запись памяти",
                    PluralForm::Few => "записи памяти",
                    PluralForm::Many => "записей памяти",
                },
                "MCP tool" => match russian_plural_form(count) {
                    PluralForm::One => "инструмент MCP",
                    PluralForm::Few => "инструмента MCP",
                    PluralForm::Many => "инструментов MCP",
                },
                "tool" => match russian_plural_form(count) {
                    PluralForm::One => "инструмент",
                    PluralForm::Few => "инструмента",
                    PluralForm::Many => "инструментов",
                },
                other => other,
            };
            format!("{count} {noun_ru}")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralForm {
    One,
    Few,
    Many,
}

pub fn russian_plural_form(n: usize) -> PluralForm {
    let n10 = n % 10;
    let n100 = n % 100;
    if n10 == 1 && n100 != 11 {
        PluralForm::One
    } else if (2..=4).contains(&n10) && !(12..=14).contains(&n100) {
        PluralForm::Few
    } else {
        PluralForm::Many
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_has_both_translations() {
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
            TextKey::Thinking,
            TextKey::Responding,
            TextKey::Compacting,
            TextKey::Verifying,
            TextKey::Cancelling,
            TextKey::Running,
            TextKey::Waiting,
            TextKey::WaitingForResponse,
            TextKey::WaitingOnSubagent,
            TextKey::WaitingOnTaskOutput,
            TextKey::WaitingOnTasks,
            TextKey::Sleeping,
            TextKey::StartingSession,
            TextKey::SendToBg,
            TextKey::Stop,
            TextKey::PressAgainTo,
            TextKey::WritingSubagentPrompt,
            TextKey::PreparingMcpTool,
            TextKey::SearchingMcpTools,
            TextKey::WritingFile,
            TextKey::WritingEdit,
            TextKey::WritingCommand,
            TextKey::UpdatingTodoList,
            TextKey::WritingWorkflow,
            TextKey::WritingImagePrompt,
            TextKey::WritingVideoPrompt,
            TextKey::PreparingQuestion,
            TextKey::PreparingToolCall,
            TextKey::SearchPrefix,
            TextKey::FetchPrefix,
            TextKey::RunPrefix,
            TextKey::WaitingOnAnswersFor,
            TextKey::ActionRequired,
            TextKey::StillRunning,
            TextKey::SendAMessageToInterrupt,
            TextKey::Queued,
            TextKey::EnterToSendNow,
            TextKey::NewWorktree,
            TextKey::ResumeSession,
            TextKey::Changelog,
            TextKey::ImportClaudeSettings,
            TextKey::Logout,
            TextKey::UpgradeSubscription,
            TextKey::Save,
            TextKey::SaveAndSend,
            TextKey::DiscardChanges,
            TextKey::DiscardAndSend,
            TextKey::DeletePrompt,
            TextKey::Cancel,
            TextKey::Reset,
        ];

        for key in values {
            assert!(!tr_for(Language::English, key).is_empty());
            assert!(!tr_for(Language::Russian, key).is_empty());
        }
    }

    #[test]
    fn russian_pluralization_tests() {
        assert_eq!(russian_plural_form(1), PluralForm::One);
        assert_eq!(russian_plural_form(21), PluralForm::One);
        assert_eq!(russian_plural_form(2), PluralForm::Few);
        assert_eq!(russian_plural_form(4), PluralForm::Few);
        assert_eq!(russian_plural_form(5), PluralForm::Many);
        assert_eq!(russian_plural_form(11), PluralForm::Many);
        assert_eq!(russian_plural_form(12), PluralForm::Many);
        assert_eq!(russian_plural_form(25), PluralForm::Many);
    }
}
