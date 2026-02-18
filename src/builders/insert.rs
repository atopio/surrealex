use crate::{
    enums::ReturnClause,
    internal_macros::push_clause,
    types::{
        create::SetField,
        insert::{InsertContent, InsertData},
    },
};
use std::fmt::Write;

pub struct InsertBuilder {
    pub data: InsertData,
}

impl InsertBuilder {
    /// Adds the `RELATION` keyword to the INSERT statement.
    ///
    /// Produces `INSERT RELATION ... INTO @what`.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("knows")
    ///     .relation()
    ///     .content("{ in: person:tobie, out: person:jaime, since: '2024-01-01' }")
    ///     .build();
    /// assert_eq!(sql, "INSERT RELATION INTO knows { in: person:tobie, out: person:jaime, since: '2024-01-01' }");
    /// ```
    pub fn relation(mut self) -> Self {
        self.data.relation = true;
        self
    }

    /// Adds the `IGNORE` keyword to the INSERT statement.
    ///
    /// Produces `INSERT IGNORE INTO @what` (or `INSERT RELATION IGNORE INTO @what`).
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("person")
    ///     .ignore()
    ///     .content("{ id: 'tobie', name: 'Tobie' }")
    ///     .build();
    /// assert_eq!(sql, "INSERT IGNORE INTO person { id: 'tobie', name: 'Tobie' }");
    /// ```
    pub fn ignore(mut self) -> Self {
        self.data.ignore = true;
        self
    }

    /// Sets the data-providing mode to a raw value expression (`@value`).
    ///
    /// This replaces any previous `content` or `fields_values` clause.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("person")
    ///     .content("{ name: 'Tobie', company: 'SurrealDB' }")
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person { name: 'Tobie', company: 'SurrealDB' }");
    /// ```
    pub fn content(mut self, value: &str) -> Self {
        self.data.content = Some(InsertContent::Value(value.to_string()));
        self
    }

    /// Sets the fields for the `(@fields) VALUES (@values)` form.
    ///
    /// This replaces any previous content clause. Call `.values()` afterwards
    /// to add one or more value tuples.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("person")
    ///     .fields(vec!["name", "age"])
    ///     .values(vec!["'Tobie'", "42"])
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person (name, age) VALUES ('Tobie', 42)");
    /// ```
    pub fn fields<S: Into<String>>(mut self, fields: Vec<S>) -> Self {
        let fields: Vec<String> = fields.into_iter().map(|s| s.into()).collect();
        match &mut self.data.content {
            Some(InsertContent::FieldsValues {
                fields: existing_fields,
                ..
            }) => {
                *existing_fields = fields;
            }
            _ => {
                self.data.content = Some(InsertContent::FieldsValues {
                    fields,
                    values: Vec::new(),
                });
            }
        }
        self
    }

    /// Adds a row of values for the `(@fields) VALUES (@values)` form.
    ///
    /// Multiple calls accumulate additional value tuples. If no `fields` have
    /// been set yet, this will initialise a `FieldsValues` content with empty fields.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("person")
    ///     .fields(vec!["name", "age"])
    ///     .values(vec!["'Tobie'", "42"])
    ///     .values(vec!["'Jaime'", "35"])
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person (name, age) VALUES ('Tobie', 42), ('Jaime', 35)");
    /// ```
    pub fn values<S: Into<String>>(mut self, row: Vec<S>) -> Self {
        let row: Vec<String> = row.into_iter().map(|s| s.into()).collect();
        match &mut self.data.content {
            Some(InsertContent::FieldsValues { values, .. }) => {
                values.push(row);
            }
            _ => {
                self.data.content = Some(InsertContent::FieldsValues {
                    fields: Vec::new(),
                    values: vec![row],
                });
            }
        }
        self
    }

    /// Adds a `field = value` pair to the `ON DUPLICATE KEY UPDATE` clause.
    ///
    /// Multiple calls accumulate assignments.
    ///
    /// # Example
    /// ```
    /// # use surrealex::QueryBuilder;
    /// let sql = QueryBuilder::insert("person")
    ///     .fields(vec!["name", "age"])
    ///     .values(vec!["'Tobie'", "42"])
    ///     .on_duplicate_key_update("age", "42")
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person (name, age) VALUES ('Tobie', 42) ON DUPLICATE KEY UPDATE age = 42");
    /// ```
    pub fn on_duplicate_key_update(mut self, field: &str, value: &str) -> Self {
        self.data.on_duplicate_key_update.push(SetField {
            field: field.to_string(),
            value: value.to_string(),
        });
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
    /// let sql = QueryBuilder::insert("person")
    ///     .content("{ name: 'Tobie' }")
    ///     .return_params(vec!["name", "id"])
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN name, id");
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
    /// let sql = QueryBuilder::insert("person")
    ///     .content("{ name: 'Tobie' }")
    ///     .return_value("name")
    ///     .build();
    /// assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN VALUE name");
    /// ```
    pub fn return_value(mut self, field: &str) -> Self {
        self.data.return_clause = Some(ReturnClause::Value(field.to_string()));
        self
    }

    /// Builds the final INSERT query string.
    pub fn build(self) -> String {
        let mut query = String::with_capacity(128);
        let target = &self.data.target;

        // INSERT [ RELATION ] [ IGNORE ] INTO @what
        match (self.data.relation, self.data.ignore) {
            (true, true) => push_clause!(query, "INSERT RELATION IGNORE INTO {target}"),
            (true, false) => push_clause!(query, "INSERT RELATION INTO {target}"),
            (false, true) => push_clause!(query, "INSERT IGNORE INTO {target}"),
            (false, false) => push_clause!(query, "INSERT INTO {target}"),
        }

        // [ @value | (@fields) VALUES (@values) ]
        if let Some(ref content) = self.data.content {
            match content {
                InsertContent::Value(value) => {
                    push_clause!(query, "{value}");
                }
                InsertContent::FieldsValues { fields, values } => {
                    if !fields.is_empty() {
                        let fields_str = fields.join(", ");
                        push_clause!(query, "({fields_str})");
                    }
                    if !values.is_empty() {
                        let value_tuples: String = values
                            .iter()
                            .map(|row| {
                                let row_str = row.join(", ");
                                format!("({row_str})")
                            })
                            .collect::<Vec<String>>()
                            .join(", ");
                        push_clause!(query, "VALUES {value_tuples}");
                    }
                }
            }
        }

        // [ ON DUPLICATE KEY UPDATE @field = @value ... ]
        if !self.data.on_duplicate_key_update.is_empty() {
            let assignments: String = self
                .data
                .on_duplicate_key_update
                .iter()
                .map(|f| format!("{} = {}", f.field, f.value))
                .collect::<Vec<String>>()
                .join(", ");
            push_clause!(query, "ON DUPLICATE KEY UPDATE {assignments}");
        }

        // [ RETURN ... ]
        if let Some(ref rc) = self.data.return_clause {
            push_clause!(query, "RETURN {rc}");
        }

        query
    }
}
