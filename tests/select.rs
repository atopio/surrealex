use surrealex::enums::{Condition, Direction, Sort};
use surrealex::types::select::GraphTraversalParams;
use surrealex::{QueryBuilder, SurrealV1};

#[test]
fn select_single_field_from_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("table")
        .build();
    assert_eq!(sql, "SELECT id FROM table");
}

#[test]
fn select_multiple_fields_from_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id", "name"))
        .from("users")
        .build();
    assert_eq!(sql, "SELECT id, name FROM users");
}

#[test]
fn select_with_aliases_and_limit_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(("id", "i"), ("name", "n")))
        .from("users")
        .limit(10)
        .build();
    assert_eq!(sql, "SELECT id AS i, name AS n FROM users LIMIT 10");
}

#[test]
fn select_only_star_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .from("posts")
        .build();
    assert_eq!(sql, "SELECT * FROM posts");
}

#[test]
fn select_from_then_limit_chaining_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .limit(5)
        .build();
    assert_eq!(sql, "SELECT id FROM t LIMIT 5");
}

#[test]
fn select_single_field_from_only_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from_only("table")
        .build();
    assert_eq!(sql, "SELECT id FROM ONLY table");
}

#[test]
fn select_from_only_then_limit_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from_only("t")
        .limit(3)
        .build();
    assert_eq!(sql, "SELECT id FROM ONLY t LIMIT 3");
}

#[test]
fn graph_traverse_with_alias_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("friends")
                .step_in("posts")
                .fields(surrealex::fields!(*))
                .alias("friend_posts"),
        )
        .from("user")
        .build();

    // graph traversal expands to ->friends<-posts.* and gets aliased
    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS friend_posts FROM user"
    );
}

#[test]
fn graph_traverse_without_alias_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("name"))
        .graph_traverse(
            GraphTraversalParams::start_in("t")
                .step_out("e")
                .fields(surrealex::fields!(*)),
        )
        .from("x")
        .build();

    // graph traversal with directions produces <-t->e.* without alias
    assert_eq!(sql, "SELECT name, <-t->e.* FROM x");
}

#[test]
fn where_simple_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .r#where("age > 18")
        .build();
    assert_eq!(sql, "SELECT id FROM users WHERE age > 18");
}

#[test]
fn where_chaining_multiple_times_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .r#where("age > 18")
        .r#where("active = true")
        .build();
    assert_eq!(sql, "SELECT id FROM users WHERE age > 18 AND active = true");
}

#[test]
fn complex_where_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .r#where(Condition::new("a = 1").and(Condition::new("b = 2").or("c = 3")))
        .build();
    assert_eq!(sql, "SELECT id FROM t WHERE (a = 1 AND (b = 2 OR c = 3))");
}

#[test]
fn very_complex_where_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .r#where(
            Condition::new("a = 1").and(
                Condition::new("b = 2").or(Condition::new("c = 3").and(
                    Condition::new("d = 4")
                        .or(Condition::new("e = 5").and(Condition::new("f = 6").or("g = 7"))),
                )),
            ),
        )
        .build();
    assert_eq!(
        sql,
        "SELECT id FROM t WHERE (a = 1 AND (b = 2 OR (c = 3 AND (d = 4 OR (e = 5 AND (f = 6 OR g = 7))))))"
    );
}

#[test]
fn fetch_single_field_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("post")
        .fetch(vec!["comments"])
        .build();
    assert_eq!(sql, "SELECT id FROM post FETCH comments");
}

#[test]
fn fetch_multiple_fields_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .from("tbl")
        .fetch(vec!["a", "b"])
        .build();
    assert_eq!(sql, "SELECT * FROM tbl FETCH a, b");
}

#[test]
fn fetch_with_graph_and_where_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("friends")
                .step_in("posts")
                .fields(surrealex::fields!(*))
                .alias("friend_posts"),
        )
        .from("user")
        .r#where("active = true")
        .fetch(vec!["friend_posts"]) // fetch the aliased expansion
        .build();

    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS friend_posts FROM user WHERE active = true FETCH friend_posts"
    );
}

#[test]
fn order_by_asc_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .order_by("name", Sort::Asc)
        .build();
    assert_eq!(sql, "SELECT id FROM t ORDER BY name ASC");
}

#[test]
fn order_by_desc_numeric_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .from("scores")
        .order_by("score", Sort::Desc.numeric())
        .build();
    assert_eq!(sql, "SELECT * FROM scores ORDER BY score NUMERIC DESC");
}

#[test]
fn order_by_multiple_terms_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .order_by("a", Sort::Asc.collate())
        .order_by("b", Sort::Desc.numeric())
        .build();
    assert_eq!(
        sql,
        "SELECT id FROM t ORDER BY a COLLATE ASC, b NUMERIC DESC"
    );
}

#[test]
fn order_by_collate_and_numeric_chained_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .order_by("a", Sort::Asc.collate().numeric())
        .order_by("b", Sort::Desc.numeric())
        .build();
    assert_eq!(
        sql,
        "SELECT id FROM t ORDER BY a COLLATE NUMERIC ASC, b NUMERIC DESC"
    );
}

#[test]
fn order_by_then_order_random_overrides_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .order_by("name", Sort::Asc)
        .order_random()
        .build();
    assert_eq!(sql, "SELECT id FROM t ORDER BY RAND()");
}

#[test]
fn order_random_then_order_by_keeps_both_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("t")
        .order_random()
        .order_by("name", Sort::Asc)
        .build();
    assert_eq!(sql, "SELECT id FROM t ORDER BY RAND(), name ASC");
}

#[test]
fn order_random_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("r")
        .order_random()
        .build();
    assert_eq!(sql, "SELECT id FROM r ORDER BY RAND()");
}

#[test]
fn start_at_basic_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("items")
        .start_at(10)
        .build();
    assert_eq!(sql, "SELECT id FROM items START AT 10");
}

#[test]
fn start_at_with_limit_order_and_fetch_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .order_by("name", Sort::Asc)
        .limit(5)
        .start_at(10)
        .fetch(vec!["profile"])
        .build();
    assert_eq!(
        sql,
        "SELECT id FROM users ORDER BY name ASC LIMIT 5 START AT 10 FETCH profile"
    );
}

#[test]
fn multi_graph_traverse_mixed_fields_builds() {
    let sql = QueryBuilder::select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("friends")
                .step(Direction::In, "posts")
                .fields(surrealex::fields!(*))
                .alias("fp"),
        )
        .graph_traverse(
            GraphTraversalParams::start_out("related")
                .step_in("items")
                .fields(surrealex::fields!(("title", "t"), "count", ("meta", "m")))
                .alias("related_items"),
        )
        .from("user")
        .build();

    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS fp, ->related<-items.{title AS t, count, meta AS m} AS related_items FROM user"
    );
}

#[test]
fn multi_graph_traverse_nested_and_aliases_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .graph_traverse(
            GraphTraversalParams::start_out("a")
                .step_out("b")
                .fields(surrealex::fields!(("x", "x_alias"), ("y", "y_alias")))
                .alias("ab"),
        )
        .graph_traverse(
            GraphTraversalParams::start_in("c")
                .step_out("d")
                .fields(surrealex::fields!(*))
                .alias("cd"),
        )
        .from("root")
        .build();

    assert_eq!(
        sql,
        "SELECT id, ->a->b.{x AS x_alias, y AS y_alias} AS ab, <-c->d.* AS cd FROM root"
    );
}

#[test]
fn v1_graph_traverse_star_with_alias_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("friends")
                .step_in("posts")
                .fields(surrealex::fields!(*))
                .alias("friend_posts"),
        )
        .from("user")
        .build();

    // SelectionFields::All is rendered the same in V1 and V2
    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS friend_posts FROM user"
    );
}

#[test]
fn v1_graph_traverse_star_without_alias_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!("name"))
        .graph_traverse(
            GraphTraversalParams::start_in("t")
                .step_out("e")
                .fields(surrealex::fields!(*)),
        )
        .from("x")
        .build();

    assert_eq!(sql, "SELECT name, <-t->e.* FROM x");
}

#[test]
fn v1_graph_traverse_fields_expand_separately_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("wrote")
                .step_out("book")
                .fields(surrealex::fields!("name", "id")),
        )
        .from("users")
        .build();

    // V1: each field gets its own path prefix instead of {name, id}
    assert_eq!(
        sql,
        "SELECT *, ->wrote->book.name, ->wrote->book.id FROM users"
    );
}

#[test]
fn v1_graph_traverse_fields_with_inner_aliases_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!("id"))
        .graph_traverse(
            GraphTraversalParams::start_out("a")
                .step_out("b")
                .fields(surrealex::fields!(("x", "x_alias"), ("y", "y_alias"))),
        )
        .from("root")
        .build();

    // V1: inner aliases are preserved on each expanded field
    assert_eq!(
        sql,
        "SELECT id, ->a->b.x AS x_alias, ->a->b.y AS y_alias FROM root"
    );
}

#[test]
fn v1_multi_graph_traverse_mixed_fields_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("friends")
                .step(Direction::In, "posts")
                .fields(surrealex::fields!(*))
                .alias("fp"),
        )
        .graph_traverse(
            GraphTraversalParams::start_out("related")
                .step_in("items")
                .fields(surrealex::fields!(("title", "t"), "count", ("meta", "m"))),
        )
        .from("user")
        .build();

    // First traversal uses All so it's the same; second expands fields separately
    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS fp, ->related<-items.title AS t, ->related<-items.count, ->related<-items.meta AS m FROM user"
    );
}

#[test]
fn v1_graph_traverse_single_field_builds() {
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!(*))
        .graph_traverse(
            GraphTraversalParams::start_out("likes")
                .step_out("cat")
                .fields(surrealex::fields!("name")),
        )
        .from("users")
        .build();

    assert_eq!(sql, "SELECT *, ->likes->cat.name FROM users");
}

#[test]
fn v1_default_select_unchanged() {
    // Non-graph queries are identical across versions
    let sql = QueryBuilder::with_version(SurrealV1)
        .select(surrealex::fields!("id", "name"))
        .from("users")
        .r#where("age > 18")
        .limit(10)
        .build();

    assert_eq!(sql, "SELECT id, name FROM users WHERE age > 18 LIMIT 10");
}

#[test]
fn select_subquery_field_builds() {
    let sub = QueryBuilder::select(surrealex::fields!("id")).from("users");

    let sql = QueryBuilder::select(surrealex::fields!(sub))
        .from("t")
        .build();

    assert_eq!(sql, "SELECT (SELECT id FROM users) FROM t");
}

#[test]
fn select_subquery_field_with_alias_builds() {
    let sub = QueryBuilder::select(surrealex::fields!("id", "name")).from("accounts");

    let sql = QueryBuilder::select(surrealex::fields!((sub, "acct")))
        .from("log")
        .build();

    assert_eq!(
        sql,
        "SELECT (SELECT id, name FROM accounts) AS acct FROM log"
    );
}

#[test]
fn select_subquery_with_where_and_outer_other_field_builds() {
    let sub = QueryBuilder::select(surrealex::fields!("count"))
        .from("visits")
        .r#where("page = 'home'");

    let sql = QueryBuilder::select(surrealex::fields!("url", sub))
        .from("pages")
        .build();

    assert_eq!(
        sql,
        "SELECT url, (SELECT count FROM visits WHERE page = 'home') FROM pages"
    );
}

#[test]
fn select_star_and_subquery_field_builds() {
    let sub = QueryBuilder::select(surrealex::fields!("recent"))
        .from("sessions")
        .order_by("ts", Sort::Desc)
        .limit(1);

    let sql = QueryBuilder::select(surrealex::fields!(*, sub))
        .from("users")
        .build();

    assert_eq!(
        sql,
        "SELECT *, (SELECT recent FROM sessions ORDER BY ts DESC LIMIT 1) FROM users"
    );
}

#[test]
fn explain_simple_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .explain()
        .build();
    assert_eq!(sql, "SELECT id FROM users EXPLAIN");
}

#[test]
fn explain_full_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .explain_full()
        .build();
    assert_eq!(sql, "SELECT id FROM users EXPLAIN FULL");
}

#[test]
fn where_order_limit_fetch_with_explain_full_builds() {
    let sql = QueryBuilder::select(surrealex::fields!("id"))
        .from("users")
        .r#where("active = true")
        .order_by("name", Sort::Asc)
        .limit(10)
        .start_at(5)
        .fetch(vec!["profile"])
        .explain_full()
        .build();

    assert_eq!(
        sql,
        "SELECT id FROM users WHERE active = true ORDER BY name ASC LIMIT 10 START AT 5 FETCH profile EXPLAIN FULL"
    );
}
