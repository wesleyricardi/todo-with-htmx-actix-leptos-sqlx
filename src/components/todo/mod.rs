pub mod form_create_new_task;
pub mod h1_title;
pub mod h2_subtitle;
pub mod li_task;
pub mod ul_tasks_list;

use {
    form_create_new_task::FormCreateNewTask, h1_title::H1Title, h2_subtitle::H2Subtitle,
    ul_tasks_list::UlTasksList,
};

use crate::entities::entity_task::EntityTask;
use leptos::*;

#[component]
pub fn TODO(cx: Scope, tasks: Vec<EntityTask>) -> impl IntoView {
    view! {
      cx,
      <main class="px-4 max-w-2xl mx-auto">
        <H1Title/>
        <H2Subtitle/>
        <UlTasksList tasks=tasks/>
        <FormCreateNewTask/>
      </main>
    }
}
