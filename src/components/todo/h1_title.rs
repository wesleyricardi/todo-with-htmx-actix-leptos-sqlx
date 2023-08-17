use leptos::*;

#[component]
pub fn H1Title(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <h1 class="text-4xl font-normal text-center my-8">
          "TO DO LIST"
      </h1>
    }
}
