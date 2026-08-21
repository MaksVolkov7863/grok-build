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

    // Shortcuts & Actions
    HintNav,
    HintPage,
    HintSelect,
    HintSend,
    HintQueue,
    HintSendNow,
    HintNewline,
    HintLines,
    HintAcceptSuggestion,
    HintMode,
    HintView,
    HintCopy,
    HintCopyOutput,
    HintKill,
    HintShowDone,
    HintHideDone,
    HintDeleteRow,
    HintEdit,
    HintReorder,
    HintGo,
    HintNextPrev,
    HintOpen,
    HintExpand,
    HintCollapse,
    HintFold,
    HintTurn,
    HintTopBtm,
    HintBack,
    HintPrompt,
    HintExpandThinking,
    HintCollapseThinking,

    // Status Bar & Credits & Goals
    GoalChip,
    GoalPhaseFailed,
    GoalPhaseInterrupted,
    GoalPhaseBudget,
    GoalPhaseDone,
    GoalPhaseVerifying,
    GoalPhasePlanning,
    GoalPhaseIdle,
    GoalPhaseExecuting,
    UsageWeeklyLimit,
    UsageMonthlyLimit,
    UsageGeneral,
    NextResetPrefix,
    CreditsPrefix,
    AutoTopupPrefix,
    AutoTopupDisabled,
    MaxMonthlyTopupPrefix,
    PayAsYouGoPrefix,
    PayAsYouGoUsedOf,
    PayAsYouGoLimitLeft,
    CreditsUsedPrefix,
    CreditsLeftPrefix,

    // Modal Titles & Messages
    ModalTitleCommands,
    ModalTitleResumeSession,
    ModalTitlePickModel,
    ModalTitlePickReasoningEffort,
    ModalTitlePickTheme,
    ModalTitlePickOption,
    ModalTitleHowToGuides,
    ModalTitleKeyboardShortcuts,
    ModalTitleMemory,
    ModalTitleSettings,
    ModalTitleResetSetting,
    ModalTitleMemoryNote,
    ModalTitleUsage,
    ModalMsgSaveAndSend,
    ModalMsgSaveChanges,

    // Cancel Turn Dialog
    CancelTurnStopRunning,
    CancelTurnContinueToRun,
    CancelTurnAlwaysStop,
    CancelTurnAlwaysContinue,

    // Command Palette Sections & Labels
    PaletteSectionSession,
    PaletteNewSession,
    PaletteNewSessionWorktree,
    PaletteAgentDashboard,
    PaletteBackToHome,
    PaletteDeleteSession,
    PaletteResumeSession,
    PaletteShareSession,
    PaletteRenameSession,
    PaletteSessionInfo,
    PaletteSendFeedback,
    PaletteSectionContext,
    PaletteCompactHistory,
    PaletteContextUsage,
    PaletteViewPlan,
    PaletteMemory,
    PaletteSectionModelInput,
    PaletteSwitchModel,
    PaletteAlwaysApproveMode,
    PaletteMultilineInput,
    PaletteEditPromptExternal,
    PaletteSectionTools,
    PaletteHooks,
    PalettePlugins,
    PaletteMarketplace,
    PaletteSkills,
    PaletteMcpServers,
    PaletteManageAgents,
    PaletteSectionOther,
    PaletteSwitchTheme,
    PaletteSettings,
    PaletteKeyboardShortcuts,
    PaletteHowToGuides,
    PaletteTutorial,
    PaletteQuit,

    // Setting Categories
    SettingCategoryAppearance,
    SettingCategoryMouse,
    SettingCategoryEditor,
    SettingCategoryAgent,
    SettingCategoryPrivacy,
    SettingCategoryModels,
    SettingCategorySession,
    SettingCategoryAdvanced,

    // Settings Modal Shortcuts & UI
    SettingsToggle,
    SettingsExpand,
    SettingsSearch,
    SettingsReset,
    SettingsClose,
    SettingsTypeToFilter,
    SettingsTypeToEdit,
    SettingsCommit,
    SettingsClear,
    SettingsTry,
    SettingsRevert,
    SettingsCursor,
    SettingsStepSmall,
    SettingsStepLarge,

    // Tasks Pane & Groups
    TasksGroupWorkflows,
    TasksGroupSubagents,
    TasksGroupTasks,
    TasksGroupWatchers,
    TaskStatusRunning,
    TaskStatusQueued,
    TaskStatusStarting,
    TaskStatusDueNow,

    // Extensions Tabs
    ExtTabHooks,
    ExtTabPlugins,
    ExtTabMarketplace,
    ExtTabSkills,
    ExtTabMcpServers,

    // Plan Approval
    PlanApprovalWaiting,
    PlanApprovalNoPlan,

    // Permissions
    PermAllowOnce,
    PermAllowAlways,
    PermRejectOnce,
    PermRejectAlways,
    PermFollowupPlaceholder,

    // Questions & Surveys
    QuestionOtherPlaceholder,
    QuestionSelectOption,
    QuestionSelectOptions,
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

        // Shortcuts & Actions
        (Language::English, TextKey::HintNav) => "nav",
        (Language::Russian, TextKey::HintNav) => "навигация",
        (Language::English, TextKey::HintPage) => "page",
        (Language::Russian, TextKey::HintPage) => "страница",
        (Language::English, TextKey::HintSelect) => "select",
        (Language::Russian, TextKey::HintSelect) => "выбрать",
        (Language::English, TextKey::HintSend) => "send",
        (Language::Russian, TextKey::HintSend) => "отправить",
        (Language::English, TextKey::HintQueue) => "queue",
        (Language::Russian, TextKey::HintQueue) => "в очередь",
        (Language::English, TextKey::HintSendNow) => "send now",
        (Language::Russian, TextKey::HintSendNow) => "отправить сейчас",
        (Language::English, TextKey::HintNewline) => "newline",
        (Language::Russian, TextKey::HintNewline) => "новая строка",
        (Language::English, TextKey::HintLines) => "lines",
        (Language::Russian, TextKey::HintLines) => "строки",
        (Language::English, TextKey::HintAcceptSuggestion) => "accept suggestion",
        (Language::Russian, TextKey::HintAcceptSuggestion) => "принять вариант",
        (Language::English, TextKey::HintMode) => "mode",
        (Language::Russian, TextKey::HintMode) => "режим",
        (Language::English, TextKey::HintView) => "view",
        (Language::Russian, TextKey::HintView) => "просмотр",
        (Language::English, TextKey::HintCopy) => "copy",
        (Language::Russian, TextKey::HintCopy) => "копировать",
        (Language::English, TextKey::HintCopyOutput) => "copy output",
        (Language::Russian, TextKey::HintCopyOutput) => "копировать вывод",
        (Language::English, TextKey::HintKill) => "kill",
        (Language::Russian, TextKey::HintKill) => "завершить",
        (Language::English, TextKey::HintShowDone) => "show done",
        (Language::Russian, TextKey::HintShowDone) => "показать готовые",
        (Language::English, TextKey::HintHideDone) => "hide done",
        (Language::Russian, TextKey::HintHideDone) => "скрыть готовые",
        (Language::English, TextKey::HintDeleteRow) => "delete row",
        (Language::Russian, TextKey::HintDeleteRow) => "удалить строку",
        (Language::English, TextKey::HintEdit) => "edit",
        (Language::Russian, TextKey::HintEdit) => "изменить",
        (Language::English, TextKey::HintReorder) => "reorder",
        (Language::Russian, TextKey::HintReorder) => "порядок",
        (Language::English, TextKey::HintGo) => "go",
        (Language::Russian, TextKey::HintGo) => "перейти",
        (Language::English, TextKey::HintNextPrev) => "next/prev",
        (Language::Russian, TextKey::HintNextPrev) => "след/пред",
        (Language::English, TextKey::HintOpen) => "open",
        (Language::Russian, TextKey::HintOpen) => "открыть",
        (Language::English, TextKey::HintExpand) => "expand",
        (Language::Russian, TextKey::HintExpand) => "развернуть",
        (Language::English, TextKey::HintCollapse) => "collapse",
        (Language::Russian, TextKey::HintCollapse) => "свернуть",
        (Language::English, TextKey::HintFold) => "fold",
        (Language::Russian, TextKey::HintFold) => "свернуть",
        (Language::English, TextKey::HintTurn) => "turn",
        (Language::Russian, TextKey::HintTurn) => "шаг",
        (Language::English, TextKey::HintTopBtm) => "top/btm",
        (Language::Russian, TextKey::HintTopBtm) => "начало/конец",
        (Language::English, TextKey::HintBack) => "back",
        (Language::Russian, TextKey::HintBack) => "назад",
        (Language::English, TextKey::HintPrompt) => "prompt",
        (Language::Russian, TextKey::HintPrompt) => "поле ввода",
        (Language::English, TextKey::HintExpandThinking) => "expand thinking",
        (Language::Russian, TextKey::HintExpandThinking) => "показать ход мыслей",
        (Language::English, TextKey::HintCollapseThinking) => "collapse thinking",
        (Language::Russian, TextKey::HintCollapseThinking) => "скрыть ход мыслей",

        // Status Bar & Credits & Goals
        (Language::English, TextKey::GoalChip) => "Goal",
        (Language::Russian, TextKey::GoalChip) => "Цель",
        (Language::English, TextKey::GoalPhaseFailed) => "Failed",
        (Language::Russian, TextKey::GoalPhaseFailed) => "Ошибка",
        (Language::English, TextKey::GoalPhaseInterrupted) => "Interrupted",
        (Language::Russian, TextKey::GoalPhaseInterrupted) => "Прервано",
        (Language::English, TextKey::GoalPhaseBudget) => "Budget",
        (Language::Russian, TextKey::GoalPhaseBudget) => "Бюджет",
        (Language::English, TextKey::GoalPhaseDone) => "Done",
        (Language::Russian, TextKey::GoalPhaseDone) => "Готово",
        (Language::English, TextKey::GoalPhaseVerifying) => "Verifying",
        (Language::Russian, TextKey::GoalPhaseVerifying) => "Проверка",
        (Language::English, TextKey::GoalPhasePlanning) => "Planning",
        (Language::Russian, TextKey::GoalPhasePlanning) => "Планирование",
        (Language::English, TextKey::GoalPhaseIdle) => "Idle",
        (Language::Russian, TextKey::GoalPhaseIdle) => "Ожидание",
        (Language::English, TextKey::GoalPhaseExecuting) => "Executing",
        (Language::Russian, TextKey::GoalPhaseExecuting) => "Выполнение",

        (Language::English, TextKey::UsageWeeklyLimit) => "Weekly limit",
        (Language::Russian, TextKey::UsageWeeklyLimit) => "Недельный лимит",
        (Language::English, TextKey::UsageMonthlyLimit) => "Monthly limit",
        (Language::Russian, TextKey::UsageMonthlyLimit) => "Месячный лимит",
        (Language::English, TextKey::UsageGeneral) => "Usage",
        (Language::Russian, TextKey::UsageGeneral) => "Использование",
        (Language::English, TextKey::NextResetPrefix) => "Next reset",
        (Language::Russian, TextKey::NextResetPrefix) => "Сброс",
        (Language::English, TextKey::CreditsPrefix) => "Credits",
        (Language::Russian, TextKey::CreditsPrefix) => "Кредиты",
        (Language::English, TextKey::AutoTopupPrefix) => "Auto topup",
        (Language::Russian, TextKey::AutoTopupPrefix) => "Автопополнение",
        (Language::English, TextKey::AutoTopupDisabled) => "disabled",
        (Language::Russian, TextKey::AutoTopupDisabled) => "отключено",
        (Language::English, TextKey::MaxMonthlyTopupPrefix) => "Max monthly topup",
        (Language::Russian, TextKey::MaxMonthlyTopupPrefix) => "Макс. автопополнение в месяц",
        (Language::English, TextKey::PayAsYouGoPrefix) => "Pay-as-you-go",
        (Language::Russian, TextKey::PayAsYouGoPrefix) => "Постоплата (PAYG)",
        (Language::English, TextKey::PayAsYouGoUsedOf) => "used of",
        (Language::Russian, TextKey::PayAsYouGoUsedOf) => "израсходовано из лимита",
        (Language::English, TextKey::PayAsYouGoLimitLeft) => "Pay-as-you-go limit left",
        (Language::Russian, TextKey::PayAsYouGoLimitLeft) => "Остаток лимита постоплаты",
        (Language::English, TextKey::CreditsUsedPrefix) => "Credits used",
        (Language::Russian, TextKey::CreditsUsedPrefix) => "Использовано кредитов",
        (Language::English, TextKey::CreditsLeftPrefix) => "Credits left",
        (Language::Russian, TextKey::CreditsLeftPrefix) => "Осталось кредитов",

        // Modal Titles & Messages
        (Language::English, TextKey::ModalTitleCommands) => "Commands",
        (Language::Russian, TextKey::ModalTitleCommands) => "Команды",
        (Language::English, TextKey::ModalTitleResumeSession) => "Resume session",
        (Language::Russian, TextKey::ModalTitleResumeSession) => "Продолжить сессию",
        (Language::English, TextKey::ModalTitlePickModel) => "Pick model",
        (Language::Russian, TextKey::ModalTitlePickModel) => "Выбор модели",
        (Language::English, TextKey::ModalTitlePickReasoningEffort) => "Pick reasoning effort",
        (Language::Russian, TextKey::ModalTitlePickReasoningEffort) => "Уровень рассуждений",
        (Language::English, TextKey::ModalTitlePickTheme) => "Pick theme",
        (Language::Russian, TextKey::ModalTitlePickTheme) => "Выбор темы",
        (Language::English, TextKey::ModalTitlePickOption) => "Pick option",
        (Language::Russian, TextKey::ModalTitlePickOption) => "Выбор варианта",
        (Language::English, TextKey::ModalTitleHowToGuides) => "How-to Guides",
        (Language::Russian, TextKey::ModalTitleHowToGuides) => "Инструкции и справка",
        (Language::English, TextKey::ModalTitleKeyboardShortcuts) => "Keyboard Shortcuts",
        (Language::Russian, TextKey::ModalTitleKeyboardShortcuts) => "Горячие клавиши",
        (Language::English, TextKey::ModalTitleMemory) => "Memory",
        (Language::Russian, TextKey::ModalTitleMemory) => "Память",
        (Language::English, TextKey::ModalTitleSettings) => "Settings",
        (Language::Russian, TextKey::ModalTitleSettings) => "Настройки",
        (Language::English, TextKey::ModalTitleResetSetting) => "Reset setting?",
        (Language::Russian, TextKey::ModalTitleResetSetting) => "Сбросить настройку?",
        (Language::English, TextKey::ModalTitleMemoryNote) => "Memory Note",
        (Language::Russian, TextKey::ModalTitleMemoryNote) => "Заметка памяти",
        (Language::English, TextKey::ModalTitleUsage) => "Usage",
        (Language::Russian, TextKey::ModalTitleUsage) => "Использование",
        (Language::English, TextKey::ModalMsgSaveAndSend) => "Save and send?",
        (Language::Russian, TextKey::ModalMsgSaveAndSend) => "Сохранить и отправить?",
        (Language::English, TextKey::ModalMsgSaveChanges) => "Save changes?",
        (Language::Russian, TextKey::ModalMsgSaveChanges) => "Сохранить изменения?",

        // Cancel Turn Dialog
        (Language::English, TextKey::CancelTurnStopRunning) => "Stop running",
        (Language::Russian, TextKey::CancelTurnStopRunning) => "Остановить выполнение",
        (Language::English, TextKey::CancelTurnContinueToRun) => "Continue to run",
        (Language::Russian, TextKey::CancelTurnContinueToRun) => "Продолжить выполнение",
        (Language::English, TextKey::CancelTurnAlwaysStop) => "Always stop",
        (Language::Russian, TextKey::CancelTurnAlwaysStop) => "Всегда останавливать",
        (Language::English, TextKey::CancelTurnAlwaysContinue) => "Always continue",
        (Language::Russian, TextKey::CancelTurnAlwaysContinue) => "Всегда продолжать",

        // Command Palette Sections & Labels
        (Language::English, TextKey::PaletteSectionSession) => "Session",
        (Language::Russian, TextKey::PaletteSectionSession) => "Сессия",
        (Language::English, TextKey::PaletteNewSession) => "New Session",
        (Language::Russian, TextKey::PaletteNewSession) => "Новая сессия",
        (Language::English, TextKey::PaletteNewSessionWorktree) => "New Session in Worktree",
        (Language::Russian, TextKey::PaletteNewSessionWorktree) => "Новая сессия в Worktree",
        (Language::English, TextKey::PaletteAgentDashboard) => "Agent Dashboard",
        (Language::Russian, TextKey::PaletteAgentDashboard) => "Панель управления агентом",
        (Language::English, TextKey::PaletteBackToHome) => "Back to Home",
        (Language::Russian, TextKey::PaletteBackToHome) => "Вернуться на главную",
        (Language::English, TextKey::PaletteDeleteSession) => "Delete This Session",
        (Language::Russian, TextKey::PaletteDeleteSession) => "Удалить эту сессию",
        (Language::English, TextKey::PaletteResumeSession) => "Resume Session",
        (Language::Russian, TextKey::PaletteResumeSession) => "Продолжить сессию",
        (Language::English, TextKey::PaletteShareSession) => "Share Session",
        (Language::Russian, TextKey::PaletteShareSession) => "Поделиться сессией",
        (Language::English, TextKey::PaletteRenameSession) => "Rename Session",
        (Language::Russian, TextKey::PaletteRenameSession) => "Переименовать сессию",
        (Language::English, TextKey::PaletteSessionInfo) => "Session Info",
        (Language::Russian, TextKey::PaletteSessionInfo) => "Информация о сессии",
        (Language::English, TextKey::PaletteSendFeedback) => "Send Feedback",
        (Language::Russian, TextKey::PaletteSendFeedback) => "Отправить отзыв",
        (Language::English, TextKey::PaletteSectionContext) => "Context",
        (Language::Russian, TextKey::PaletteSectionContext) => "Контекст",
        (Language::English, TextKey::PaletteCompactHistory) => "Compact History",
        (Language::Russian, TextKey::PaletteCompactHistory) => "Сжать историю диалога",
        (Language::English, TextKey::PaletteContextUsage) => "Context Usage",
        (Language::Russian, TextKey::PaletteContextUsage) => "Использование контекста",
        (Language::English, TextKey::PaletteViewPlan) => "View Plan",
        (Language::Russian, TextKey::PaletteViewPlan) => "Посмотреть план",
        (Language::English, TextKey::PaletteMemory) => "Memory",
        (Language::Russian, TextKey::PaletteMemory) => "Память",
        (Language::English, TextKey::PaletteSectionModelInput) => "Model & Input",
        (Language::Russian, TextKey::PaletteSectionModelInput) => "Модель и ввод",
        (Language::English, TextKey::PaletteSwitchModel) => "Switch Model",
        (Language::Russian, TextKey::PaletteSwitchModel) => "Сменить модель",
        (Language::English, TextKey::PaletteAlwaysApproveMode) => "Always Approve Mode",
        (Language::Russian, TextKey::PaletteAlwaysApproveMode) => "Режим авто-одобрения",
        (Language::English, TextKey::PaletteMultilineInput) => "Multiline Input",
        (Language::Russian, TextKey::PaletteMultilineInput) => "Многострочный ввод",
        (Language::English, TextKey::PaletteEditPromptExternal) => "Edit Prompt in External Editor",
        (Language::Russian, TextKey::PaletteEditPromptExternal) => "Редактировать во внешнем редакторе",
        (Language::English, TextKey::PaletteSectionTools) => "Tools",
        (Language::Russian, TextKey::PaletteSectionTools) => "Инструменты",
        (Language::English, TextKey::PaletteHooks) => "Hooks",
        (Language::Russian, TextKey::PaletteHooks) => "Хуки (Hooks)",
        (Language::English, TextKey::PalettePlugins) => "Plugins",
        (Language::Russian, TextKey::PalettePlugins) => "Плагины (Plugins)",
        (Language::English, TextKey::PaletteMarketplace) => "Marketplace",
        (Language::Russian, TextKey::PaletteMarketplace) => "Каталог расширений (Marketplace)",
        (Language::English, TextKey::PaletteSkills) => "Skills",
        (Language::Russian, TextKey::PaletteSkills) => "Навыки (Skills)",
        (Language::English, TextKey::PaletteMcpServers) => "MCP Servers",
        (Language::Russian, TextKey::PaletteMcpServers) => "Серверы MCP",
        (Language::English, TextKey::PaletteManageAgents) => "Manage Agents",
        (Language::Russian, TextKey::PaletteManageAgents) => "Управление агентами",
        (Language::English, TextKey::PaletteSectionOther) => "Other",
        (Language::Russian, TextKey::PaletteSectionOther) => "Другое",
        (Language::English, TextKey::PaletteSwitchTheme) => "Switch Theme",
        (Language::Russian, TextKey::PaletteSwitchTheme) => "Сменить тему",
        (Language::English, TextKey::PaletteSettings) => "Settings",
        (Language::Russian, TextKey::PaletteSettings) => "Настройки",
        (Language::English, TextKey::PaletteKeyboardShortcuts) => "Keyboard Shortcuts",
        (Language::Russian, TextKey::PaletteKeyboardShortcuts) => "Горячие клавиши",
        (Language::English, TextKey::PaletteHowToGuides) => "How-to Guides",
        (Language::Russian, TextKey::PaletteHowToGuides) => "Инструкции и справка",
        (Language::English, TextKey::PaletteTutorial) => "Tutorial",
        (Language::Russian, TextKey::PaletteTutorial) => "Обучение",
        (Language::English, TextKey::PaletteQuit) => "Quit",
        (Language::Russian, TextKey::PaletteQuit) => "Выйти",

        // Setting Categories
        (Language::English, TextKey::SettingCategoryAppearance) => "Appearance",
        (Language::Russian, TextKey::SettingCategoryAppearance) => "Внешний вид",
        (Language::English, TextKey::SettingCategoryMouse) => "Mouse",
        (Language::Russian, TextKey::SettingCategoryMouse) => "Мышь",
        (Language::English, TextKey::SettingCategoryEditor) => "Editor & Input",
        (Language::Russian, TextKey::SettingCategoryEditor) => "Редактор и ввод",
        (Language::English, TextKey::SettingCategoryAgent) => "Agent & Approval",
        (Language::Russian, TextKey::SettingCategoryAgent) => "Агент и подтверждения",
        (Language::English, TextKey::SettingCategoryPrivacy) => "Privacy",
        (Language::Russian, TextKey::SettingCategoryPrivacy) => "Конфиденциальность",
        (Language::English, TextKey::SettingCategoryModels) => "Models",
        (Language::Russian, TextKey::SettingCategoryModels) => "Модели",
        (Language::English, TextKey::SettingCategorySession) => "Session",
        (Language::Russian, TextKey::SettingCategorySession) => "Сессия",
        (Language::English, TextKey::SettingCategoryAdvanced) => "Advanced",
        (Language::Russian, TextKey::SettingCategoryAdvanced) => "Дополнительно",

        // Settings Modal Shortcuts & UI
        (Language::English, TextKey::SettingsToggle) => "toggle",
        (Language::Russian, TextKey::SettingsToggle) => "переключить",
        (Language::English, TextKey::SettingsExpand) => "expand",
        (Language::Russian, TextKey::SettingsExpand) => "развернуть",
        (Language::English, TextKey::SettingsSearch) => "search",
        (Language::Russian, TextKey::SettingsSearch) => "поиск",
        (Language::English, TextKey::SettingsReset) => "reset",
        (Language::Russian, TextKey::SettingsReset) => "сбросить",
        (Language::English, TextKey::SettingsClose) => "close",
        (Language::Russian, TextKey::SettingsClose) => "закрыть",
        (Language::English, TextKey::SettingsTypeToFilter) => "type to filter",
        (Language::Russian, TextKey::SettingsTypeToFilter) => "ввод для фильтра",
        (Language::English, TextKey::SettingsTypeToEdit) => "type to edit",
        (Language::Russian, TextKey::SettingsTypeToEdit) => "ввод для изменения",
        (Language::English, TextKey::SettingsCommit) => "commit",
        (Language::Russian, TextKey::SettingsCommit) => "применить",
        (Language::English, TextKey::SettingsClear) => "clear",
        (Language::Russian, TextKey::SettingsClear) => "очистить",
        (Language::English, TextKey::SettingsTry) => "try",
        (Language::Russian, TextKey::SettingsTry) => "предпросмотр",
        (Language::English, TextKey::SettingsRevert) => "revert",
        (Language::Russian, TextKey::SettingsRevert) => "вернуть",
        (Language::English, TextKey::SettingsCursor) => "cursor",
        (Language::Russian, TextKey::SettingsCursor) => "курсор",
        (Language::English, TextKey::SettingsStepSmall) => "±1/±10 step",
        (Language::Russian, TextKey::SettingsStepSmall) => "шаг ±1/±10",
        (Language::English, TextKey::SettingsStepLarge) => "±100/±1000 step",
        (Language::Russian, TextKey::SettingsStepLarge) => "шаг ±100/±1000",

        // Tasks Pane & Groups
        (Language::English, TextKey::TasksGroupWorkflows) => "Workflows",
        (Language::Russian, TextKey::TasksGroupWorkflows) => "Рабочие процессы",
        (Language::English, TextKey::TasksGroupSubagents) => "Subagents",
        (Language::Russian, TextKey::TasksGroupSubagents) => "Субагенты",
        (Language::English, TextKey::TasksGroupTasks) => "Tasks",
        (Language::Russian, TextKey::TasksGroupTasks) => "Задачи",
        (Language::English, TextKey::TasksGroupWatchers) => "Watchers",
        (Language::Russian, TextKey::TasksGroupWatchers) => "Наблюдатели",
        (Language::English, TextKey::TaskStatusRunning) => "running",
        (Language::Russian, TextKey::TaskStatusRunning) => "выполняется",
        (Language::English, TextKey::TaskStatusQueued) => "queued",
        (Language::Russian, TextKey::TaskStatusQueued) => "в очереди",
        (Language::English, TextKey::TaskStatusStarting) => "starting",
        (Language::Russian, TextKey::TaskStatusStarting) => "запуск",
        (Language::English, TextKey::TaskStatusDueNow) => "due now",
        (Language::Russian, TextKey::TaskStatusDueNow) => "сейчас",

        // Extensions Tabs
        (Language::English, TextKey::ExtTabHooks) => "Hooks",
        (Language::Russian, TextKey::ExtTabHooks) => "Хуки",
        (Language::English, TextKey::ExtTabPlugins) => "Plugins",
        (Language::Russian, TextKey::ExtTabPlugins) => "Плагины",
        (Language::English, TextKey::ExtTabMarketplace) => "Marketplace",
        (Language::Russian, TextKey::ExtTabMarketplace) => "Каталог",
        (Language::English, TextKey::ExtTabSkills) => "Skills",
        (Language::Russian, TextKey::ExtTabSkills) => "Навыки",
        (Language::English, TextKey::ExtTabMcpServers) => "MCP Servers",
        (Language::Russian, TextKey::ExtTabMcpServers) => "Серверы MCP",

        // Plan Approval
        (Language::English, TextKey::PlanApprovalWaiting) => "Waiting on plan approval",
        (Language::Russian, TextKey::PlanApprovalWaiting) => "Ожидание утверждения плана",
        (Language::English, TextKey::PlanApprovalNoPlan) => {
            "No plan written — approve or request changes"
        }
        (Language::Russian, TextKey::PlanApprovalNoPlan) => {
            "План не написан — утвердите или запросите изменения"
        }

        // Permissions
        (Language::English, TextKey::PermAllowOnce) => "Allow once",
        (Language::Russian, TextKey::PermAllowOnce) => "Разрешить один раз",
        (Language::English, TextKey::PermAllowAlways) => "Always allow",
        (Language::Russian, TextKey::PermAllowAlways) => "Разрешать всегда",
        (Language::English, TextKey::PermRejectOnce) => "Reject once",
        (Language::Russian, TextKey::PermRejectOnce) => "Отклонить один раз",
        (Language::English, TextKey::PermRejectAlways) => "Always reject",
        (Language::Russian, TextKey::PermRejectAlways) => "Всегда отклонять",
        (Language::English, TextKey::PermFollowupPlaceholder) => "Type message for agent...",
        (Language::Russian, TextKey::PermFollowupPlaceholder) => "Введите сообщение для агента...",

        // Questions & Surveys
        (Language::English, TextKey::QuestionOtherPlaceholder) => "Type custom answer...",
        (Language::Russian, TextKey::QuestionOtherPlaceholder) => "Введите свой вариант ответа...",
        (Language::English, TextKey::QuestionSelectOption) => "Select option",
        (Language::Russian, TextKey::QuestionSelectOption) => "Выберите вариант",
        (Language::English, TextKey::QuestionSelectOptions) => "Select options",
        (Language::Russian, TextKey::QuestionSelectOptions) => "Выберите варианты",
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
