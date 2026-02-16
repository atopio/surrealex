use crate::{
    enums::ReturnClause,
    internal_macros::push_clause,
    types::create::{ContentMode, CreateData, SetField},
};
use std::fmt::Write;

pub struct CreateBuilder {
    pub data: CreateData,
}

impl CreateBuilder {
    /// Switches the statement from `CREATE ...` to `CREATE ONLY ...`.
    ///
    /// **Note:** SurrealDB expects a single-result `RETURN` when using `ONLY`.
    /// The builder does not enforce this — the server will validate it at runtime.
    pub fn only(mut self) -> Self {
        self.data.only = true;
        self
    }

    /// Sets the data-setting mode to `CONTENT @value`.
    ///
    /// This replaces any previous `CONTENT` or `SET` clause.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::create("person")
    ///     .content("{ name: 'Tobie', company: 'SurrealDB' }")
    ///     .build();
    /// assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie', company: 'SurrealDB' }");
    /// ```
    pub fn content(mut self, value: &str) -> Self {
        self.data.content = Some(ContentMode::Content(value.to_string()));
        self
    }

    /// Adds a `SET field = value` assignment.
    ///
    /// Multiple calls accumulate assignments. If a `CONTENT` clause was previously
    /// set, it is replaced by the `SET` clause.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::create("person")
    ///     .set("name", "'Tobie'")
    ///     .set("company", "'SurrealDB'")
    ///     .build();
    /// assert_eq!(sql, "CREATE person SET name = 'Tobie', company = 'SurrealDB'");
    /// ```
    pub fn set(mut self, field: &str, value: &str) -> Self {
        match &mut self.data.content {
            Some(ContentMode::Set(fields)) => {
                fields.push(SetField {
                    field: field.to_string(),
                    value: value.to_string(),
                });
            }
            _ => {
                self.data.content = Some(ContentMode::Set(vec![SetField {
                    field: field.to_string(),
                    value: value.to_string(),
                }]));
            }
        }
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
    /// let sql = QueryBuilder::create("person")
    ///     .set("name", "'Tobie'")
    ///     .return_params(vec!["name", "id"])
    ///     .build();
    /// assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN name, id");
    /// ```
    pub fn return_params<S: Into<String>>(mut self, params: Vec<S>) -> Self {
        self.data.return_clause = Some(ReturnClause::Params(
            params.into_iter().map(|s| s.into()).collect(),
        ));
        self
    }

    /// Sets the RETURN clause to `RETURN VALUE <field>`.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::create("person")
    ///     .set("name", "'Tobie'")
    ///     .return_value("name")
    ///     .build();
    /// assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN VALUE name");
    /// ```
    pub fn return_value(mut self, field: &str) -> Self {
        self.data.return_clause = Some(ReturnClause::Value(field.to_string()));
        self
    }

    /// Sets the TIMEOUT clause with a raw SurrealQL duration string.
    ///
    /// Accepts SurrealQL duration syntax such as `"500ms"`, `"2s"`, `"1m"`.
    pub fn timeout(mut self, duration: &str) -> Self {
        self.data.timeout = Some(duration.to_string());
        self
    }

    /// Builds the final CREATE query string.
    pub fn build(self) -> String {
        let mut query = String::with_capacity(128);
        let targets = &self.data.targets;

        if self.data.only {
            push_clause!(query, "CREATE ONLY {targets}");
        } else {
            push_clause!(query, "CREATE {targets}");
        }

        if let Some(ref content) = self.data.content {
            match content {
                ContentMode::Content(value) => {
                    push_clause!(query, "CONTENT {value}");
                }
                ContentMode::Set(fields) => {
                    let assignments: String = fields
                        .iter()
                        .map(|f| format!("{} = {}", f.field, f.value))
                        .collect::<Vec<String>>()
                        .join(", ");
                    push_clause!(query, "SET {assignments}");
                }
            }
        }

        if let Some(ref rc) = self.data.return_clause {
            push_clause!(query, "RETURN {rc}");
        }

        if let Some(ref duration) = self.data.timeout {
            push_clause!(query, "TIMEOUT {duration}");
        }

        query
    }
}
