use leptos::*;

#[component]
pub fn H2Subtitle(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <h2 class="text-2xl font-normal text-left my-8">
        "TASK LIST"
      </h2>
    }
}
