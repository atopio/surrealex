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

// --- ONLY clause tests ---

#[test]
fn only_emits_delete_only_instead_of_delete_from() {
    let sql = QueryBuilder::delete("person:one").only().build();
    assert_eq!(sql, "DELETE ONLY person:one");
}

#[test]
fn only_with_return_before() {
    let sql = QueryBuilder::delete("person:one")
        .only()
        .return_before()
        .build();
    assert_eq!(sql, "DELETE ONLY person:one RETURN BEFORE");
}

#[test]
fn only_with_return_after() {
    let sql = QueryBuilder::delete("person:one")
        .only()
        .return_after()
        .build();
    assert_eq!(sql, "DELETE ONLY person:one RETURN AFTER");
}

#[test]
fn only_without_return_generates_query_without_validation() {
    // SurrealDB may error at runtime when ONLY is used without a single-result RETURN,
    // but the builder should still generate the query and leave validation to the server.
    let sql = QueryBuilder::delete("person:one").only().build();
    assert_eq!(sql, "DELETE ONLY person:one");
}

// --- RETURN clause tests ---

#[test]
fn return_none_clause() {
    let sql = QueryBuilder::delete("users").return_none().build();
    assert_eq!(sql, "DELETE FROM users RETURN NONE");
}

#[test]
fn return_before_clause() {
    let sql = QueryBuilder::delete("users").return_before().build();
    assert_eq!(sql, "DELETE FROM users RETURN BEFORE");
}

#[test]
fn return_after_clause() {
    let sql = QueryBuilder::delete("users").return_after().build();
    assert_eq!(sql, "DELETE FROM users RETURN AFTER");
}

#[test]
fn return_diff_clause() {
    let sql = QueryBuilder::delete("users").return_diff().build();
    assert_eq!(sql, "DELETE FROM users RETURN DIFF");
}

#[test]
fn return_params_with_multiple_fields() {
    let sql = QueryBuilder::delete("users")
        .return_params(vec!["$before", "$after"])
        .build();
    assert_eq!(sql, "DELETE FROM users RETURN $before, $after");
}

#[test]
fn return_params_with_single_field() {
    let sql = QueryBuilder::delete("users")
        .return_params(vec!["name"])
        .build();
    assert_eq!(sql, "DELETE FROM users RETURN name");
}

#[test]
fn return_params_accepts_owned_strings() {
    let sql = QueryBuilder::delete("users")
        .return_params(vec!["id".to_string(), "email".to_string()])
        .build();
    assert_eq!(sql, "DELETE FROM users RETURN id, email");
}

// --- TIMEOUT clause tests ---

#[test]
fn timeout_with_seconds() {
    let sql = QueryBuilder::delete("users").timeout("2s").build();
    assert_eq!(sql, "DELETE FROM users TIMEOUT 2s");
}

#[test]
fn timeout_with_milliseconds() {
    let sql = QueryBuilder::delete("users").timeout("500ms").build();
    assert_eq!(sql, "DELETE FROM users TIMEOUT 500ms");
}

#[test]
fn timeout_with_minutes() {
    let sql = QueryBuilder::delete("users").timeout("1m").build();
    assert_eq!(sql, "DELETE FROM users TIMEOUT 1m");
}

// --- EXPLAIN clause tests ---

#[test]
fn explain_simple() {
    let sql = QueryBuilder::delete("users").explain().build();
    assert_eq!(sql, "DELETE FROM users EXPLAIN");
}

#[test]
fn explain_full() {
    let sql = QueryBuilder::delete("users").explain_full().build();
    assert_eq!(sql, "DELETE FROM users EXPLAIN FULL");
}

// --- Combined / chained clause tests ---

#[test]
fn where_with_return_diff_timeout_and_explain_full() {
    let sql = QueryBuilder::delete("users")
        .r#where("active = false")
        .return_diff()
        .timeout("2s")
        .explain_full()
        .build();

    assert_eq!(
        sql,
        "DELETE FROM users WHERE active = false RETURN DIFF TIMEOUT 2s EXPLAIN FULL"
    );
}

#[test]
fn only_with_where_and_return_before() {
    let sql = QueryBuilder::delete("person:one")
        .only()
        .r#where("age > 18")
        .return_before()
        .build();

    assert_eq!(sql, "DELETE ONLY person:one WHERE age > 18 RETURN BEFORE");
}

#[test]
fn all_clauses_combined() {
    let sql = QueryBuilder::delete("logs")
        .only()
        .r#where("created_at < '2024-01-01'")
        .return_none()
        .timeout("5s")
        .explain_full()
        .build();

    assert_eq!(
        sql,
        "DELETE ONLY logs WHERE created_at < '2024-01-01' RETURN NONE TIMEOUT 5s EXPLAIN FULL"
    );
}

#[test]
fn where_and_timeout_without_return() {
    let sql = QueryBuilder::delete("sessions")
        .r#where("expired = true")
        .timeout("10s")
        .build();

    assert_eq!(sql, "DELETE FROM sessions WHERE expired = true TIMEOUT 10s");
}

#[test]
fn return_params_with_where_and_explain() {
    let sql = QueryBuilder::delete("orders")
        .r#where("status = 'cancelled'")
        .return_params(vec!["id", "status", "total"])
        .explain()
        .build();

    assert_eq!(
        sql,
        "DELETE FROM orders WHERE status = 'cancelled' RETURN id, status, total EXPLAIN"
    );
}

#[test]
fn complex_nested_conditions_with_return_and_timeout() {
    let nested = Condition::And(vec![
        Condition::Simple("active = false".into()),
        Condition::Or(vec![
            Condition::Simple("role = 'guest'".into()),
            Condition::Simple("last_login < '2023-01-01'".into()),
        ]),
    ]);

    let sql = QueryBuilder::delete("users")
        .r#where(nested)
        .return_before()
        .timeout("3s")
        .build();

    assert_eq!(
        sql,
        "DELETE FROM users WHERE (active = false AND (role = 'guest' OR last_login < '2023-01-01')) RETURN BEFORE TIMEOUT 3s"
    );
}

// --- Clause ordering tests ---

#[test]
fn clauses_are_emitted_in_correct_order_regardless_of_call_order() {
    // The builder should always emit: DELETE [FROM|ONLY] ... WHERE ... RETURN ... TIMEOUT ... EXPLAIN ...
    // regardless of the order the methods are called.
    let sql = QueryBuilder::delete("data")
        .timeout("1s")
        .explain_full()
        .return_after()
        .r#where("valid = true")
        .build();

    assert_eq!(
        sql,
        "DELETE FROM data WHERE valid = true RETURN AFTER TIMEOUT 1s EXPLAIN FULL"
    );
}

// --- Last-write-wins semantics ---

#[test]
fn calling_return_multiple_times_uses_last_value() {
    let sql = QueryBuilder::delete("users")
        .return_before()
        .return_after()
        .build();

    assert_eq!(sql, "DELETE FROM users RETURN AFTER");
}

#[test]
fn calling_explain_then_explain_full_uses_last_value() {
    let sql = QueryBuilder::delete("users")
        .explain()
        .explain_full()
        .build();

    assert_eq!(sql, "DELETE FROM users EXPLAIN FULL");
}

#[test]
fn calling_timeout_multiple_times_uses_last_value() {
    let sql = QueryBuilder::delete("users")
        .timeout("1s")
        .timeout("5s")
        .build();

    assert_eq!(sql, "DELETE FROM users TIMEOUT 5s");
}
