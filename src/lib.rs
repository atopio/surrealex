pub mod enums;

#[cfg(feature = "macros")]
pub mod macros;

pub mod builders;
pub(crate) mod internal_macros;
pub mod structs;
pub mod traits;

use crate::{
    builders::select::SelectBuilder,
    enums::SelectionFields,
    structs::{SelectData, SelectField},
};

#[derive(Debug)]
pub struct QueryBuilder;

impl QueryBuilder {
    pub fn select(fields: SelectionFields) -> SelectBuilder {
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
        SelectBuilder { data }
    }
}
