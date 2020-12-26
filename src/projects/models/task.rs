use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::projects::models::phase::Phase;

#[derive(Serialize, Debug)]
pub struct Task {
    #[serde(skip_serializing)]
    pub id: i32,
    pub phase_id: i32,
    #[serde(skip_serializing)]
    pub index: i32,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize, Debug)]
pub struct TaskRequest {
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub async fn set(phase: &Phase, tasks: Vec<TaskRequest>, pool: &PgPool) -> Result<Vec<Task>> {
        let mut tx = pool.begin().await?;
        sqlx::query!("DELETE FROM tasks WHERE phase_id=$1", phase.id)
            .execute(pool)
            .await?;

        let mut res = Vec::new();

        for (index, task) in tasks.iter().enumerate() {
            let r = sqlx::query_as!(
                Task,
                "INSERT INTO tasks (phase_id, index, name, description, completed) VALUES ($1, $2, $3, $4, $5) RETURNING *",
                phase.id,
                index as i32,
                task.name,
                task.description,
                task.completed
                )
                .fetch_one(&mut tx)
                .await?;

            res.push(r);
        }

        tx.commit().await?;

        Ok(res)
    }
}
