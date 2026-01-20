use crate::query_builder::kinds::*;
use std::marker::PhantomData;

use crate::query::Query;

#[expect(dead_code)]
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

#[derive(Default)]
pub struct QueryBuilder<T> {
    table: String,
    columns: Vec<String>,
    conditions: Option<String>,
    values: Vec<String>,
    _type: PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    pub fn table(mut self, table: impl ToString) -> Self {
        self.table = table.to_string();
        self
    }

    pub fn column<S>(mut self, column: S) -> Self
    where
        S: ToString,
    {
        self.columns.push(column.to_string());
        self
    }

    pub fn columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.columns = columns.into_iter().map(|col| col.to_string()).collect();
        self
    }

    pub fn where_clause(mut self, condition: impl ToString) -> Self {
        self.conditions = Some(condition.to_string());
        self
    }

    pub fn values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.values = values.into_iter().map(|val| val.to_string()).collect();
        self
    }

    pub fn and_where(mut self, condition: impl ToString) -> Self {
        if let Some(ref mut existing_conditions) = self.conditions {
            existing_conditions.push_str(" AND ");
            existing_conditions.push_str(&condition.to_string());
        } else {
            return self.where_clause(condition);
        }
        self
    }

    pub fn or_where(mut self, condition: impl ToString) -> Self {
        if let Some(ref mut existing_conditions) = self.conditions {
            existing_conditions.push_str(" OR ");
            existing_conditions.push_str(&condition.to_string());
        } else {
            return self.where_clause(condition);
        }
        self
    }
}

// TODO: Make a macro to define make all of these
// Maybe just for the query part, since if let Some and push ; and Query new are the same
impl QueryBuilder<Select> {
    #[must_use]
    pub fn build(self) -> Query<Select> {
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut query = format!("SELECT {} FROM {}", cols, self.table);
        if let Some(ref cond) = self.conditions {
            query.push_str(" WHERE ");
            query.push_str(cond);
        }
        query.push(';');
        Query::new(query)
    }
}

impl QueryBuilder<Delete> {
    #[must_use]
    pub fn build(self) -> Query<Delete> {
        let mut query = format!("DELETE FROM {}", self.table);
        if let Some(ref cond) = self.conditions {
            query.push_str(" WHERE ");
            query.push_str(cond);
        }
        query.push(';');
        Query::new(query)
    }
}

impl QueryBuilder<Insert> {
    #[must_use]
    pub fn build(self) -> Query<Insert> {
        let mut query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            self.columns.join(", "),
            self.values.join(", "),
        );
        query.push(';');
        Query::new(query)
    }
}

impl QueryBuilder<Update> {
    #[must_use]
    pub fn build(self) -> Query<Update> {
        assert!(
            !self.columns.is_empty(),
            "UPDATE requires at least one column"
        );
        assert!(
            self.columns.len() == self.values.len(),
            "columns and values must have the same length"
        );

        let set_clause = self
            .columns
            .into_iter()
            .zip(self.values)
            .map(|(col, val)| format!("{col} = {val}"))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = format!("UPDATE {} SET {}", self.table, set_clause);

        if let Some(ref cond) = self.conditions {
            query.push_str(" WHERE ");
            query.push_str(cond);
        }

        query.push(';');
        Query::new(query)
    }
}

impl QueryBuilder<CreateTable> {
    pub fn build(self) -> Query<CreateTable> {
        assert!(
            !self.columns.is_empty(),
            "CREATE TABLE requires at least one column"
        );

        let mut query = format!("CREATE TABLE {}", self.table);
        query.push_str(" (");
        query.push_str(&self.columns.join(", "));
        query.push_str(");");

        Query::new(query)
    }
}
