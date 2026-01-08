use crate::enums::Condition;

#[derive(Default, Debug, Clone)]
pub struct DeleteData {
    pub targets: String,
    pub where_clause: Vec<Condition>,
}
