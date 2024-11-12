use http::Uri;
use tracing::{error, warn};
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(DividendsRefresher)]
pub fn dividends_refresher() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    let data =
        use_context::<UseReducerHandle<crate::types::data::APIData>>().expect("no ctx found");
    {
        let dispatcher = data.dispatcher();
        let ctx = user_ctx.clone();

        yew_hooks::use_mount(move || {
            refresh(dispatcher.clone(), ctx.clone(), None);
        });
        let dispatcher = data.dispatcher();
        let data = data.clone();
        let user_ctx = user_ctx.clone();
        yew_hooks::use_interval(
            move || {
                let dispatcher = dispatcher.clone();
                let user_ctx = user_ctx.clone();
                refresh(dispatcher, user_ctx, data.dividends.cursor);
            },
            15000,
        );
    }
    html! { <></> }
}

fn refresh(
    dispatcher: UseReducerDispatcher<crate::types::data::APIData>,
    user_ctx: Handle,
    cursor: Option<i64>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(c) = user_ctx.client() {
            match c.get_paid_dividends(Some(50), cursor, None).await {
                Ok(dividends) => {
                    for dividend in &dividends.items {
                        dispatcher.dispatch(crate::types::data::APIDataAction::AddDividend(
                            dividend.clone(),
                        ));
                    }

                    if let Some(next) = dividends.next_page_path {
                        match next.parse::<Uri>() {
                            Ok(uri) => {
                                if let Some(query) = uri.query() {
                                    let pairs = form_urlencoded::parse(query.as_bytes());
                                    for (key, value) in pairs {
                                        if key == "cursor" {
                                            match value.to_string().parse::<i64>() {
                                                Ok(cursor) => {
                                                    dispatcher.dispatch(crate::types::data::APIDataAction::SetDividendsCursor(
                                                        Some(cursor)
                                                    ));
                                                    return;
                                                }
                                                Err(e) => {
                                                    error!("Error parsing cursor: {:?}", e);
                                                }
                                            };
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error parsing next page path: {:?}", e);
                            }
                        };
                    }
                    warn!("Most likely reached the end of dividends");
                    dispatcher
                        .dispatch(crate::types::data::APIDataAction::SetDividendsCursor(None));
                    dispatcher
                        .dispatch(crate::types::data::APIDataAction::SetDividendsLoaded(true));
                }
                Err(e) => {
                    error!("Failed to fetch dividends: {:?}", e);
                }
            }
        }
    });
}
