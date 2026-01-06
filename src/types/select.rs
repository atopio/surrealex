use std::fmt::Display;

use crate::enums::{Condition, Direction, SelectionFields, Sort};

#[derive(Default, Debug, Clone)]
pub struct SelectData {
    pub fields: Vec<SelectField>,
    pub table: Option<String>,
    pub limit: Option<u64>,
    pub only: bool,
    pub where_clause: Vec<Condition>,
    pub fetch_fields: Vec<String>,
    pub order_by: Vec<String>,
    pub start_at: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct SelectField {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Default)]
pub struct OrderTerm {
    pub field: String,
    pub direction: Sort,
    pub numeric: bool,
    pub collate: bool,
}

impl Display for OrderTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.numeric && self.collate {
            write!(f, "{} COLLATE NUMERIC {}", self.field, self.direction)
        } else if self.numeric {
            write!(f, "{} NUMERIC {}", self.field, self.direction)
        } else if self.collate {
            write!(f, "{} COLLATE {}", self.field, self.direction)
        } else {
            write!(f, "{} {}", self.field, self.direction)
        }
    }
}

/// Parameters for a two-step graph traversal expansion.
#[derive(Debug, Clone)]
pub struct GraphExpandParams {
    /// First traversal (direction and graph table).
    pub from: (Direction, String),
    /// Second traversal (direction and edge table).
    pub to: (Direction, String),
    /// Optional alias for the expansion.
    pub alias: Option<String>,
    pub fields: SelectionFields,
}

#[derive(Debug, Clone, Default)]
pub struct OrderOptions {
    pub numeric: bool,
    pub collate: bool,
    pub direction: Sort,
}

impl OrderOptions {
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    pub fn collate(mut self) -> Self {
        self.collate = true;
        self
    }
}

impl From<()> for OrderOptions {
    fn from(_: ()) -> Self {
        OrderOptions::default()
    }
}
