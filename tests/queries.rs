use rusquery::prelude::*;
use sqlparser::{dialect::SQLiteDialect, parser::Parser};

fn gen_insert() -> Query<Insert> {
    QueryBuilder::insert()
        .table("users")
        .columns(["name", "age"])
        .values(["'John'", "30"])
        .build()
}

fn gen_select() -> Query<Select> {
    QueryBuilder::select()
        .table("users")
        .columns(["id", "name"])
        .where_clause("age > 30")
        .and_where("name = 'Alice'")
        .or_where("name = 'Bob'")
        .build()
}

fn gen_create() -> Query<CreateTable> {
    QueryBuilder::create_table()
        .table("users")
        .column("id PRIMARY KEY")
        .column("name TEXT")
        .column("age INTEGER")
        .build()
}

fn parse_query<T>(q: Query<T>) -> bool {
    let dialect = SQLiteDialect {};
    let ast = Parser::parse_sql(&dialect, &q.get());
    ast.is_ok()
}

#[test]
fn test_select() {
    assert_eq!(
        gen_select().get(),
        "SELECT id, name FROM users WHERE age > 30 AND name = 'Alice' OR name = 'Bob';"
    )
}

#[test]
fn parse_select() {
    assert!(parse_query(gen_select()))
}

#[test]
fn test_insert() {
    assert_eq!(
        gen_insert().get(),
        "INSERT INTO users (name, age) VALUES ('John', 30);"
    )
}

#[test]
fn parse_insert() {
    assert!(parse_query(gen_insert()))
}

#[test]
fn parse_create() {
    assert!(parse_query(gen_create()))
}
