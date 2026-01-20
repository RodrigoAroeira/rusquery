use super::QueryBuilder;
use super::macros::impl_query_kind;
use crate::query_builder::macros::define_query_method;
use crate::query_builder::macros::define_sql_type;
use paste::paste;

pub trait SQLType {
    const KEYWORD: &'static str;
    fn build_base(db: &QueryBuilder<Self>) -> String
    where
        Self: Sized,
    {
        _ = db;
        String::from(Self::KEYWORD)
    }
}

impl_query_kind!(Select, Update, Delete, Insert, CreateTable);
