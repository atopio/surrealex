pub mod enums;
pub(crate) mod states;
pub mod structs;
pub mod traits;

use std::marker::PhantomData;

use crate::{
    enums::{Condition, Direction, Sort},
    states::{DefaultState, FromReady, SelectState},
    structs::{OrderTerm, SelectData, SelectField},
    traits::ToSelectField,
};

/// Parameters for a two-step graph traversal expansion.
#[derive(Debug, Clone)]
pub struct GraphExpandParams {
    /// First traversal (direction and graph table).
    pub from: (Direction, String),
    /// Second traversal (direction and edge table).
    pub to: (Direction, String),
    /// Optional alias for the expansion.
    pub alias: Option<String>,
}

#[derive(Debug)]
pub struct QueryBuilder<S, D> {
    data: D,
    _state: PhantomData<S>,
}

impl QueryBuilder<DefaultState, ()> {
    pub fn select<T: ToSelectField>(items: Vec<T>) -> QueryBuilder<SelectState, SelectData> {
        let data = SelectData {
            fields: items
                .into_iter()
                .map(|item| item.to_select_field())
                .collect(),
            ..Default::default()
        };
        QueryBuilder {
            data: data,
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
        let name = format!("{}{}{}{}.*", dir1, params.from.1, dir2, params.to.1);

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

    pub fn order_by(mut self, field: &str, order: Sort, numeric: bool) -> Self {
        let order_term = OrderTerm {
            field: field.to_string(),
            order,
            numeric,
            collate: false,
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
            if self.data.only {
                query.push_str(&format!(" FROM ONLY {}", table));
            } else {
                query.push_str(&format!(" FROM {}", table));
            }
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
            query.push_str(&format!(" ORDER BY {}", order_terms));
        }

        if let Some(limit) = self.data.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if !self.data.fetch_fields.is_empty() {
            let fetch_fields = self.data.fetch_fields.join(", ");
            query.push_str(&format!(" FETCH {}", fetch_fields));
        }

        query
    }
}
