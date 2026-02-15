pub mod enums;

#[cfg(feature = "macros")]
pub mod macros;

pub mod builders;
pub(crate) mod internal_macros;
pub mod traits;
pub mod types;

use crate::{
    builders::{create::CreateBuilder, delete::DeleteBuilder, select::SelectBuilder},
    enums::SelectionFields,
    types::{
        create::CreateData,
        delete::DeleteData,
        select::{SelectData, SelectField},
    },
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

    pub fn delete(targets: &str) -> DeleteBuilder {
        let data = DeleteData {
            targets: targets.to_string(),
            ..Default::default()
        };
        DeleteBuilder { data }
    }

    pub fn create(targets: &str) -> CreateBuilder {
        let data = CreateData {
            targets: targets.to_string(),
            ..Default::default()
        };
        CreateBuilder { data }
    }
}
