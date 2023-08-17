use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    entities::entity_task::EntityTask, error::app_error::AppError,
    utils::adapters::map_sqlx_to_app_error::sqlx_error_to_app_error,
};

#[async_trait]
pub trait TraitRepositoryTask: Sync + Send {
    async fn store(&self, task: String) -> Result<EntityTask, AppError>;
    async fn get_one_by_id(&self, id: i32) -> Result<EntityTask, AppError>;
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError>;
    async fn update(&self, task: EntityTask) -> Result<bool, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct RepositoryTask<'a> {
    pub pool: &'a Pool<Postgres>,
}

#[async_trait]
impl TraitRepositoryTask for RepositoryTask<'_> {
    async fn store(&self, task: String) -> Result<EntityTask, AppError> {
        let new_task = sqlx::query!(
            "INSERT INTO tasks (task) VALUES ($1) RETURNING id, completed, task",
            task
        )
        .fetch_one(self.pool)
        .await
        .map_err(|error| sqlx_error_to_app_error(error))?;

        Ok(EntityTask {
            id: new_task.id,
            task: new_task.task,
            completed: new_task.completed,
        })
    }
    async fn get_one_by_id(&self, id: i32) -> Result<EntityTask, AppError> {
        match sqlx::query_as!(
            EntityTask,
            "SELECT id, task, completed FROM tasks WHERE id = $1",
            id
        )
        .fetch_one(self.pool)
        .await
        {
            Ok(entity) => Ok(entity),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError> {
        match sqlx::query_as!(
            EntityTask,
            "SELECT id, task, completed FROM tasks ORDER by id ASC"
        )
        .fetch_all(self.pool)
        .await
        {
            Ok(entity) => Ok(entity),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn update(&self, task: EntityTask) -> Result<bool, AppError> {
        match sqlx::query!(
            "UPDATE tasks SET task=$1, completed=$2 WHERE id = $3",
            task.task,
            task.completed,
            task.id
        )
        .execute(self.pool)
        .await
        {
            Ok(_) => Ok(true),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        match sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(error) => Err(sqlx_error_to_app_error(error)),
        }
    }
}
