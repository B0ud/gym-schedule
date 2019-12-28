use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub trait Paginate: AsQuery + Sized {
    fn paginate(self, limit: Option<i64>, offset: Option<i64>) -> Paginated<Self::Query> {
        Paginated {
            query: self.as_query(),
            limit,
            offset,
        }
    }
}
impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T: AsQuery> Paginate for T {}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> Paginated<T> {
    pub fn load_and_count_total<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let results = self.load::<(U, i64)>(conn)?;
        let total: i64 = results.get(0).map(|x| x.1).unwrap_or(0);
        let records: Vec<U> = results.into_iter().map(|x| x.0).collect();
        Ok((records, total))
    }
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;

        match &self.limit {
            None => out.push_sql(") t "),
            Some(limit) => {
                out.push_sql(") t LIMIT ");
                out.push_bind_param::<BigInt, _>(limit)?;
            }
        }
        match &self.offset {
            None => (),
            Some(offset) => {
                out.push_sql(" OFFSET ");
                out.push_bind_param::<BigInt, _>(offset)?;
            }
        }
        Ok(())
    }
}
