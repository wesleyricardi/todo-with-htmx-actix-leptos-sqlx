use leptos::*;

use crate::{components::todo::li_task::LiTask, entities::entity_task::EntityTask};

#[component]
pub fn UlTasksList(cx: Scope, tasks: Vec<EntityTask>) -> impl IntoView {
    view! {
      cx,
      <ul id="task_list">
          {tasks.into_iter().map(|task| view! {
            cx,
            <LiTask task=task />
          })
          .collect_view(cx)}
      </ul>
    }
}
