use tracing::debug;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::user_context_provider::UserContextProvider;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <crate::pages::header::Header />
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </UserContextProvider>
    }
}

#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    debug!("Routing to {:?}", routes);
    match routes {
        Route::Home => html! ( <crate::pages::home::Home /> ),
        Route::NotFound => html! ( <crate::pages::page_not_found::PageNotFound /> ),
    }
}