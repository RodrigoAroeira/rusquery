macro_rules! define_sql_type {
    ($typ:ident $(,)?) => {
        #[derive(Default)]
        pub struct $typ;
        impl SQLType for $typ {
            paste! {
                const KEYWORD: &'static str = stringify!([<$typ:upper>]);
            }
        }
    };
    ($($typ:ident),+ $(,)?) => {
        $(define_sql_type!($typ))+
    };
}

macro_rules! define_query_method {
    ($typ:ident $(,)?) => {
        paste! {
            impl QueryBuilder<$typ> {
                pub fn [<$typ:snake>]() -> QueryBuilder<$typ> {
                    Default::default()
                }
            }
        }
    };
    ($($typ:ident),+ $(,)?) => {
        $(define_query_method!($typ))+
    };
}

macro_rules! impl_query_kind {
    ($typ:ident $(,)?) => {
        define_sql_type!($typ);
        define_query_method!($typ);
    };
    ($($typ:ident),+ $(,)?) => {
        $(impl_query_kind!($typ);)+
    };
}

pub(crate) use define_query_method;
pub(crate) use define_sql_type;
pub(crate) use impl_query_kind;
