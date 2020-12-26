use crate::projects::models::project::Project;
use anyhow::Result;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, Debug)]
pub struct User {
    pub project_id: i32,
    pub user_id: String,
}

impl User {
    pub async fn set(project: &Project, users: Vec<String>, pool: &PgPool) -> Result<Vec<User>> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM users WHERE project_id=$1")
            .bind(project.id)
            .execute(&mut tx)
            .await?;

        let mut res = Vec::new();

        for user_id in users {
            let u = sqlx::query_as!(
                User,
                "INSERT INTO users (project_id, user_id) VALUES ($1, $2) RETURNING *",
                project.id,
                user_id
            )
            .fetch_one(&mut tx)
            .await?;

            res.push(u);
        }

        tx.commit().await?;

        Ok(res)
    }
}
