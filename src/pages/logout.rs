use yew::{function_component, html, Html};
use yew_hooks::use_timeout;

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    {
        use_timeout(
            move || {
                user_ctx.navigate_to(&crate::app::Route::Home);
            },
            2000,
        )
    };

    html!(
        <div class="container pb-0">
            <h1 class="title is-1">{ "Good bye..." }</h1>
        </div>
    )
}
