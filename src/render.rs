/// Target SurrealDB version for query rendering.
///
/// Controls how certain syntax constructs (e.g. graph traversal field
/// destructuring) are emitted.
///
/// - `V2` (default) — uses the current SurrealDB v2 syntax
///   (e.g. `->edge->table.{field1, field2}`).
/// - `V1` — uses SurrealDB v1-compatible syntax where object
///   destructuring is not available (e.g. `->edge->table.field1,
///   ->edge->table.field2`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SurrealVersion {
    /// SurrealDB v1 — no object destructuring on graph traversals.
    V1,
    /// SurrealDB v2 (default) — supports `->edge->table.{field, …}` syntax.
    #[default]
    V2,
}
