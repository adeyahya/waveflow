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
    let workflow = workflows
        .filter(slug.eq(slug_input))
        .select((id, name, slug, secret, content))
        .first::<(String, String, String, String, String)>(conn);

    match workflow {
        Ok(workflow) => Ok(Workflow {
            id: workflow.0,
            name: workflow.1,
            slug: workflow.2,
            secret: workflow.3,
            content: workflow.4,
        }),
        Err(_) => Err(()),
    }
}

pub async fn insert<'a>(conn: &'a ConnectionPool, data: Workflow) -> Result<Workflow, ()> {
    match diesel::insert_into(workflows)
        .values(data.to_owned())
        .execute(conn)
    {
        Ok(_) => Ok(data.to_owned()),
        Err(_) => Err(()),
    }
}

pub async fn get_all<'a>(conn: &'a ConnectionPool) -> Result<Vec<Workflow>, ()> {
    match workflows.select((id, name, slug, secret, content)).load::<(
        String,
        String,
        String,
        String,
        String,
    )>(conn)
    {
        Ok(results) => {
            let results = results
                .into_iter()
                .map(|x| Workflow {
                    id: x.0,
                    name: x.1,
                    slug: x.2,
                    secret: x.3,
                    content: x.4,
                })
                .rev()
                .collect();

            Ok(results)
        }
        Err(_) => Err(()),
    }
}
