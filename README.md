# Surrealex

Dead simple SurrealDB query generator.

A Rust library for building SurrealQL queries with a fluent, type-safe API.

## 🌟 Features

- Fluent builder API with compile-time state checking
- Type-safe field selection using the `fields!` macro
- Complex WHERE conditions and graph traversal support
- Support for `SELECT`, `FROM`, `WHERE`, `FETCH`, `ORDER BY`, `LIMIT`, and `START AT`

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
    .order_by("created_at", Sort::Desc, false, false)
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
use surrealex::{enums::Direction, structs::GraphExpandParams};

let query = QueryBuilder::select(surrealex::fields!(*))
    .graph_traverse(GraphExpandParams {
        from: (Direction::Out, "friends".into()),
        to: (Direction::In, "posts".into()),
        alias: Some("friend_posts".into()),
        fields: surrealex::fields!(*),
    })
    .from("user")
    .build();

assert_eq!(
    query,
    "SELECT *, ->friends<-posts.* AS friend_posts FROM user"
);
```

## 📝 License

This project is open source. See the repository for license details.

## 🔗 Links

- Repository: https://github.com/MordechaiHadad/surrealex
- SurrealDB: https://surrealdb.com