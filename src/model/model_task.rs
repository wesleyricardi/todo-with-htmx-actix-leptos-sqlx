use async_trait::async_trait;

use crate::{
    entities::entity_task::EntityTask, error::app_error::AppError,
    repository::repository_tasks::TraitRepositoryTask,
};

#[async_trait]
pub trait TraitModelTask: Sync + Send {
    async fn add(&self, task: String) -> Result<EntityTask, AppError>;
    async fn get_one_by_id(&self, id: i32) -> Result<EntityTask, AppError>;
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError>;
    async fn update(&self, task: EntityTask) -> Result<bool, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct ModelTask<R> {
    pub repository: R,
}

#[async_trait]
impl<R: TraitRepositoryTask> TraitModelTask for ModelTask<R> {
    async fn add(&self, task: String) -> Result<EntityTask, AppError> {
        Ok(self.repository.store(task).await?)
    }
    async fn get_one_by_id(&self, id: i32) -> Result<EntityTask, AppError> {
        Ok(self.repository.get_one_by_id(id).await?)
    }
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError> {
        Ok(self.repository.get_all().await?)
    }
    async fn update(&self, task: EntityTask) -> Result<bool, AppError> {
        Ok(self.repository.update(task).await?)
    }
    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        Ok(self.repository.delete(id).await?)
    }
}
