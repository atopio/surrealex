use crate::{
    enums::{Condition, ExplainClause, ReturnClause},
    internal_macros::push_clause,
    types::delete::DeleteData,
};
use std::fmt::Write;

pub struct DeleteBuilder {
    pub data: DeleteData,
}

impl DeleteBuilder {
    /// Switches the statement from `DELETE FROM ...` to `DELETE ONLY ...`.
    ///
    /// **Note:** SurrealDB expects a single-result `RETURN` when using `ONLY`.
    /// The builder does not enforce this — the server will validate it at runtime.
    pub fn only(mut self) -> Self {
        self.data.only = true;
        self
    }

    /// Appends a WHERE condition. Multiple calls are joined with `AND`.
    pub fn r#where<T: Into<Condition>>(mut self, condition: T) -> Self {
        self.data.where_clause.push(condition.into());
        self
    }

    /// Sets the RETURN clause to `RETURN NONE`.
    pub fn return_none(mut self) -> Self {
        self.data.return_clause = Some(ReturnClause::None);
        self
    }

    /// Sets the RETURN clause to `RETURN BEFORE`.
    pub fn return_before(mut self) -> Self {
        self.data.return_clause = Some(ReturnClause::Before);
        self
    }

    /// Sets the RETURN clause to `RETURN AFTER`.
    pub fn return_after(mut self) -> Self {
        self.data.return_clause = Some(ReturnClause::After);
        self
    }

    /// Sets the RETURN clause to `RETURN DIFF`.
    pub fn return_diff(mut self) -> Self {
        self.data.return_clause = Some(ReturnClause::Diff);
        self
    }

    /// Sets the RETURN clause to `RETURN <param1>, <param2>, ...`.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::delete("users")
    ///     .return_params(vec!["$before", "$after"])
    ///     .build();
    /// assert_eq!(sql, "DELETE FROM users RETURN $before, $after");
    /// ```
    pub fn return_params<S: Into<String>>(mut self, params: Vec<S>) -> Self {
        self.data.return_clause = Some(ReturnClause::Params(
            params.into_iter().map(|s| s.into()).collect(),
        ));
        self
    }

    /// Sets the TIMEOUT clause with a raw SurrealQL duration string.
    ///
    /// Accepts SurrealQL duration syntax such as `"500ms"`, `"2s"`, `"1m"`.
    pub fn timeout(mut self, duration: &str) -> Self {
        self.data.timeout = Some(duration.to_string());
        self
    }

    /// Adds an `EXPLAIN` clause to the statement.
    pub fn explain(mut self) -> Self {
        self.data.explain = Some(ExplainClause::Simple);
        self
    }

    /// Adds an `EXPLAIN FULL` clause to the statement.
    pub fn explain_full(mut self) -> Self {
        self.data.explain = Some(ExplainClause::Full);
        self
    }

    /// Builds the final DELETE query string.
    pub fn build(self) -> String {
        let mut query = String::with_capacity(128);
        let targets = &self.data.targets;

        if self.data.only {
            push_clause!(query, "DELETE ONLY {targets}");
        } else {
            push_clause!(query, "DELETE FROM {targets}");
        }

        if !self.data.where_clause.is_empty() {
            let conditions: String = self
                .data
                .where_clause
                .iter()
                .map(|cond| cond.to_string())
                .collect::<Vec<String>>()
                .join(" AND ");

            push_clause!(query, "WHERE {conditions}");
        }

        if let Some(ref rc) = self.data.return_clause {
            push_clause!(query, "RETURN {rc}");
        }

        if let Some(ref duration) = self.data.timeout {
            push_clause!(query, "TIMEOUT {duration}");
        }

        if let Some(ref mode) = self.data.explain {
            push_clause!(query, "{mode}");
        }

        query
    }
}
