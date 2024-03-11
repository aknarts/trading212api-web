use yew::{function_component, Html, html};
use crate::services::requests::get_token;

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Welcome..." }</h1>
                    {
                        if get_token().is_some() {
                            has_token_view()
                        } else {
                            html!(
                                <crate::pages::token::no_token::NoToken />
                            )
                        }
                    }
                </div>
            </div>
        </div>
    )
}

fn has_token_view() -> Html {
    html!( <h2> {"Has Token"}</h2> )
}