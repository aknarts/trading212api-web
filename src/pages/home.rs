use yew::{function_component, html, Html};

use crate::services::requests::get_token;

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    if get_token().is_none() {
        user_ctx.navigate_to(&crate::app::Route::Login);
    }
    html!(
        <div class="container pb-0">
            {
                if get_token().is_some() {
                    html!(
                        <crate::pages::dashboard::Dashboard />
                    )
                } else {
                    html!(
                        <crate::pages::token::no_token::NoToken />
                    )
                }
            }
        </div>
    )
}
