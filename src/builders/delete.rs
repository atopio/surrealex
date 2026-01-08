use crate::{enums::Condition, internal_macros::push_clause, types::delete::DeleteData};
use std::fmt::Write;

pub struct DeleteBuilder {
    pub data: DeleteData,
}

impl DeleteBuilder {
    pub fn r#where<T: Into<Condition>>(mut self, condition: T) -> Self {
        self.data.where_clause.push(condition.into());
        self
    }

    pub fn build(self) -> String {
        let mut query = String::with_capacity(128);
        let targets = self.data.targets;
        push_clause!(query, "DELETE FROM {targets}");

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

        query
    }
}
