use crate::{
    controller::controller_task::{ControllerTask, TraitControllerTask},
    entities::entity_task::EntityTask,
    model::model_task::ModelTask,
    repository::repository_tasks::RepositoryTask,
    utils::adapters::map_app_error_to_actix_response::app_error_to_actix_response,
    view::view_task::{TraitViewTask, ViewTask},
    AppState,
};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    task: String,
}

#[get("/")]
pub async fn index(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let repository = RepositoryTask {
        pool: &data.postgres_pool,
    };
    let model = ModelTask { repository };
    let controller = ControllerTask { model };

    match controller.get_all().await {
        Ok(tasks) => {
            let view = ViewTask;
            let html = view.render_todo(tasks).await;

            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html);
        }
        Err(error) => app_error_to_actix_response(error),
    }
}

#[post("/task")]
pub async fn add_task(
    _req: HttpRequest,
    data: web::Data<AppState>,
    form: web::Form<FormData>,
) -> HttpResponse {
    let repository = RepositoryTask {
        pool: &data.postgres_pool,
    };
    let model = ModelTask { repository };
    let controller = ControllerTask { model };

    match controller.add(form.task.clone()).await {
        Ok(task) => {
            let view = ViewTask;
            let html = view.render_todo_li(task).await;

            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html);
        }
        Err(error) => app_error_to_actix_response(error),
    }
}

#[derive(Deserialize, Debug)]
struct FormUpdateTask {
    completed: Option<String>,
    task: String,
}

#[put("/task/{task_id}")]
async fn update_task(
    _req: HttpRequest,
    data: web::Data<AppState>,
    form: web::Form<FormUpdateTask>,
    path: web::Path<i32>,
) -> HttpResponse {
    let task_id = path.into_inner();

    let completed = if form.completed.is_some() {
        true
    } else {
        false
    };

    let repository = RepositoryTask {
        pool: &data.postgres_pool,
    };
    let model = ModelTask { repository };
    let controller = ControllerTask { model };
    match controller
        .update(EntityTask {
            id: task_id,
            task: form.task.clone(),
            completed,
        })
        .await
    {
        Ok(_) => {
            let view = ViewTask;
            let html = view
                .render_todo_li(EntityTask {
                    id: task_id,
                    task: form.task.clone(),
                    completed,
                })
                .await;

            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html);
        }
        Err(error) => app_error_to_actix_response(error),
    }
}

#[delete("/task/{task_id}")]
async fn delete_task(
    _req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let task_id = path.into_inner();

    let repository = RepositoryTask {
        pool: &data.postgres_pool,
    };
    let model = ModelTask { repository };
    let controller = ControllerTask { model };

    match controller.delete(task_id).await {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(""),
        Err(error) => app_error_to_actix_response(error),
    }
}
