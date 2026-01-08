use surrealex::{QueryBuilder, enums::Condition};

#[test]
fn build_delete_from_targets() {
    let sql = QueryBuilder::delete("my_table").build();
    assert_eq!(sql, "DELETE FROM my_table");
}

#[test]
fn where_clauses_are_included_in_build_and_display_correctly() {
    let sql = QueryBuilder::delete("users")
        .r#where("active = true")
        .r#where(Condition::Simple("age > 30".to_string()))
        .r#where("country = 'US'".to_string())
        .build();

    assert_eq!(
        sql,
        "DELETE FROM users WHERE active = true AND age > 30 AND country = 'US'"
    );
}

#[test]
fn where_accepts_different_into_condition_types_and_builds() {
    let sql = QueryBuilder::delete("items")
        .r#where("available = true")
        .r#where(Condition::Simple("price > 100".into()))
        .r#where("category = 'books'".to_string())
        .build();

    assert_eq!(
        sql,
        "DELETE FROM items WHERE available = true AND price > 100 AND category = 'books'"
    );
}

#[test]
fn complex_nested_conditions_render_and_build() {
    // Build a nested condition: (a = 1 AND (b = 2 OR c = 3))
    let nested = Condition::And(vec![
        Condition::Simple("a = 1".into()),
        Condition::Or(vec![
            Condition::Simple("b = 2".into()),
            Condition::Simple("c = 3".into()),
        ]),
    ]);

    let sql = QueryBuilder::delete("posts").r#where(nested).build();

    assert_eq!(sql, "DELETE FROM posts WHERE (a = 1 AND (b = 2 OR c = 3))");
}
