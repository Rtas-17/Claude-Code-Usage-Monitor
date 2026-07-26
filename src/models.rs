use std::time::SystemTime;

#[derive(Clone, Debug, Default)]
pub struct UsageSection {
    pub percentage: f64,
    pub resets_at: Option<SystemTime>,
}

#[derive(Clone, Debug, Default)]
pub struct UsageData {
    pub session: UsageSection,
    pub weekly: UsageSection,
    /// Weekly, model-scoped usage for Fable (Claude Code only). Present only
    /// when the usage endpoint reports a Fable-scoped weekly limit.
    pub fable: Option<UsageSection>,
    /// Label for the weekly section when it can represent more than one
    /// window (e.g. OpenCode picks whichever of 7d/30d is more used).
    pub weekly_label: Option<&'static str>,
}

#[derive(Clone, Debug, Default)]
pub struct AppUsageData {
    pub claude_code: Option<UsageData>,
    pub codex: Option<UsageData>,
    pub antigravity: Option<UsageData>,
    pub minimax: Option<UsageData>,
    /// Cursor plan usage. Auto maps to `session`, API maps to `weekly`.
    pub cursor: Option<UsageData>,
    pub ollama: Option<UsageData>,
    pub opencode: Option<UsageData>,
}