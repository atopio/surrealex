use crate::enums::{Condition, ExplainClause, ReturnClause};

#[derive(Default, Debug, Clone)]
pub struct DeleteData {
    pub targets: String,
    pub where_clause: Vec<Condition>,
    /// When `true`, emits `DELETE ONLY` instead of `DELETE FROM`.
    ///
    /// **Note:** SurrealDB expects a single-result `RETURN` when using `ONLY`.
    /// The builder does not enforce this — the server will validate it at runtime.
    pub only: bool,
    /// Optional RETURN clause (`RETURN NONE | BEFORE | AFTER | DIFF | <params> | VALUE <param>`).
    pub return_clause: Option<ReturnClause>,
    /// Optional TIMEOUT duration as a raw SurrealQL duration string (e.g., `"2s"`, `"500ms"`).
    pub timeout: Option<String>,
    /// Optional EXPLAIN mode (`EXPLAIN` or `EXPLAIN FULL`).
    pub explain: Option<ExplainClause>,
}
