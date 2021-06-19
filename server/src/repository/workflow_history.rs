use crate::models::*;
use crate::schema::workflows_history::dsl::*;
use crate::*;
use ::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use uuid::Uuid;

pub async fn get_by_workflow_id<'a, 'b>(
    conn: &'a PooledConnection<ConnectionManager<SqliteConnection>>,
    relation_id: &'b String,
) -> Vec<WorkflowHistory> {
    let mut results = vec![];
    let histories = workflows_history
        .filter(workflow_id.eq(relation_id))
        .select((id, workflow_id, is_success, content))
        .load::<(String, String, bool, Option<String>)>(conn);

    if histories.is_ok() {
        for entry in histories.unwrap() {
            results.push(WorkflowHistory {
                id: entry.0,
                workflow_id: entry.1,
                is_success: entry.2,
                content: entry.3,
            })
        }
    };

    results
}

pub async fn insert<'a>(
    conn: &'a PooledConnection<ConnectionManager<SqliteConnection>>,
    relation_id: String,
    log: Option<String>,
    status: bool,
) -> Result<WorkflowHistory, ()> {
    let history = WorkflowHistory {
        id: Uuid::new_v4().to_string(),
        workflow_id: relation_id,
        content: log,
        is_success: status,
    };
    let query = diesel::insert_into(workflows_history)
        .values(&history)
        .execute(conn);

    match query {
        Ok(_) => Ok(history),
        Err(_) => Err(()),
    }
}
