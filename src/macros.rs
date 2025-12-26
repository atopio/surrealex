#[macro_export]
macro_rules! fields {
    (*) => { $crate::enums::SelectionFields::All };
    (all) => { $crate::enums::SelectionFields::All };
    ($($item:expr),*) => {
        $crate::enums::SelectionFields::Fields(vec![
            $( $crate::traits::ToSelectField::to_select_field(&$item) ),*
        ])
    };
}
