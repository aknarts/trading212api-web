use tracing::debug;
use web_sys::Window;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::user_context_provider::UserContextProvider;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/logout")]
    Logout,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[allow(dead_code)]
// DOCS: https://developer.mozilla.org/en-US/docs/Web/API/Window
pub fn window() -> Option<Window> {
    web_sys::window()
}

pub fn document() -> Option<web_sys::Document> {
    window().and_then(|w| w.document())
}

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| crate::types::theme::Theme { dark: true });

    document().and_then(|doc| {
        doc.document_element().and_then(|el| {
            if ctx.get_dark() {
                el.set_attribute("data-bs-theme", "dark").unwrap();
            } else {
                el.set_attribute("data-bs-theme", "danger").unwrap();
            };
            Some(())
        });
        Some(())
    });

    html! {
        <ContextProvider<UseStateHandle<crate::types::theme::Theme>> context={ctx}>
            <UserContextProvider>
                <BrowserRouter>
                    <crate::pages::header::Header />
                    <main>
                        <Switch<Route> render={switch} />
                    </main>
                </BrowserRouter>
            </UserContextProvider>
        </ContextProvider<UseStateHandle<crate::types::theme::Theme>>>
    }
}

#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    debug!("Routing to {:?}", routes);
    match routes {
        Route::Home => html!( <crate::pages::home::Home /> ),
        Route::NotFound => html!( <crate::pages::page_not_found::PageNotFound /> ),
        Route::Logout => html!( <crate::pages::logout::Logout /> ),
        Route::Login => html!( <crate::pages::token::no_token::NoToken /> ),
    }
}
