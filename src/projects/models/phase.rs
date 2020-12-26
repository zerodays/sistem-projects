use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::project::Project;
use crate::projects::models::task::Task;

#[derive(Serialize, Debug)]
pub struct Phase {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub description: String,
    pub deadline: chrono::DateTime<Utc>,
    pub date_created: chrono::DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct PhaseRequest {
    pub name: String,
    pub description: String,
    pub deadline: chrono::DateTime<Utc>,
}

impl Phase {
    pub async fn get(project: &Project, id: i32, pool: &PgPool) -> Result<Phase> {
        let r = sqlx::query_as!(
            Phase,
            "SELECT * FROM phases WHERE project_id=$1 AND id=$2",
            project.id,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(r)
    }

    pub async fn create(project: &Project, phase: PhaseRequest, pool: &PgPool) -> Result<Phase> {
        let r = sqlx::query_as!(Phase, "INSERT INTO phases (project_id, name, description, deadline) VALUES ($1, $2, $3, $4) RETURNING *",
            project.id,
            phase.name,
            phase.description,
            phase.deadline).fetch_one(pool).await?;
        Ok(r)
    }

    pub async fn update(id: i32, phase: PhaseRequest, pool: &PgPool) -> Result<Phase> {
        let r = sqlx::query_as!(
            Phase,
            "UPDATE phases SET name=$1, description=$2, deadline=$3 WHERE id=$4 RETURNING *",
            phase.name,
            phase.description,
            phase.deadline,
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(r)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM phases WHERE id=$1", id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn tasks(&self, pool: &PgPool) -> Result<Vec<Task>> {
        let res = sqlx::query_as!(
            Task,
            "SELECT * FROM tasks WHERE phase_id=$1 ORDER BY index",
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(res)
    }
}
