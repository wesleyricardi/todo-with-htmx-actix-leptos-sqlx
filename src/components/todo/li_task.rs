use leptos::*;

use crate::entities::entity_task::EntityTask;

#[component]
pub fn LiTask(cx: Scope, task: EntityTask) -> impl IntoView {
    let EntityTask {
        id,
        task,
        completed,
    } = task;

    view! {
      cx,
      <li class="flex border-b-2 py-3 border-solid border-black" id=format!("task_id_{}", id)>
      <form onsubmit="return false;" class="flex grow mb-0" hx-put=format!("/task/{}", id)
        hx-trigger=format!("focusout, click from:#checkbox_task_id_{}", {id})
        hx-target=format!("#task_id_{}", id)
        hx-swap="outerHTML">
        {
        if completed {view! {
            cx, <input
                class="h-8 w-8 accent-emerald-600 border-emerald-600 text-neutral-200 rounded-full"
                id=format!("checkbox_task_id_{}", id)
                type="checkbox"
                name="completed"
                checked/>
        }} else {view! {
            cx, <input
                class="h-8 w-8 accent-emerald-600 border-emerald-600 text-neutral-200 rounded-full"
                id=format!("checkbox_task_id_{}", id)
                type="checkbox"
                name="completed"/>
        }}
        }
        <input
          class="grow ml-2 pl-4"
          type="text"
          name="task"
          id=format!("input_task_id_{}", id)
          value=task
          required/>

      </form>
        <button
        class="h-8 w-8 rounded-full opacity-30 hover:opacity-100 bg-red-600 text-neutral-200"
        hx-delete=format!("/task/{}", id)
        hx-trigger="click"
        hx-target=format!("#task_id_{}", id)
        hx-swap="outerHTML"
        >
          "✕"
        </button>
      </li>
    }
}
