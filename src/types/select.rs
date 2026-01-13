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
pub struct GraphTraversalParams {
    /// Steps defining the traversal.
    pub steps: Vec<GraphStep>,
    /// Optional alias for the expansion.
    pub alias: Option<String>,
    pub fields: SelectionFields,
}

impl GraphTraversalParams {
    pub fn start(direction: Direction, table: impl Into<String>) -> Self {
        Self {
            steps: vec![GraphStep {
                direction,
                table: table.into(),
            }],
            alias: None,
            fields: SelectionFields::All,
        }
    }

    #[inline]
    pub fn start_in(table: impl Into<String>) -> Self {
        Self::start(Direction::In, table)
    }

    #[inline]
    pub fn start_out(table: impl Into<String>) -> Self {
        Self::start(Direction::Out, table)
    }

    pub fn step(mut self, dir: Direction, table: impl Into<String>) -> Self {
        self.steps.push(GraphStep {
            direction: dir,
            table: table.into(),
        });
        self
    }

    #[inline]
    pub fn step_in(self, table: impl Into<String>) -> Self {
        self.step(Direction::In, table)
    }

    #[inline]
    pub fn step_out(self, table: impl Into<String>) -> Self {
        self.step(Direction::Out, table)
    }

    pub fn fields(mut self, fields: SelectionFields) -> Self {
        self.fields = fields;
        self
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct GraphStep {
    pub direction: Direction,
    pub table: String,
}

impl Display for GraphStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This leverages the Display impl of Direction
        // to write the arrow followed by the table name.
        write!(f, "{}{}", self.direction, self.table)
    }
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
