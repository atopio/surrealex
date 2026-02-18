use crate::enums::ReturnClause;
use crate::types::create::SetField;

/// Represents the data-providing mode for an INSERT statement.
///
/// SurrealQL supports two ways to provide data in an INSERT:
/// - `@value` — a raw JSON/SurrealQL object or array of objects
/// - `(@fields) VALUES (@values), ...` — explicit fields and value tuples
#[derive(Debug, Clone)]
pub enum InsertContent {
    /// A raw value expression (e.g., `{ name: 'Tobie', age: 30 }` or
    /// `[{ name: 'Tobie' }, { name: 'Jaime' }]`).
    Value(String),
    /// Explicit `(@fields) VALUES (@values), ...` form.
    FieldsValues {
        /// The field names (e.g., `["name", "age"]`).
        fields: Vec<String>,
        /// One or more value tuples. Each inner `Vec` corresponds to one row
        /// and must have the same length as `fields`.
        values: Vec<Vec<String>>,
    },
}

/// Holds all the data needed to build an INSERT statement.
#[derive(Default, Debug, Clone)]
pub struct InsertData {
    /// The target table or record id (e.g., `"person"`, `"person:tobie"`).
    pub target: String,
    /// When `true`, emits `INSERT RELATION` instead of `INSERT`.
    pub relation: bool,
    /// When `true`, emits `IGNORE` after `INSERT [RELATION]`.
    pub ignore: bool,
    /// Optional data content (`@value` or `(@fields) VALUES (@values)`).
    pub content: Option<InsertContent>,
    /// Optional `ON DUPLICATE KEY UPDATE` assignments.
    pub on_duplicate_key_update: Vec<SetField>,
    /// Optional RETURN clause (`RETURN NONE | BEFORE | AFTER | DIFF | <params> | VALUE <param>`).
    pub return_clause: Option<ReturnClause>,
}
