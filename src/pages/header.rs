use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    let theme = use_context::<UseStateHandle<crate::types::theme::Theme>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let activated = *active;

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let theme_class = if (*theme).get_dark() {
        "fa-solid"
    } else {
        "fa-regular"
    };

    let onclick_theme = {
        Callback::from(move |_| {
            theme.set(crate::types::theme::Theme {
                dark: !theme.get_dark(),
            });
            theme.toggle_dark();
        })
    };

    let onclick_logout = {
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };

    html!(
        <>
            <nav class={classes!("navbar", "navbar-expand-lg", " sticky-top", "shadow")} aria-label="main navigation">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand fs-2">
                        { "TR^DING 212 API" }
                    </Link<Route>>
                    <div class="d-flex">
                    <button class={classes!("navbar-toggler", active_class.1)} type="button" onclick={onclick.clone()} aria-controls="navbarSupportedContent" aria-expanded={(!activated).to_string()} aria-label="Toggle navigation">
                      <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class={classes!("collapse","navbar-collapse", active_class.0)} id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        {
                            if crate::services::requests::get_token().is_some() {
                                html!(
                                    <>
                                        <li class="nav-item">
                                            <a class="nav-link" onclick={onclick_theme}>
                                                <i class={classes!(theme_class, "fa-sun")}></i>
                                            </a>
                                        </li>
                                        <li class="nav-item">
                                            <a class="nav-link" onclick={onclick_logout}>
                                                <i class="fa-solid fa-right-from-bracket"></i>
                                            </a>
                                        </li>
                                    </>
                                )
                            } else {
                               html!(
                                    <li class="nav-item">
                                        <a class="nav-link" onclick={onclick_theme}>
                                            <i class={classes!(theme_class, "fa-sun")}></i>
                                        </a>
                                    </li>
                                )
                            }
                        }
                        </ul>
                    </div>
                    </div>
                </div>
            </nav>
        </>
    )
}
