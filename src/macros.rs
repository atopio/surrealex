#[macro_export]
macro_rules! fields {
    // 1. Handle the "Only Star" cases
    (*) => { $crate::enums::SelectionFields::All };
    (all) => { $crate::enums::SelectionFields::All };

    // 2. Handle the "Mixed" cases (entry point)
    ($($item:tt),*) => {
        $crate::enums::SelectionFields::Fields(vec![
            $( $crate::fields!(@item $item) ),*
        ])
    };

    // 3. The "Internal Arm" that converts tokens to SelectFields
    (@item *) => {
        $crate::structs::SelectField {
            name: "*".to_string(),
            alias: None,
        }
    };
    (@item $expr:expr) => {
        $crate::traits::ToSelectField::to_select_field($expr)
    };
}
