use std::fmt::Display;

use crate::enums::{Condition, Sort};

#[derive(Default, Debug)]
pub struct SelectData {
    pub fields: Vec<SelectField>,
    pub table: Option<String>,
    pub limit: Option<u64>,
    pub only: bool,
    pub where_clause: Vec<Condition>,
    pub fetch_fields: Vec<String>,
    pub order_by: Vec<String>,
}

#[derive(Debug)]
pub struct SelectField {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Default)]
pub struct OrderTerm {
    pub field: String,
    pub order: Sort,
    pub numeric: bool,
    pub collate: bool,
}

impl Display for OrderTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.numeric {
            write!(f, "{} {} NUMERIC", self.field, self.order)
        } else if self.collate {
            write!(f, "{} {} COLLATE", self.field, self.order)
        } else {
            write!(f, "{} {}", self.field, self.order)
        }
    }
}
