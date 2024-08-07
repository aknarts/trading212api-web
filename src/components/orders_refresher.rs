use http::Uri;
use tracing::{error, warn};
use trading212::error::Error;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(OrdersRefresher)]
pub fn orderss_refresher() -> Html {
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
                let cursor = if (*data).orders.loaded {
                    None
                } else {
                    (*data).orders.cursor
                };
                refresh(dispatcher, user_ctx, cursor);
            },
            18000,
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
        let mut retries = 0;
        while let Some(c) = user_ctx.client() {
            match c.get_historical_orders(Some(50), cursor, None).await {
                Ok(orders) => {
                    for order in &orders.items {
                        dispatcher
                            .dispatch(crate::types::data::APIDataAction::AddOrder(order.clone()));
                    }

                    if let Some(next) = orders.next_page_path {
                        match next.parse::<Uri>() {
                            Ok(uri) => {
                                if let Some(query) = uri.query() {
                                    let mut pairs = form_urlencoded::parse(query.as_bytes());
                                    while let Some((key, value)) = pairs.next() {
                                        if key == "cursor" {
                                            match value.to_string().parse::<i64>() {
                                                Ok(cursor) => {
                                                    dispatcher.dispatch(crate::types::data::APIDataAction::SetOrdersCursor(
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
                    warn!("Most likely reached the end of orders");
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetOrdersCursor(None));
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetOrdersLoaded(true));
                    break;
                }
                Err(e) => {
                    if let Error::Limit = e {
                        warn!("Failed to fetch orders, retrying");
                        yew::platform::time::sleep(std::time::Duration::from_secs(5)).await;
                        if retries < 2 {
                            retries += 1;
                            continue;
                        }
                        warn!("Failed to fetch pies orders 2 retries");
                        break;
                    }
                    error!("Failed to fetch orders: {:?}", e);
                    break;
                }
            }
        }
    });
}
