use surrealex::QueryBuilder;

#[test]
fn build_insert_into_target() {
    let sql = QueryBuilder::insert("person").build();
    assert_eq!(sql, "INSERT INTO person");
}

#[test]
fn build_insert_with_record_id() {
    let sql = QueryBuilder::insert("person:tobie").build();
    assert_eq!(sql, "INSERT INTO person:tobie");
}

#[test]
fn insert_relation() {
    let sql = QueryBuilder::insert("knows").relation().build();
    assert_eq!(sql, "INSERT RELATION INTO knows");
}

#[test]
fn insert_relation_with_content() {
    let sql = QueryBuilder::insert("knows")
        .relation()
        .content("{ in: person:tobie, out: person:jaime, since: '2024-01-01' }")
        .build();
    assert_eq!(
        sql,
        "INSERT RELATION INTO knows { in: person:tobie, out: person:jaime, since: '2024-01-01' }"
    );
}

#[test]
fn insert_ignore() {
    let sql = QueryBuilder::insert("person")
        .ignore()
        .content("{ id: 'tobie', name: 'Tobie' }")
        .build();
    assert_eq!(
        sql,
        "INSERT IGNORE INTO person { id: 'tobie', name: 'Tobie' }"
    );
}

#[test]
fn insert_relation_ignore() {
    let sql = QueryBuilder::insert("knows")
        .relation()
        .ignore()
        .content("{ in: person:tobie, out: person:jaime }")
        .build();
    assert_eq!(
        sql,
        "INSERT RELATION IGNORE INTO knows { in: person:tobie, out: person:jaime }"
    );
}

#[test]
fn content_with_object() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie', company: 'SurrealDB' }")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { name: 'Tobie', company: 'SurrealDB' }"
    );
}

#[test]
fn content_with_array_of_objects() {
    let sql = QueryBuilder::insert("person")
        .content("[{ name: 'Tobie' }, { name: 'Jaime' }]")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person [{ name: 'Tobie' }, { name: 'Jaime' }]"
    );
}

#[test]
fn content_with_nested_object() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie', settings: { theme: 'dark', lang: 'en' } }")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { name: 'Tobie', settings: { theme: 'dark', lang: 'en' } }"
    );
}

#[test]
fn content_with_array_field() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { name: 'Tobie', skills: ['Rust', 'Go', 'JavaScript'] }"
    );
}

#[test]
fn content_with_json_like_value() {
    let sql = QueryBuilder::insert("config")
        .content(r#"{ "key": "value", "nested": { "a": 1, "b": [2, 3] } }"#)
        .build();
    assert_eq!(
        sql,
        r#"INSERT INTO config { "key": "value", "nested": { "a": 1, "b": [2, 3] } }"#
    );
}

#[test]
fn calling_content_multiple_times_uses_last_value() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'first' }")
        .content("{ name: 'second' }")
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'second' }");
}

#[test]
fn fields_values_single_row() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .build();
    assert_eq!(sql, "INSERT INTO person (name, age) VALUES ('Tobie', 42)");
}

#[test]
fn fields_values_multiple_rows() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .values(vec!["'Jaime'", "35"])
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, age) VALUES ('Tobie', 42), ('Jaime', 35)"
    );
}

#[test]
fn fields_values_three_rows() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age", "active"])
        .values(vec!["'Tobie'", "42", "true"])
        .values(vec!["'Jaime'", "35", "true"])
        .values(vec!["'Alex'", "28", "false"])
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, age, active) VALUES ('Tobie', 42, true), ('Jaime', 35, true), ('Alex', 28, false)"
    );
}

#[test]
fn fields_accepts_owned_strings() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name".to_string(), "age".to_string()])
        .values(vec!["'Tobie'", "42"])
        .build();
    assert_eq!(sql, "INSERT INTO person (name, age) VALUES ('Tobie', 42)");
}

#[test]
fn values_accepts_owned_strings() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name"])
        .values(vec!["'Tobie'".to_string()])
        .build();
    assert_eq!(sql, "INSERT INTO person (name) VALUES ('Tobie')");
}

#[test]
fn content_after_fields_values_replaces_them() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name"])
        .values(vec!["'Tobie'"])
        .content("{ name: 'Jaime' }")
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Jaime' }");
}

#[test]
fn fields_after_content_replaces_content() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .fields(vec!["name"])
        .values(vec!["'Jaime'"])
        .build();
    assert_eq!(sql, "INSERT INTO person (name) VALUES ('Jaime')");
}

#[test]
fn on_duplicate_key_update_single_field() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .on_duplicate_key_update("age", "42")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, age) VALUES ('Tobie', 42) ON DUPLICATE KEY UPDATE age = 42"
    );
}

#[test]
fn on_duplicate_key_update_multiple_fields() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age", "active"])
        .values(vec!["'Tobie'", "42", "true"])
        .on_duplicate_key_update("age", "42")
        .on_duplicate_key_update("active", "true")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, age, active) VALUES ('Tobie', 42, true) ON DUPLICATE KEY UPDATE age = 42, active = true"
    );
}

#[test]
fn on_duplicate_key_update_with_content() {
    let sql = QueryBuilder::insert("person")
        .content("{ id: 'tobie', name: 'Tobie', age: 42 }")
        .on_duplicate_key_update("age", "42")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { id: 'tobie', name: 'Tobie', age: 42 } ON DUPLICATE KEY UPDATE age = 42"
    );
}

#[test]
fn on_duplicate_key_update_with_expression() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "visit_count"])
        .values(vec!["'Tobie'", "1"])
        .on_duplicate_key_update("visit_count", "$input.visit_count + visit_count")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, visit_count) VALUES ('Tobie', 1) ON DUPLICATE KEY UPDATE visit_count = $input.visit_count + visit_count"
    );
}

#[test]
fn return_none() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_none()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN NONE");
}

#[test]
fn return_before() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_before()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN BEFORE");
}

#[test]
fn return_after() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_after()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN AFTER");
}

#[test]
fn return_diff() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_diff()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN DIFF");
}

#[test]
fn return_params_with_multiple_fields() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie', age: 30 }")
        .return_params(vec!["name", "age"])
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { name: 'Tobie', age: 30 } RETURN name, age"
    );
}

#[test]
fn return_params_with_single_field() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_params(vec!["id"])
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN id");
}

#[test]
fn return_params_accepts_owned_strings() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_params(vec!["id".to_string(), "name".to_string()])
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN id, name");
}

#[test]
fn return_value_clause() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_value("name")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { name: 'Tobie' } RETURN VALUE name"
    );
}

#[test]
fn calling_return_multiple_times_uses_last_value() {
    let sql = QueryBuilder::insert("person")
        .content("{ name: 'Tobie' }")
        .return_before()
        .return_after()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN AFTER");
}

#[test]
fn relation_ignore_with_content_and_return() {
    let sql = QueryBuilder::insert("knows")
        .relation()
        .ignore()
        .content("{ in: person:tobie, out: person:jaime }")
        .return_after()
        .build();
    assert_eq!(
        sql,
        "INSERT RELATION IGNORE INTO knows { in: person:tobie, out: person:jaime } RETURN AFTER"
    );
}

#[test]
fn fields_values_with_on_duplicate_and_return() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .on_duplicate_key_update("age", "42")
        .return_none()
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (name, age) VALUES ('Tobie', 42) ON DUPLICATE KEY UPDATE age = 42 RETURN NONE"
    );
}

#[test]
fn ignore_with_fields_values_and_return_diff() {
    let sql = QueryBuilder::insert("person")
        .ignore()
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .return_diff()
        .build();
    assert_eq!(
        sql,
        "INSERT IGNORE INTO person (name, age) VALUES ('Tobie', 42) RETURN DIFF"
    );
}

#[test]
fn content_with_on_duplicate_and_return_value() {
    let sql = QueryBuilder::insert("person")
        .content("{ id: 'tobie', name: 'Tobie', age: 42 }")
        .on_duplicate_key_update("age", "42")
        .on_duplicate_key_update("name", "'Tobie Updated'")
        .return_value("id")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { id: 'tobie', name: 'Tobie', age: 42 } ON DUPLICATE KEY UPDATE age = 42, name = 'Tobie Updated' RETURN VALUE id"
    );
}

#[test]
fn all_clauses_combined_content_mode() {
    let sql = QueryBuilder::insert("knows")
        .relation()
        .ignore()
        .content("{ in: person:tobie, out: person:jaime }")
        .on_duplicate_key_update("updated_at", "time::now()")
        .return_after()
        .build();
    assert_eq!(
        sql,
        "INSERT RELATION IGNORE INTO knows { in: person:tobie, out: person:jaime } ON DUPLICATE KEY UPDATE updated_at = time::now() RETURN AFTER"
    );
}

#[test]
fn all_clauses_combined_fields_values_mode() {
    let sql = QueryBuilder::insert("person")
        .ignore()
        .fields(vec!["name", "age"])
        .values(vec!["'Tobie'", "42"])
        .values(vec!["'Jaime'", "35"])
        .on_duplicate_key_update("age", "$input.age")
        .return_params(vec!["name", "age"])
        .build();
    assert_eq!(
        sql,
        "INSERT IGNORE INTO person (name, age) VALUES ('Tobie', 42), ('Jaime', 35) ON DUPLICATE KEY UPDATE age = $input.age RETURN name, age"
    );
}

#[test]
fn clauses_are_emitted_in_correct_order_regardless_of_call_order() {
    let sql = QueryBuilder::insert("person")
        .return_after()
        .ignore()
        .content("{ name: 'Tobie' }")
        .build();
    assert_eq!(
        sql,
        "INSERT IGNORE INTO person { name: 'Tobie' } RETURN AFTER"
    );
}

#[test]
fn relation_and_ignore_order_does_not_matter() {
    let sql1 = QueryBuilder::insert("knows").relation().ignore().build();
    let sql2 = QueryBuilder::insert("knows").ignore().relation().build();
    assert_eq!(sql1, "INSERT RELATION IGNORE INTO knows");
    assert_eq!(sql2, "INSERT RELATION IGNORE INTO knows");
}

#[test]
fn insert_without_content() {
    let sql = QueryBuilder::insert("person").return_after().build();
    assert_eq!(sql, "INSERT INTO person RETURN AFTER");
}

#[test]
fn insert_with_only_on_duplicate_key_update() {
    let sql = QueryBuilder::insert("person")
        .content("{ id: 'tobie', name: 'Tobie' }")
        .on_duplicate_key_update("name", "'Tobie'")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person { id: 'tobie', name: 'Tobie' } ON DUPLICATE KEY UPDATE name = 'Tobie'"
    );
}

#[test]
fn insert_with_function_call_in_on_duplicate() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["id", "name"])
        .values(vec!["'tobie'", "'Tobie'"])
        .on_duplicate_key_update("updated_at", "time::now()")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO person (id, name) VALUES ('tobie', 'Tobie') ON DUPLICATE KEY UPDATE updated_at = time::now()"
    );
}

#[test]
fn insert_with_single_field_single_value() {
    let sql = QueryBuilder::insert("person")
        .fields(vec!["name"])
        .values(vec!["'Tobie'"])
        .build();
    assert_eq!(sql, "INSERT INTO person (name) VALUES ('Tobie')");
}

#[test]
fn insert_with_complex_target() {
    let sql = QueryBuilder::insert("person:ulid()")
        .content("{ name: 'Generated' }")
        .build();
    assert_eq!(sql, "INSERT INTO person:ulid() { name: 'Generated' }");
}

#[test]
fn insert_with_record_link_in_value() {
    let sql = QueryBuilder::insert("post")
        .content("{ title: 'My Post', author: person:tobie }")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO post { title: 'My Post', author: person:tobie }"
    );
}

#[test]
fn insert_with_subquery_in_on_duplicate() {
    let sql = QueryBuilder::insert("stats")
        .content("{ id: 'global', total: 1 }")
        .on_duplicate_key_update("total", "(SELECT count() FROM events GROUP ALL)")
        .build();
    assert_eq!(
        sql,
        "INSERT INTO stats { id: 'global', total: 1 } ON DUPLICATE KEY UPDATE total = (SELECT count() FROM events GROUP ALL)"
    );
}

#[test]
fn versioned_builder_insert() {
    let sql = surrealex::QueryBuilder::with_version(surrealex::SurrealV2)
        .insert("person")
        .content("{ name: 'Tobie' }")
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' }");
}

#[test]
fn versioned_builder_insert_v1() {
    let sql = surrealex::QueryBuilder::with_version(surrealex::SurrealV1)
        .insert("person")
        .content("{ name: 'Tobie' }")
        .return_after()
        .build();
    assert_eq!(sql, "INSERT INTO person { name: 'Tobie' } RETURN AFTER");
}
