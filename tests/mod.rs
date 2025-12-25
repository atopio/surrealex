use surrealex::enums::{Condition, Direction, Sort};
use surrealex::{GraphExpandParams, QueryBuilder};

#[test]
fn select_single_field_from_builds() {
    let sql = QueryBuilder::select(vec!["id"]).from("table").build();
    assert_eq!(sql, "SELECT id FROM table");
}

#[test]
fn select_multiple_fields_from_builds() {
    let sql = QueryBuilder::select(vec!["id", "name"])
        .from("users")
        .build();
    assert_eq!(sql, "SELECT id, name FROM users");
}

#[test]
fn select_with_aliases_and_limit_builds() {
    let sql = QueryBuilder::select(vec![("id", "i"), ("name", "n")])
        .from("users")
        .limit(10)
        .build();
    assert_eq!(sql, "SELECT id AS i, name AS n FROM users LIMIT 10");
}

#[test]
fn select_only_star_builds() {
    let sql = QueryBuilder::select(vec!["*"]).from("posts").build();
    assert_eq!(sql, "SELECT * FROM posts");
}

#[test]
fn select_from_then_limit_chaining_builds() {
    // limit is available after calling `from` (FromReady state)
    let sql = QueryBuilder::select(vec!["id"]).from("t").limit(5).build();
    assert_eq!(sql, "SELECT id FROM t LIMIT 5");
}

#[test]
fn select_single_field_from_only_builds() {
    let sql = QueryBuilder::select(vec!["id"]).from_only("table").build();
    assert_eq!(sql, "SELECT id FROM ONLY table");
}

#[test]
fn select_from_only_then_limit_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from_only("t")
        .limit(3)
        .build();
    assert_eq!(sql, "SELECT id FROM ONLY t LIMIT 3");
}

#[test]
fn graph_traverse_with_alias_builds() {
    let sql = QueryBuilder::select(vec!["*"])
        .graph_traverse(GraphExpandParams {
            from: (Direction::Out, "friends".into()),
            to: (Direction::In, "posts".into()),
            alias: Some("friend_posts".into()),
        })
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
    let sql = QueryBuilder::select(vec!["name"])
        .graph_traverse(GraphExpandParams {
            from: (Direction::In, "t".into()),
            to: (Direction::Out, "e".into()),
            alias: None,
        })
        .from("x")
        .build();

    // graph traversal with directions produces <-t->e.* without alias
    assert_eq!(sql, "SELECT name, <-t->e.* FROM x");
}

#[test]
fn where_simple_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("users")
        .r#where("age > 18")
        .build();
    assert_eq!(sql, "SELECT id FROM users WHERE age > 18");
}

#[test]
fn complex_where_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("t")
        .r#where(Condition::And(vec![
            Condition::Simple("a = 1".into()),
            Condition::Or(vec![
                Condition::Simple("b = 2".into()),
                Condition::Simple("c = 3".into()),
            ]),
        ]))
        .build();
    assert_eq!(sql, "SELECT id FROM t WHERE (a = 1 AND (b = 2 OR c = 3))");
}

#[test]
fn fetch_single_field_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("post")
        .fetch(vec!["comments"])
        .build();
    assert_eq!(sql, "SELECT id FROM post FETCH comments");
}

#[test]
fn fetch_multiple_fields_builds() {
    let sql = QueryBuilder::select(vec!["*"])
        .from("tbl")
        .fetch(vec!["a", "b"])
        .build();
    assert_eq!(sql, "SELECT * FROM tbl FETCH a, b");
}

#[test]
fn fetch_with_graph_and_where_builds() {
    let sql = QueryBuilder::select(vec!["*"])
        .graph_traverse(GraphExpandParams {
            from: (Direction::Out, "friends".into()),
            to: (Direction::In, "posts".into()),
            alias: Some("friend_posts".into()),
        })
        .from("user")
        .r#where("active = true")
        .fetch(vec!["friend_posts"]) // fetch the aliased expansion
        .build();

    assert_eq!(
        sql,
        "SELECT *, ->friends<-posts.* AS friend_posts FROM user WHERE active = true FETCH friend_posts"
    );
}

//
// ORDER BY tests
//
#[test]
fn order_by_asc_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("t")
        .order_by("name", Sort::Asc, false)
        .build();
    assert_eq!(sql, "SELECT id FROM t ORDER BY name ASC");
}

#[test]
fn order_by_desc_numeric_builds() {
    let sql = QueryBuilder::select(vec!["*"])
        .from("scores")
        .order_by("score", Sort::Desc, true)
        .build();
    assert_eq!(sql, "SELECT * FROM scores ORDER BY score DESC NUMERIC");
}

#[test]
fn order_by_multiple_terms_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("t")
        .order_by("a", Sort::Asc, false)
        .order_by("b", Sort::Desc, true)
        .build();
    assert_eq!(sql, "SELECT id FROM t ORDER BY a ASC, b DESC NUMERIC");
}

#[test]
fn order_random_builds() {
    let sql = QueryBuilder::select(vec!["id"])
        .from("r")
        .order_random()
        .build();
    assert_eq!(sql, "SELECT id FROM r ORDER BY RAND()");
}
