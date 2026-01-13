use std::fmt::Write;

use crate::{
    enums::{Condition, SelectionFields},
    internal_macros::push_clause,
    traits::ToSelectField,
    types::select::{GraphTraversalParams, OrderOptions, OrderTerm, SelectData, SelectField},
};

pub struct SelectBuilder {
    pub data: SelectData,
}

impl SelectBuilder {
    pub fn graph_traverse(mut self, params: GraphTraversalParams) -> Self {
        let path = params
            .steps
            .iter()
            .map(|step| step.to_string())
            .collect::<String>();

        let fields = match params.fields {
            SelectionFields::All => "*".to_string(),
            SelectionFields::Fields(select_fields) => {
                let joined = select_fields
                    .iter()
                    .map(|f| {
                        if let Some(alias) = &f.alias {
                            format!("{} AS {}", f.name, alias)
                        } else {
                            f.name.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", joined)
            }
        };

        let name = format!("{}.{}", path, fields);
        let alias = params.alias;

        self.data.fields.push(SelectField { name, alias });

        self
    }

    pub fn subquery(mut self, subquery: FromReady) -> Self {
        let field = subquery.to_select_field();
        self.data.fields.push(field);
        self
    }

    pub fn subquery_as(mut self, subquery: FromReady, alias: &str) -> Self {
        let field = (subquery, alias).to_select_field();
        self.data.fields.push(field);
        self
    }

    pub fn from(mut self, table: &str) -> FromReady {
        self.data.table = Some(table.to_string());
        self.data.only = false;
        self.transition_to_ready()
    }

    pub fn from_only(mut self, table: &str) -> FromReady {
        self.data.table = Some(table.to_string());
        self.data.only = true;
        self.transition_to_ready()
    }

    fn transition_to_ready(self) -> FromReady {
        FromReady { data: self.data }
    }
}

#[derive(Debug, Clone)]
pub struct FromReady {
    data: SelectData,
}

impl FromReady {
    pub fn r#where<T: Into<Condition>>(mut self, condition: T) -> Self {
        self.data.where_clause.push(condition.into());
        self
    }

    pub fn order_by(mut self, field: &str, order: impl Into<OrderOptions>) -> Self {
        let opt = order.into();

        let order_term = OrderTerm {
            field: field.to_string(),
            direction: opt.direction,
            numeric: opt.numeric,
            collate: opt.collate,
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
        let mut query = String::with_capacity(128);
        push_clause!(query, "SELECT");

        let fields: String = self
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
            .collect::<Vec<String>>()
            .join(", ");

        push_clause!(query, "{fields}");

        if let Some(table) = &self.data.table {
            let only = if self.data.only { " ONLY" } else { "" };
            push_clause!(query, "FROM{only} {table}");
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

        if !self.data.order_by.is_empty() {
            let order_terms = self.data.order_by.join(", ");
            push_clause!(query, "ORDER BY {order_terms}");
        }

        if let Some(limit) = self.data.limit {
            push_clause!(query, "LIMIT {limit}");
        }

        if let Some(offset) = self.data.start_at {
            push_clause!(query, "START AT {offset}");
        }

        if !self.data.fetch_fields.is_empty() {
            let fetch_fields = self.data.fetch_fields.join(", ");
            push_clause!(query, "FETCH {fetch_fields}");
        }

        query
    }
}

impl ToSelectField for FromReady {
    fn to_select_field(self) -> SelectField {
        let subquery = self.build();

        SelectField {
            name: format!("({})", subquery),
            alias: None,
        }
    }
}

impl ToSelectField for (FromReady, &str) {
    fn to_select_field(self) -> SelectField {
        let subquery = self.0.clone().build();

        SelectField {
            name: format!("({})", subquery),
            alias: Some(self.1.to_string()),
        }
    }
}
