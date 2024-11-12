use http::Uri;
use tracing::{error, warn};
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
                refresh(dispatcher, user_ctx, data.transactions.cursor.clone());
            },
            17000,
        );
    }
    html! { <></> }
}

fn refresh(
    dispatcher: UseReducerDispatcher<crate::types::data::APIData>,
    user_ctx: Handle,
    cursor: Option<(String, String)>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(c) = user_ctx.client() {
            match c
                .transaction_list(
                    Some(50),
                    match cursor.clone() {
                        None => None,
                        Some(cursor) => Some(cursor.0),
                    },
                    match cursor {
                        None => None,
                        Some(cursor) => Some(cursor.1),
                    },
                )
                .await
            {
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
                                    let pairs = form_urlencoded::parse(query.as_bytes());
                                    let mut cursor: Option<String> = None;
                                    let mut time: Option<String> = None;
                                    for (key, value) in pairs {
                                        if key == "cursor" {
                                            cursor = Some(value.to_string());
                                        } else if key == "time" {
                                            time = Some(value.to_string());
                                        }
                                    }
                                    if let Some(cursor) = cursor {
                                        if let Some(time) = time {
                                            dispatcher.dispatch(
                                                crate::types::data::APIDataAction::SetTransactionsCursor(
                                                    Some((cursor, time)),
                                                ),
                                            );
                                            return;
                                        }
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
                }
                Err(e) => {
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetTransactionsCursor(
                        None,
                    ));
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetTransactionsLoaded(
                        true,
                    ));
                    error!("Failed to fetch transactions: {:?}", e);
                }
            }
        }
    });
}
