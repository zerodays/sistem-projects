use super::phase::Phase;
use crate::projects::models::user::User;
use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub date_created: chrono::DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectRequest {
    pub name: String,
    pub description: String,
}

impl Project {
    pub async fn all(pool: &PgPool) -> Result<Vec<Project>> {
        let records = sqlx::query!("SELECT * FROM projects ORDER BY date_created DESC")
            .fetch_all(pool)
            .await?;

        let mut projects = Vec::new();
        for rec in records {
            projects.push(Project {
                id: rec.id,
                name: rec.name,
                description: rec.description,
                date_created: rec.date_created,
            });
        }

        Ok(projects)
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Project> {
        let record = sqlx::query!("SELECT * FROM projects WHERE id=$1", id)
            .fetch_one(pool)
            .await?;
        Ok(Project {
            id: record.id,
            name: record.name,
            description: record.description,
            date_created: record.date_created,
        })
    }

    pub async fn create(project: ProjectRequest, pool: &PgPool) -> Result<Project> {
        let record = sqlx::query!(
            "INSERT INTO projects (name, description) VALUES ($1, $2) RETURNING *",
            project.name,
            project.description
        )
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: record.id,
            name: record.name,
            description: record.description,
            date_created: record.date_created,
        })
    }

    pub async fn update(id: i32, project: ProjectRequest, pool: &PgPool) -> Result<Project> {
        let record = sqlx::query!(
            "UPDATE projects SET name=$1, description=$2 WHERE id=$3 RETURNING *",
            project.name,
            project.description,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Project {
            id: record.id,
            name: record.name,
            description: record.description,
            date_created: record.date_created,
        })
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM projects WHERE id=$1", id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn phases(&self, pool: &PgPool) -> Result<Vec<Phase>> {
        let records = sqlx::query!(
            "SELECT * FROM phases WHERE project_id=$1 ORDER BY deadline DESC",
            self.id
        )
        .fetch_all(pool)
        .await?;

        let mut phases = Vec::new();
        for rec in records {
            phases.push(Phase {
                id: rec.id,
                project_id: rec.project_id,
                name: rec.name,
                description: rec.description,
                deadline: rec.deadline,
                date_created: rec.date_created,
            });
        }

        Ok(phases)
    }

    pub async fn users(&self, pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users WHERE project_id=$1", self.id)
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}
