use leptos::*;

#[component]
pub fn FormCreateNewTask(cx: Scope) -> impl IntoView {
    view! {
      cx,

        <form onsubmit="return false;" class="flex border-b-2 py-3 pl-8 border-solid border-black w-full"
        hx-trigger="focusout from:#new_task_input, keyup[keyCode==13] from:#new_task_input"
        hx-post="/task"
        hx-target="#task_list"
        hx-swap="beforeend"
        hx-on="htmx:afterRequest: this.reset()">
          <input id="new_task_input" class="grow pl-4" type="text" name="task" required/>
          <button class="h-8 w-8 rounded-full opacity-40 hover:opacity-100 bg-emerald-600 text-neutral-200">"✓"</button>
        </form>

    }
}
