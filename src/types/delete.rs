use std::fmt::Display;

use crate::enums::Condition;

/// Represents the RETURN clause variants for a DELETE statement.
///
/// SurrealQL supports: `RETURN NONE | BEFORE | AFTER | DIFF | <params>`
#[derive(Debug, Clone)]
pub enum ReturnClause {
    /// `RETURN NONE`
    None,
    /// `RETURN BEFORE`
    Before,
    /// `RETURN AFTER`
    After,
    /// `RETURN DIFF`
    Diff,
    /// `RETURN <field1>, <field2>, ...`
    Params(Vec<String>),
}

impl Display for ReturnClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnClause::None => write!(f, "NONE"),
            ReturnClause::Before => write!(f, "BEFORE"),
            ReturnClause::After => write!(f, "AFTER"),
            ReturnClause::Diff => write!(f, "DIFF"),
            ReturnClause::Params(params) => {
                let joined = params.join(", ");
                write!(f, "{joined}")
            }
        }
    }
}

/// Represents the EXPLAIN clause mode for a DELETE statement.
///
/// SurrealQL supports: `EXPLAIN` or `EXPLAIN FULL`
#[derive(Debug, Clone, PartialEq)]
pub enum ExplainMode {
    /// `EXPLAIN`
    Simple,
    /// `EXPLAIN FULL`
    Full,
}

impl Display for ExplainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainMode::Simple => write!(f, "EXPLAIN"),
            ExplainMode::Full => write!(f, "EXPLAIN FULL"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct DeleteData {
    pub targets: String,
    pub where_clause: Vec<Condition>,
    /// When `true`, emits `DELETE ONLY` instead of `DELETE FROM`.
    ///
    /// **Note:** SurrealDB expects a single-result `RETURN` when using `ONLY`.
    /// The builder does not enforce this — the server will validate it at runtime.
    pub only: bool,
    /// Optional RETURN clause (`RETURN NONE | BEFORE | AFTER | DIFF | <params>`).
    pub return_clause: Option<ReturnClause>,
    /// Optional TIMEOUT duration as a raw SurrealQL duration string (e.g., `"2s"`, `"500ms"`).
    pub timeout: Option<String>,
    /// Optional EXPLAIN mode (`EXPLAIN` or `EXPLAIN FULL`).
    pub explain: Option<ExplainMode>,
}
