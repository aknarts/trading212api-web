use http::Uri;
use tracing::{error, warn};
use trading212::error::Error;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(TransactionsRefresher)]
pub fn transactions_refresher() -> Html {
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
                let cursor = if (*data).transactions.loaded {
                    None
                } else {
                    // FIXME: Pagination does not work yet, falling back to None
                    //(*data).transactions.cursor
                    None
                };
                refresh(dispatcher, user_ctx, cursor);
            },
            11000,
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
            match c.transaction_list(Some(50), cursor).await {
                Ok(transactions) => {
                    for transaction in &transactions.items {
                        dispatcher.dispatch(crate::types::data::APIDataAction::AddTransaction(
                            transaction.clone(),
                        ));
                    }

                    if let Some(next) = transactions.next_page_path {
                        match next.parse::<Uri>() {
                            Ok(uri) => {
                                if let Some(query) = uri.query() {
                                    let mut pairs = form_urlencoded::parse(query.as_bytes());
                                    while let Some((_key, _value)) = pairs.next() {
                                        // FIXME: It is unclear what the cursor is at this point, when Trading212 fixes the docs and the endpoint this needs to get resolved
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error parsing next page path: {:?}", e);
                            }
                        };
                    }
                    warn!("Most likely reached the end of transactions");
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetTransactionsCursor(
                        None,
                    ));
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetTransactionsLoaded(
                        true,
                    ));
                    break;
                }
                Err(e) => {
                    if let Error::Limit = e {
                        warn!("Failed to fetch transactions, retrying");
                        yew::platform::time::sleep(std::time::Duration::from_secs(5)).await;
                        if retries < 2 {
                            retries += 1;
                            continue;
                        }
                        warn!("Failed to fetch transactions after 2 retries");
                        break;
                    }
                    error!("Failed to fetch transactions: {:?}", e);
                    break;
                }
            }
        }
    });
}
