use crate::components::todo::{li_task::LiTask, TODO};
use crate::entities::entity_task::EntityTask;
use async_trait::async_trait;
use leptos::*;

#[async_trait]
pub trait TraitViewTask: Sync + Send {
    async fn render_todo(&self, tasks: Vec<EntityTask>) -> String;
    async fn render_todo_li(&self, task: EntityTask) -> String;
}

pub struct ViewTask;

#[async_trait]
impl TraitViewTask for ViewTask {
    async fn render_todo(&self, tasks: Vec<EntityTask>) -> String {
        leptos::ssr::render_to_string(move |cx| {
            view! { cx,
                <head>
                <meta charset="utf-8"/>
                <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
                <title>"TODO using htmx + actix + leptos"</title>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" type="text/css" media="screen" href="static/styles/output.css"/>
                <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV"
                crossorigin="anonymous"></script>
                <script src="/static/js/error_handler.js"></script>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Playfair+Display&display=swap" rel="stylesheet"/>
                </head>
                <body>
                <TODO tasks={tasks} />
                </body>
            }
        })
    }
    async fn render_todo_li(&self, task: EntityTask) -> String {
        leptos::ssr::render_to_string(move |cx| {
            view! { cx,
                <LiTask task=task/>
            }
        })
    }
}
