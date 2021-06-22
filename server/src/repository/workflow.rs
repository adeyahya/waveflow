use crate::models::*;
use crate::schema::workflows::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

type ConnectionPool = PooledConnection<ConnectionManager<SqliteConnection>>;

pub async fn get_by_slug<'a, 'b>(
    conn: &'a ConnectionPool,
    slug_input: &'b String,
) -> Result<Workflow, ()> {
    workflows
        .filter(slug.eq(slug_input))
        .first::<Workflow>(conn)
        .map(|data| Ok(data))
        .unwrap_or_else(|_| Err(()))
}

pub async fn insert<'a>(conn: &'a ConnectionPool, input: Workflow) -> Result<Workflow, ()> {
    diesel::insert_into(workflows)
        .values(input.to_owned())
        .execute(conn)
        .map(|_| async move { get_by_slug(conn, &input.slug).await })
        .unwrap()
        .await
}

pub async fn get_all<'a>(conn: &'a ConnectionPool) -> Result<Vec<Workflow>, ()> {
    workflows
        .load(conn)
        .map(|data| data.into_iter())
        .map(|item| item.rev().collect())
        .map(|data| Ok(data))
        .unwrap_or_else(|_| Err(()))
}
