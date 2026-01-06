use crate::types::select::SelectField;

pub trait ToSelectField {
    fn to_select_field(self) -> SelectField;
}

impl ToSelectField for &str {
    fn to_select_field(self) -> SelectField {
        SelectField {
            name: self.to_string(),
            alias: None,
        }
    }
}

impl ToSelectField for (&str, &str) {
    fn to_select_field(self) -> SelectField {
        SelectField {
            name: self.0.to_string(),
            alias: Some(self.1.to_string()),
        }
    }
}
