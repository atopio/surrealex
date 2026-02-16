# Surrealex

Dead simple SurrealDB query generator.

A Rust library for building SurrealQL queries with a fluent, type-safe API.

## 🌟 Features

- Fluent builder API with compile-time state checking
- Type-safe field selection using the `fields!` macro
- Complex WHERE conditions and graph traversal support
- Support for `SELECT`, `FROM`, `WHERE`, `FETCH`, `ORDER BY`, `LIMIT`, and `START AT`
- Full `DELETE` statement support with `ONLY`, `RETURN`, `TIMEOUT`, and `EXPLAIN` clauses

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
surrealex = { version = "0.1.0", features = ["macros"] }
```

Or from Git:

```toml
[dependencies]
surrealex = { git = "https://github.com/MordechaiHadad/surrealex", features = ["macros"] }
```

## 🔧 Usage

### Basic Query

```rust
use surrealex::QueryBuilder;

let query = QueryBuilder::select(surrealex::fields!("id", "name"))
    .from("users")
    .r#where("age > 18")
    .order_by("created_at", Sort::Desc)
    .limit(10)
    .build();

assert_eq!(query, "SELECT id, name FROM users WHERE age > 18 ORDER BY created_at DESC LIMIT 10");
```

### Complex WHERE Conditions

```rust
use surrealex::enums::Condition;

let query = QueryBuilder::select(surrealex::fields!("id"))
    .from("users")
    .r#where(
        Condition::new("age > 18")
        .and(Condition::new("status = 'active'").or("status = 'pending'"))
    )
    .build();

assert_eq!(
    query,
    "SELECT id FROM users WHERE (age > 18 AND (status = 'active' OR status = 'pending'))"
);
```

### Graph Traversal

```rust
use surrealex::types::select::GraphTraversalParams;

let query = QueryBuilder::select(surrealex::fields!(*))
    .graph_traverse(
        GraphTraversalParams::start_out("friends")
            .step_in("posts")
            .fields(surrealex::fields!(*))
            .alias("friend_posts"),
    )
    .from("user")
    .build();

assert_eq!(
    query,
    "SELECT *, ->friends<-posts.* AS friend_posts FROM user"
);
```

### Versioning API (`with_version`)

`QueryBuilder::select(...)` uses `SurrealV2` by default.  
You can target a specific SurrealDB version with `QueryBuilder::with_version(...)`.

```rust
use surrealex::{QueryBuilder, SurrealV1};
use surrealex::enums::SelectionFields;

let query = QueryBuilder::with_version(SurrealV1)
    .select(SelectionFields::All)
    .graph_traverse(
        surrealex::types::select::GraphTraversalParams::start_out("friends")
            .step_in("posts")
            .fields(surrealex::fields!("title", "created_at"))
            .alias("friend_posts"),
    )
    .from("user")
    .build();

assert_eq!(
    query,
    "SELECT *, ->friends<-posts.title, ->friends<-posts.created_at AS friend_posts FROM user"
);
```

`SurrealV2` and `SurrealV3` use object destructuring for graph traversal field selection:

```rust
use surrealex::{QueryBuilder, SurrealV3};
use surrealex::enums::SelectionFields;
use surrealex::types::select::GraphTraversalParams;

let query = QueryBuilder::with_version(SurrealV3)
    .select(SelectionFields::All)
    .graph_traverse(
        GraphTraversalParams::start_out("friends")
            .step_in("posts")
            .fields(surrealex::fields!("title", "created_at"))
            .alias("friend_posts"),
    )
    .from("user")
    .build();

assert_eq!(
    query,
    "SELECT *, ->friends<-posts.{title, created_at} AS friend_posts FROM user"
);
```

### Delete Statement

```rust
use surrealex::QueryBuilder;

// Basic delete
let query = QueryBuilder::delete("users")
    .r#where("active = false")
    .build();

assert_eq!(query, "DELETE FROM users WHERE active = false");
```

#### DELETE ONLY with RETURN

When using `ONLY`, SurrealDB expects a single-result `RETURN` clause. The builder generates the query and leaves validation to the server.

```rust
use surrealex::QueryBuilder;

let query = QueryBuilder::delete("person:one")
    .only()
    .return_before()
    .build();

assert_eq!(query, "DELETE ONLY person:one RETURN BEFORE");
```

#### RETURN Variants

```rust
use surrealex::QueryBuilder;

// RETURN NONE / BEFORE / AFTER / DIFF
let query = QueryBuilder::delete("users")
    .r#where("expired = true")
    .return_diff()
    .build();

assert_eq!(query, "DELETE FROM users WHERE expired = true RETURN DIFF");

// RETURN specific fields
let query = QueryBuilder::delete("users")
    .return_params(vec!["$before", "$after"])
    .build();

assert_eq!(query, "DELETE FROM users RETURN $before, $after");
```

#### TIMEOUT and EXPLAIN

```rust
use surrealex::QueryBuilder;

let query = QueryBuilder::delete("logs")
    .r#where("created_at < '2024-01-01'")
    .return_none()
    .timeout("5s")
    .explain_full()
    .build();

assert_eq!(
    query,
    "DELETE FROM logs WHERE created_at < '2024-01-01' RETURN NONE TIMEOUT 5s EXPLAIN FULL"
);
```

## 📝 License

This project is open source. See the repository for license details.

## 🔗 Links

- Repository: https://github.com/MordechaiHadad/surrealex
- SurrealDB: https://surrealdb.com
