use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html!(
        <>
            <nav class="navbar navbar-expand sticky-top shadow bg-light" aria-label="main navigation">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand fs-2">
                        { "TR^DING 212 API" }
                    </Link<Route>>
                </div>
            </nav>
        </>
    )
}