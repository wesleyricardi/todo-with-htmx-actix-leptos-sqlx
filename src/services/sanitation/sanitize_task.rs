use sanitizer::StringSanitizer;

use crate::error::app_error::AppError;

pub trait TraitSanitizerTask {
    fn sanitize_task_input(&self, task: String) -> Result<String, AppError>;
}

pub struct SanitizerTask;

impl TraitSanitizerTask for SanitizerTask {
    fn sanitize_task_input(&self, task: String) -> Result<String, AppError> {
        if task.is_empty() {
            return Err(AppError::invalid_argument("Task is empty"));
        }

        let mut instance = StringSanitizer::from(task);
        instance.trim();

        let task_sanitized = instance.get();
        if task_sanitized.is_empty() {
            return Err(AppError::invalid_argument("Task is empty after sanitizer"));
        }

        Ok(task_sanitized)
    }
}
