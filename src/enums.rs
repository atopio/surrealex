use std::fmt::Display;

use crate::{
    traits::ToSelectField,
    types::select::{OrderOptions, SelectField},
};

/// Represents the RETURN clause variants shared across statements.
///
/// SurrealQL supports:
/// - `RETURN NONE`
/// - `RETURN BEFORE`
/// - `RETURN AFTER`
/// - `RETURN DIFF`
/// - `RETURN <param1>, <param2>, ...`
/// - `RETURN VALUE <param>`
#[derive(Debug, Clone)]
pub enum ReturnClause {
    /// `RETURN NONE`
    None,
    /// `RETURN BEFORE`
    Before,
    /// `RETURN AFTER`
    After,
    /// `RETURN DIFF`
    Diff,
    /// `RETURN <field1>, <field2>, ...`
    Params(Vec<String>),
    /// `RETURN VALUE <field>`
    Value(String),
}

impl Display for ReturnClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnClause::None => write!(f, "NONE"),
            ReturnClause::Before => write!(f, "BEFORE"),
            ReturnClause::After => write!(f, "AFTER"),
            ReturnClause::Diff => write!(f, "DIFF"),
            ReturnClause::Params(params) => {
                let joined = params.join(", ");
                write!(f, "{joined}")
            }
            ReturnClause::Value(field) => write!(f, "VALUE {field}"),
        }
    }
}

/// Represents EXPLAIN clause modes shared across statements.
///
/// SurrealQL supports: `EXPLAIN` or `EXPLAIN FULL`.
#[derive(Debug, Clone, PartialEq)]
pub enum ExplainClause {
    /// `EXPLAIN`
    Simple,
    /// `EXPLAIN FULL`
    Full,
}

impl Display for ExplainClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplainClause::Simple => write!(f, "EXPLAIN"),
            ExplainClause::Full => write!(f, "EXPLAIN FULL"),
        }
    }
}

/// Direction of graph traversal arrows.
#[derive(Debug, Clone)]
pub enum Direction {
    /// Outgoing (`->`).
    Out,
    /// Incoming (`<-`).
    In,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Out => write!(f, "->"),
            Direction::In => write!(f, "<-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    /// A simple, raw condition string (e.g., "price > 50").
    Simple(String),
    /// A list of conditions that will be joined by 'AND'.
    And(Vec<Condition>),
    /// A list of conditions that will be joined by 'OR'.
    Or(Vec<Condition>),
}

impl Condition {
    pub fn new(s: impl Into<String>) -> Self {
        Condition::Simple(s.into())
    }

    pub fn and(self, cond: impl Into<Condition>) -> Self {
        match self {
            Condition::And(mut conds) => {
                conds.push(cond.into());
                Condition::And(conds)
            }
            _ => Condition::And(vec![self, cond.into()]),
        }
    }

    pub fn or(self, cond: impl Into<Condition>) -> Self {
        Condition::Or(vec![self, cond.into()])
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Simple(s) => write!(f, "{}", s),
            Condition::And(conds) => {
                write!(f, "(")?;
                for (i, condition) in conds.iter().enumerate() {
                    if i > 0 {
                        write!(f, " AND ")?;
                    }
                    write!(f, "{}", condition)?;
                }
                write!(f, ")")
            }
            Condition::Or(conds) => {
                write!(f, "(")?;
                for (i, condition) in conds.iter().enumerate() {
                    if i > 0 {
                        write!(f, " OR ")?;
                    }
                    write!(f, "{}", condition)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl From<&str> for Condition {
    fn from(s: &str) -> Self {
        Condition::Simple(s.to_string())
    }
}

impl From<String> for Condition {
    fn from(s: String) -> Self {
        Condition::Simple(s)
    }
}

#[derive(Debug, Clone, Default)]
pub enum Sort {
    #[default]
    Asc,
    Desc,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::Asc => write!(f, "ASC"),
            Sort::Desc => write!(f, "DESC"),
        }
    }
}

impl From<Sort> for OrderOptions {
    fn from(direction: Sort) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }
}

impl Sort {
    pub fn numeric(self) -> OrderOptions {
        OrderOptions::from(self).numeric()
    }

    pub fn collate(self) -> OrderOptions {
        OrderOptions::from(self).collate()
    }
}

#[derive(Debug, Clone, Default)]
pub enum SelectionFields {
    /// Equivalent to .*
    #[default]
    All,
    /// Equivalent to .{field1, field2 AS alias}
    Fields(Vec<SelectField>),
}

impl SelectionFields {
    /// Helper to create a Fields variant from anything that implements ToSelectField
    pub fn from_items<T: ToSelectField>(items: Vec<T>) -> Self {
        SelectionFields::Fields(items.into_iter().map(|i| i.to_select_field()).collect())
    }
}
