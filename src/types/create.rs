use crate::enums::ReturnClause;

/// Represents the data-setting mode for a CREATE statement.
///
/// SurrealQL supports two mutually exclusive ways to set data on a new record:
/// - `CONTENT @value` — a raw JSON/SurrealQL object
/// - `SET @field = @value, ...` — individual field assignments
#[derive(Debug, Clone)]
pub enum ContentMode {
    /// `CONTENT @value`
    Content(String),
    /// `SET @field = @value, ...`
    Set(Vec<SetField>),
}

/// A single `field = value` pair used in the `SET` clause.
#[derive(Debug, Clone)]
pub struct SetField {
    /// The field name (e.g., `"name"`, `"settings.theme"`).
    pub field: String,
    /// The raw value expression (e.g., `"'Tobie'"`, `"42"`, `"['Rust', 'Go']"`).
    pub value: String,
}

/// Holds all the data needed to build a CREATE statement.
#[derive(Default, Debug, Clone)]
pub struct CreateData {
    /// The target table or record id (e.g., `"person"`, `"person:tobie"`).
    pub targets: String,
    /// When `true`, emits `CREATE ONLY` instead of `CREATE`.
    ///
    /// **Note:** SurrealDB expects a single-result `RETURN` when using `ONLY`.
    /// The builder does not enforce this — the server will validate it at runtime.
    pub only: bool,
    /// Optional data-setting mode (`CONTENT` or `SET`).
    pub content: Option<ContentMode>,
    /// Optional RETURN clause (`RETURN NONE | BEFORE | AFTER | DIFF | <params> | VALUE <param>`).
    pub return_clause: Option<ReturnClause>,
    /// Optional TIMEOUT duration as a raw SurrealQL duration string (e.g., `"2s"`, `"500ms"`).
    pub timeout: Option<String>,
}
