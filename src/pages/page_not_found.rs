use yew::prelude::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! (
        <div class="container">
            <h1 class="title">
                { "Page not found" }
            </h1>
            <h2 class="subtitle">
                { "Page does not seem to exist" }
            </h2>
        </div>
    )
}
