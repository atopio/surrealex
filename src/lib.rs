pub mod enums;

#[cfg(feature = "macros")]
pub mod macros;

pub(crate) mod states;
pub mod structs;
pub mod traits;

use std::fmt::Write;
use std::marker::PhantomData;

use crate::{
    enums::{Condition, Direction, SelectionFields, Sort},
    states::{DefaultState, FromReady, SelectState},
    structs::{GraphExpandParams, OrderTerm, SelectData, SelectField},
};

#[derive(Debug)]
pub struct QueryBuilder<S, D> {
    data: D,
    _state: PhantomData<S>,
}

impl QueryBuilder<DefaultState, ()> {
    pub fn select(fields: SelectionFields) -> QueryBuilder<SelectState, SelectData> {
        let data = SelectData {
            fields: match fields {
                SelectionFields::All => vec![SelectField {
                    name: "*".to_string(),
                    alias: None,
                }],
                SelectionFields::Fields(select_fields) => select_fields,
            },
            ..Default::default()
        };
        QueryBuilder {
            data,
            _state: PhantomData,
        }
    }
}

impl QueryBuilder<SelectState, SelectData> {
    pub fn graph_traverse(mut self, params: GraphExpandParams) -> Self {
        let dir1 = match params.from.0 {
            Direction::Out => "->",
            Direction::In => "<-",
        };
        let dir2 = match params.to.0 {
            Direction::Out => "->",
            Direction::In => "<-",
        };

        // Building: ->table1->table2.*
        let fields = match params.fields {
            SelectionFields::All => "*".to_string(),
            SelectionFields::Fields(select_fields) => format!(
                "{{{}}}",
                select_fields
                    .iter()
                    .map(|f| {
                        if let Some(alias) = &f.alias {
                            format!("{} AS {}", f.name, alias)
                        } else {
                            f.name.clone()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        };
        let name = format!("{}{}{}{}.{fields}", dir1, params.from.1, dir2, params.to.1);

        // In SurrealDB, aliases for graph traversals are very common
        let alias = params.alias;

        // Push it into our unified items list
        self.data.fields.push(SelectField { name, alias });

        self
    }

    pub fn from(mut self, table: &str) -> QueryBuilder<FromReady, SelectData> {
        self.data.table = Some(table.to_string());
        self.data.only = false;
        self.transition_to_ready()
    }

    pub fn from_only(mut self, table: &str) -> QueryBuilder<FromReady, SelectData> {
        self.data.table = Some(table.to_string());
        self.data.only = true;
        self.transition_to_ready()
    }

    fn transition_to_ready(self) -> QueryBuilder<FromReady, SelectData> {
        QueryBuilder {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl QueryBuilder<FromReady, SelectData> {
    pub fn r#where<T: Into<Condition>>(mut self, condition: T) -> Self {
        self.data.where_clause.push(condition.into());
        self
    }

    pub fn order_by(mut self, field: &str, order: Sort, numeric: bool, collate: bool) -> Self {
        let order_term = OrderTerm {
            field: field.to_string(),
            order,
            numeric,
            collate,
        };
        self.data.order_by.push(order_term.to_string());
        self
    }

    pub fn order_random(mut self) -> Self {
        self.data.order_by = vec!["RAND()".to_string()];
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.data.limit = Some(limit);
        self
    }

    pub fn start_at(mut self, offset: u64) -> Self {
        self.data.start_at = Some(offset);
        self
    }

    pub fn fetch(mut self, fields: Vec<&str>) -> Self {
        self.data
            .fetch_fields
            .extend(fields.iter().map(|s| s.to_string()));
        self
    }

    pub fn build(self) -> String {
        let mut query = String::from("SELECT ");

        let fields: Vec<String> = self
            .data
            .fields
            .iter()
            .map(|field| {
                if let Some(alias) = &field.alias {
                    format!("{} AS {}", field.name, alias)
                } else {
                    field.name.clone()
                }
            })
            .collect();

        query.push_str(&fields.join(", "));

        if let Some(table) = &self.data.table {
            let only = if self.data.only { " ONLY" } else { "" };
            let _ = write!(query, " FROM{only} {table}");
        }

        if !self.data.where_clause.is_empty() {
            let conditions: String = self
                .data
                .where_clause
                .iter()
                .map(|cond| cond.to_string())
                .collect::<Vec<String>>()
                .join(" AND ");

            query.push_str(" WHERE ");
            query.push_str(&conditions);
        }

        if !self.data.order_by.is_empty() {
            let order_terms = self.data.order_by.join(", ");
            let _ = write!(query, " ORDER BY {order_terms}");
        }

        if let Some(limit) = self.data.limit {
            let _ = write!(query, " LIMIT {limit}");
        }

        if let Some(offset) = self.data.start_at {
            let _ = write!(query, " START AT {offset}");
        }

        if !self.data.fetch_fields.is_empty() {
            let fetch_fields = self.data.fetch_fields.join(", ");
            let _ = write!(query, " FETCH {fetch_fields}");
        }

        query
    }
}
