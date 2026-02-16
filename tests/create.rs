use surrealex::QueryBuilder;

#[test]
fn build_create_targets() {
    let sql = QueryBuilder::create("person").build();
    assert_eq!(sql, "CREATE person");
}

#[test]
fn build_create_with_record_id() {
    let sql = QueryBuilder::create("person:tobie").build();
    assert_eq!(sql, "CREATE person:tobie");
}

#[test]
fn only_emits_create_only() {
    let sql = QueryBuilder::create("person:tobie").only().build();
    assert_eq!(sql, "CREATE ONLY person:tobie");
}

#[test]
fn only_with_return_after() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .return_after()
        .build();
    assert_eq!(sql, "CREATE ONLY person:tobie RETURN AFTER");
}

#[test]
fn only_without_return_generates_query_without_validation() {
    let sql = QueryBuilder::create("person:one").only().build();
    assert_eq!(sql, "CREATE ONLY person:one");
}

#[test]
fn content_with_object() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie', company: 'SurrealDB' }")
        .build();
    assert_eq!(
        sql,
        "CREATE person CONTENT { name: 'Tobie', company: 'SurrealDB' }"
    );
}

#[test]
fn content_with_nested_object() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie', settings: { theme: 'dark', lang: 'en' } }")
        .build();
    assert_eq!(
        sql,
        "CREATE person CONTENT { name: 'Tobie', settings: { theme: 'dark', lang: 'en' } }"
    );
}

#[test]
fn content_with_array_field() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }")
        .build();
    assert_eq!(
        sql,
        "CREATE person CONTENT { name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }"
    );
}

#[test]
fn only_with_content() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .content("{ name: 'Tobie' }")
        .build();
    assert_eq!(sql, "CREATE ONLY person:tobie CONTENT { name: 'Tobie' }");
}

#[test]
fn set_single_field() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie'");
}

#[test]
fn set_multiple_fields() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .set("company", "'SurrealDB'")
        .set("skills", "['Rust', 'Go', 'JavaScript']")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript']"
    );
}

#[test]
fn set_with_numeric_value() {
    let sql = QueryBuilder::create("product")
        .set("name", "'Widget'")
        .set("price", "9.99")
        .set("quantity", "100")
        .build();
    assert_eq!(
        sql,
        "CREATE product SET name = 'Widget', price = 9.99, quantity = 100"
    );
}

#[test]
fn set_with_nested_field() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .set("settings.theme", "'dark'")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie', settings.theme = 'dark'"
    );
}

#[test]
fn only_with_set() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .set("name", "'Tobie'")
        .set("company", "'SurrealDB'")
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB'"
    );
}

#[test]
fn set_after_content_replaces_content() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .set("name", "'Jaime'")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Jaime'");
}

#[test]
fn content_after_set_replaces_set() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .content("{ name: 'Jaime' }")
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Jaime' }");
}

#[test]
fn return_none_clause() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .return_none()
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } RETURN NONE");
}

#[test]
fn return_before_clause() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .return_before()
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } RETURN BEFORE");
}

#[test]
fn return_after_clause() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .return_after()
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } RETURN AFTER");
}

#[test]
fn return_diff_clause() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .return_diff()
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } RETURN DIFF");
}

#[test]
fn return_params_with_multiple_fields() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .set("age", "30")
        .return_params(vec!["name", "age"])
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie', age = 30 RETURN name, age"
    );
}

#[test]
fn return_params_with_single_field() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_params(vec!["id"])
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN id");
}

#[test]
fn return_params_accepts_owned_strings() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_params(vec!["id".to_string(), "name".to_string()])
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN id, name");
}

#[test]
fn return_value_clause() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_value("name")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN VALUE name");
}

#[test]
fn return_value_with_content() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie', age: 30 }")
        .return_value("id")
        .build();
    assert_eq!(
        sql,
        "CREATE person CONTENT { name: 'Tobie', age: 30 } RETURN VALUE id"
    );
}

#[test]
fn timeout_with_seconds() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie' }")
        .timeout("2s")
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'Tobie' } TIMEOUT 2s");
}

#[test]
fn timeout_with_milliseconds() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .timeout("500ms")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' TIMEOUT 500ms");
}

#[test]
fn timeout_with_minutes() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .timeout("1m")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' TIMEOUT 1m");
}

#[test]
fn content_with_return_and_timeout() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'Tobie', company: 'SurrealDB' }")
        .return_after()
        .timeout("5s")
        .build();
    assert_eq!(
        sql,
        "CREATE person CONTENT { name: 'Tobie', company: 'SurrealDB' } RETURN AFTER TIMEOUT 5s"
    );
}

#[test]
fn set_with_return_diff_and_timeout() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .set("age", "30")
        .return_diff()
        .timeout("2s")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie', age = 30 RETURN DIFF TIMEOUT 2s"
    );
}

#[test]
fn only_with_content_and_return_none() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .content("{ name: 'Tobie' }")
        .return_none()
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:tobie CONTENT { name: 'Tobie' } RETURN NONE"
    );
}

#[test]
fn only_with_set_return_value_and_timeout() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .set("name", "'Tobie'")
        .set("company", "'SurrealDB'")
        .return_value("name")
        .timeout("3s")
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB' RETURN VALUE name TIMEOUT 3s"
    );
}

#[test]
fn all_clauses_combined_with_content() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .content("{ name: 'Tobie', company: 'SurrealDB', skills: ['Rust', 'Go'] }")
        .return_after()
        .timeout("10s")
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:tobie CONTENT { name: 'Tobie', company: 'SurrealDB', skills: ['Rust', 'Go'] } RETURN AFTER TIMEOUT 10s"
    );
}

#[test]
fn all_clauses_combined_with_set() {
    let sql = QueryBuilder::create("person:tobie")
        .only()
        .set("name", "'Tobie'")
        .set("company", "'SurrealDB'")
        .set("skills", "['Rust', 'Go']")
        .return_before()
        .timeout("10s")
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go'] RETURN BEFORE TIMEOUT 10s"
    );
}

#[test]
fn return_params_with_timeout() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_params(vec!["id", "name"])
        .timeout("1s")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie' RETURN id, name TIMEOUT 1s"
    );
}

#[test]
fn return_value_with_timeout() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_value("id")
        .timeout("1s")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie' RETURN VALUE id TIMEOUT 1s"
    );
}

#[test]
fn clauses_are_emitted_in_correct_order_regardless_of_call_order() {
    // The builder should always emit: CREATE [ONLY] ... [CONTENT|SET] ... RETURN ... TIMEOUT ...
    // regardless of the order the methods are called.
    let sql = QueryBuilder::create("person")
        .timeout("1s")
        .return_after()
        .set("name", "'Tobie'")
        .build();
    assert_eq!(
        sql,
        "CREATE person SET name = 'Tobie' RETURN AFTER TIMEOUT 1s"
    );
}

#[test]
fn only_and_timeout_set_before_content() {
    let sql = QueryBuilder::create("person:one")
        .only()
        .timeout("2s")
        .return_diff()
        .content("{ name: 'Test' }")
        .build();
    assert_eq!(
        sql,
        "CREATE ONLY person:one CONTENT { name: 'Test' } RETURN DIFF TIMEOUT 2s"
    );
}

#[test]
fn calling_return_multiple_times_uses_last_value() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .return_before()
        .return_after()
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' RETURN AFTER");
}

#[test]
fn calling_timeout_multiple_times_uses_last_value() {
    let sql = QueryBuilder::create("person")
        .set("name", "'Tobie'")
        .timeout("1s")
        .timeout("5s")
        .build();
    assert_eq!(sql, "CREATE person SET name = 'Tobie' TIMEOUT 5s");
}

#[test]
fn calling_content_multiple_times_uses_last_value() {
    let sql = QueryBuilder::create("person")
        .content("{ name: 'first' }")
        .content("{ name: 'second' }")
        .build();
    assert_eq!(sql, "CREATE person CONTENT { name: 'second' }");
}

#[test]
fn create_without_content_or_set() {
    let sql = QueryBuilder::create("person").return_after().build();
    assert_eq!(sql, "CREATE person RETURN AFTER");
}

#[test]
fn create_only_without_content_or_set() {
    let sql = QueryBuilder::create("person:one")
        .only()
        .return_none()
        .build();
    assert_eq!(sql, "CREATE ONLY person:one RETURN NONE");
}

#[test]
fn create_with_only_timeout() {
    let sql = QueryBuilder::create("person").timeout("3s").build();
    assert_eq!(sql, "CREATE person TIMEOUT 3s");
}

#[test]
fn set_with_function_call_value() {
    let sql = QueryBuilder::create("event")
        .set("created_at", "time::now()")
        .set("id", "rand::uuid()")
        .build();
    assert_eq!(
        sql,
        "CREATE event SET created_at = time::now(), id = rand::uuid()"
    );
}

#[test]
fn set_with_subquery_value() {
    let sql = QueryBuilder::create("stats")
        .set("total", "(SELECT count() FROM events GROUP ALL)")
        .build();
    assert_eq!(
        sql,
        "CREATE stats SET total = (SELECT count() FROM events GROUP ALL)"
    );
}

#[test]
fn set_with_record_link() {
    let sql = QueryBuilder::create("post")
        .set("title", "'My Post'")
        .set("author", "person:tobie")
        .build();
    assert_eq!(
        sql,
        "CREATE post SET title = 'My Post', author = person:tobie"
    );
}

#[test]
fn content_with_json_like_value() {
    let sql = QueryBuilder::create("config")
        .content(r#"{ "key": "value", "nested": { "a": 1, "b": [2, 3] } }"#)
        .build();
    assert_eq!(
        sql,
        r#"CREATE config CONTENT { "key": "value", "nested": { "a": 1, "b": [2, 3] } }"#
    );
}

#[test]
fn create_with_complex_target() {
    let sql = QueryBuilder::create("person:ulid()")
        .set("name", "'Generated'")
        .build();
    assert_eq!(sql, "CREATE person:ulid() SET name = 'Generated'");
}
