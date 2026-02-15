pub mod enums;

#[cfg(feature = "macros")]
pub mod macros;

pub mod builders;
pub(crate) mod internal_macros;
pub mod render;
pub mod structs;
pub mod traits;
pub mod types;

pub use crate::render::SurrealVersion;

use crate::{
    builders::{delete::DeleteBuilder, select::SelectBuilder},
    enums::SelectionFields,
    types::{
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

    /// Create a version-aware query builder.
    ///
    /// Use this to target a specific SurrealDB version for query rendering.
    ///
    /// ```rust
    /// use surrealex::{QueryBuilder, SurrealVersion};
    /// let q = QueryBuilder::with_version(SurrealVersion::V1)
    ///     .select(surrealex::enums::SelectionFields::All)
    ///     .from("users")
    ///     .build();
    /// ```
    pub fn with_version(version: SurrealVersion) -> VersionedQueryBuilder {
        VersionedQueryBuilder { version }
    }
}

/// A query builder that targets a specific [`SurrealVersion`].
///
/// Created via [`QueryBuilder::with_version`].
#[derive(Debug, Clone, Copy)]
pub struct VersionedQueryBuilder {
    version: SurrealVersion,
}

impl VersionedQueryBuilder {
    pub fn select(self, fields: SelectionFields) -> SelectBuilder {
        let data = SelectData {
            fields: match fields {
                SelectionFields::All => vec![SelectField {
                    name: "*".to_string(),
                    alias: None,
                }],
                SelectionFields::Fields(select_fields) => select_fields,
            },
            version: self.version,
            ..Default::default()
        };
        SelectBuilder { data }
    }

    pub fn delete(self, targets: &str) -> DeleteBuilder {
        let data = DeleteData {
            targets: targets.to_string(),
            ..Default::default()
        };
        DeleteBuilder { data }
    }
}
