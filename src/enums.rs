use std::fmt::Display;

/// Direction of graph traversal arrows.
#[derive(Debug, Clone)]
pub enum Direction {
    /// Outgoing (`->`).
    Out,
    /// Incoming (`<-`).
    In,
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

#[derive(Debug, Clone)]
pub enum Sort {
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

impl Default for Sort {
    fn default() -> Self {
        Sort::Asc
    }
}
