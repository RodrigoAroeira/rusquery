# rusquery

**rusquery** is a small, type-safe SQL query builder for Rust, focused on **clarity, composability, and zero runtime SQL generation magic**.

It provides a fluent API for constructing SQL queries while preserving the *kind* of query (SELECT, INSERT, UPDATE, etc.) at the type level.

> [!WARNING]
> This project is in an **early stage** and **unfit** for use in production. APIs may change.

---

## Goals

*  Type-safe query kinds (`SELECT`, `INSERT`, `UPDATE`, …)
*  Simple, readable SQL output
*  Queries are just strings when you’re done

Non-goals (at least for now):

*  Query execution
*  Automatic SQL escaping
*  Dialect-specific features
*  Full SQL coverage

---

## Design Overview

### Typed Queries

Each query carries a **phantom type** representing its SQL kind:

```rust
Query<Select>
Query<Insert>
Query<Update>
Query<Delete>
Query<CreateTable>
```

This allows:

* Clear intent in APIs
* Compile-time separation of query kinds
* Future extensibility (e.g. different traits per query type)

The `Query<T>` itself is just a wrapper around a `String`:

```rust
pub struct Query<T> {
    inner: String,
    _type: PhantomData<T>,
}
```

---

### QueryBuilder

Queries are built using `QueryBuilder<T>` with a fluent API:

```rust
use rusquery::prelude::*;

let query = QueryBuilder::select()
    .table("users")
    .columns(["id", "name"])
    .where_clause("age > 30")
    .and_where("name = 'Alice'")
    .or_where("name = 'Bob'")
    .build();

println!("{}", query);
```

Output:

```sql
SELECT id, name FROM users WHERE age > 30 AND name = 'Alice' OR name = 'Bob';
```

---

## Supported Queries

### SELECT

```rust
QueryBuilder::select()
    .table("users")
    .columns(["id", "name"])
    .where_clause("age > 30")
    .build();
```

```sql
SELECT id, name FROM users WHERE age > 30;
```

---

### INSERT

```rust
QueryBuilder::insert()
    .table("users")
    .columns(["name", "age"])
    .values(["'John'", "30"])
    .build()
```

```sql
INSERT INTO users (name, age) VALUES ('John', 30);
```

---

### UPDATE

```rust
QueryBuilder::update()
    .table("users")
    .columns(["name", "age"])
    .values(["'John Doe'", "31"])
    .where_clause("id = 1")
    .build()
```

```sql
UPDATE users SET name = 'John Doe', age = 31 WHERE id = 1;
```

---

### DELETE

```rust
 QueryBuilder::delete()
    .table("users")
    .where_clause("id = 1")
    .build()
```

```sql
DELETE FROM users WHERE id = 1;
```

---

### CREATE TABLE

```rust
 QueryBuilder::create_table()
    .table("users")
    .column("id PRIMARY KEY")
    .column("name TEXT")
    .column("age INTEGER")
    .build()
```

```sql
CREATE TABLE users (id PRIMARY KEY, name TEXT, age INTEGER);
```

---

## Prelude

Most users should import the prelude:

```rust
use rusquery::prelude::*;
```

This re-exports:

* `Query`
* `QueryBuilder`
* All query kind marker types (`Select`, `Insert`, etc.)

---

## Testing

Generated SQL is validated using [`sqlparser`](https://crates.io/crates/sqlparser):

```rust
let dialect = SQLiteDialect {};
assert!(Parser::parse_sql(&dialect, &query.get()).is_ok());
```

This ensures the output is **syntactically valid SQL**, not just string-matching correct.

---

## Current Limitations

* No SQL escaping or parameter binding
* No joins, ordering, grouping, or limits (yet)
* Dialect-agnostic SQL only
* Some logic is duplicated across query kinds (planned macro refactor)
* CREATE only supports TABLE (planned future implementations)

---

## Future Ideas

* Macro-based `build()` implementations
* Generic `SQLType`-driven builders
* Optional parameter binding support
* Dialect-specific extensions
* Better compile-time guarantees (e.g. required fields)

---

## License

No license has been chosen yet.

Until a license is added, this project is **not licensed** for use,
modification, or redistribution.
