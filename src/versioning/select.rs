use crate::{
    SurrealV1, SurrealV2,
    enums::SelectionFields,
    types::select::{GraphTraversalParams, SelectData, SelectField},
    versioning::SurrealV3,
};

/// Trait for version-specific SELECT statement rendering behavior.
///
/// Each SurrealDB version type implements this trait to provide
/// version-appropriate query generation. The implementing types
/// are zero-sized, so storing them in a builder adds no memory overhead
/// and all dispatch is monomorphized at compile time.
pub trait VersionedSelect {
    /// Applies a graph traversal expansion to the select data.
    ///
    /// Different SurrealDB versions handle field destructuring differently:
    /// - V1 expands each field into its own path (e.g. `->edge->table.field1, ->edge->table.field2`)
    /// - V2 and V3 use object destructuring syntax (e.g. `->edge->table.{field1, field2}`)
    fn graph_traverse(&self, data: &mut SelectData, params: GraphTraversalParams) {
        let path = params
            .steps
            .iter()
            .map(|step| step.to_string())
            .collect::<String>();

        let fields = match params.fields {
            SelectionFields::All => "*".to_string(),
            SelectionFields::Fields(select_fields) => {
                let joined = select_fields
                    .iter()
                    .map(|f| {
                        if let Some(alias) = &f.alias {
                            format!("{} AS {}", f.name, alias)
                        } else {
                            f.name.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", joined)
            }
        };

        let name = format!("{}.{}", path, fields);
        let alias = params.alias;

        data.fields.push(SelectField { name, alias });
    }
}

impl VersionedSelect for SurrealV1 {
    fn graph_traverse(&self, data: &mut SelectData, params: GraphTraversalParams) {
        let path = params
            .steps
            .iter()
            .map(|step| step.to_string())
            .collect::<String>();

        match params.fields {
            SelectionFields::All => {
                let name = format!("{}.*", path);
                data.fields.push(SelectField {
                    name,
                    alias: params.alias,
                });
            }
            SelectionFields::Fields(select_fields) => {
                for field in select_fields {
                    let name = format!("{}.{}", path, field.name);
                    data.fields.push(SelectField {
                        name,
                        alias: field.alias,
                    });
                }
            }
        }
    }
}

impl VersionedSelect for SurrealV2 {}
impl VersionedSelect for SurrealV3 {}
