pub mod enums;

#[cfg(feature = "macros")]
pub mod macros;

pub mod builders;
pub(crate) mod internal_macros;
pub mod traits;
pub mod types;
pub mod versioning;

pub use crate::versioning::{SurrealV1, SurrealV2};

use crate::{
    builders::{create::CreateBuilder, delete::DeleteBuilder, select::SelectBuilder},
    enums::SelectionFields,
    types::{
        create::CreateData,
        delete::DeleteData,
        select::{SelectData, SelectField},
    },
    versioning::select::VersionedSelect,
};

#[derive(Debug)]
pub struct QueryBuilder;

impl QueryBuilder {
    pub fn select(fields: SelectionFields) -> SelectBuilder<SurrealV2> {
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
        SelectBuilder {
            data,
            renderer: SurrealV2,
        }
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

    /// Create a version-aware query builder.
    ///
    /// Use this to target a specific SurrealDB version for query rendering.
    /// The version is expressed as a zero-sized type (`SurrealV1` or `SurrealV2`),
    /// enabling fully monomorphized, zero-cost dispatch at compile time.
    ///
    /// ```rust
    /// use surrealex::{QueryBuilder, SurrealV1};
    /// let q = QueryBuilder::with_version(SurrealV1)
    ///     .select(surrealex::enums::SelectionFields::All)
    ///     .from("users")
    ///     .build();
    /// ```
    pub fn with_version<V>(renderer: V) -> VersionedQueryBuilder<V> {
        VersionedQueryBuilder { renderer }
    }
}

/// A query builder that targets a specific SurrealDB version.
///
/// Created via [`QueryBuilder::with_version`]. The version type `V` is a
/// zero-sized struct (e.g. [`SurrealV1`] or [`SurrealV2`]), so this builder
/// carries no runtime overhead.
#[derive(Debug, Clone, Copy)]
pub struct VersionedQueryBuilder<V> {
    renderer: V,
}

impl<V: VersionedSelect> VersionedQueryBuilder<V> {
    pub fn select(self, fields: SelectionFields) -> SelectBuilder<V> {
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
        SelectBuilder {
            data,
            renderer: self.renderer,
        }
    }
}

impl<V> VersionedQueryBuilder<V> {
    pub fn delete(self, targets: &str) -> DeleteBuilder {
        let data = DeleteData {
            targets: targets.to_string(),
            ..Default::default()
        };
        DeleteBuilder { data }
    }

    pub fn create(self, targets: &str) -> CreateBuilder {
        let data = CreateData {
            targets: targets.to_string(),
            ..Default::default()
        };
        CreateBuilder { data }
    }
}
