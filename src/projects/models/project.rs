use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub date_created: chrono::DateTime<Utc>,
}

impl Project {
    pub async fn all(pool: &PgPool) -> Result<Vec<Project>> {
        let records = sqlx::query!("SELECT * FROM projects ORDER BY date_created DESC").fetch_all(pool).await?;

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
}
