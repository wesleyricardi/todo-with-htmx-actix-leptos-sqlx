use crate::{
    entities::entity_task::EntityTask,
    error::app_error::AppError,
    model::model_task::TraitModelTask,
    services::sanitation::sanitize_task::{SanitizerTask, TraitSanitizerTask},
};
use async_trait::async_trait;

#[async_trait]
pub trait TraitControllerTask {
    async fn add(&self, task: String) -> Result<EntityTask, AppError>;
    async fn get_one(&self, id: i32) -> Result<EntityTask, AppError>;
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError>;
    async fn update(&self, task: EntityTask) -> Result<bool, AppError>;
    async fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct ControllerTask<M> {
    pub model: M,
}

#[async_trait]
impl<M: TraitModelTask> TraitControllerTask for ControllerTask<M> {
    async fn add(&self, task: String) -> Result<EntityTask, AppError> {
        let sanitation_task = SanitizerTask;
        let task_sanitized = sanitation_task.sanitize_task_input(task)?;

        Ok(self.model.add(task_sanitized).await?)
    }
    async fn get_one(&self, id: i32) -> Result<EntityTask, AppError> {
        Ok(self.model.get_one_by_id(id).await?)
    }
    async fn get_all(&self) -> Result<Vec<EntityTask>, AppError> {
        Ok(self.model.get_all().await?)
    }
    async fn update(&self, task: EntityTask) -> Result<bool, AppError> {
        let sanitation_task = SanitizerTask;

        let task_sanitized = sanitation_task.sanitize_task_input(task.task)?;

        Ok(self
            .model
            .update(EntityTask {
                id: task.id,
                task: task_sanitized,
                completed: task.completed,
            })
            .await?)
    }
    async fn delete(&self, id: i32) -> Result<bool, AppError> {
        Ok(self.model.delete(id).await?)
    }
}
