use tracing::{error, warn};
use trading212::error::Error;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(AccountRefresher)]
pub fn account_refresher() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    let data =
        use_context::<UseReducerHandle<crate::types::data::APIData>>().expect("no ctx found");
    {
        let dispatcher = data.dispatcher();
        let ctx = user_ctx.clone();

        yew_hooks::use_mount(move || {
            refresh(dispatcher.clone(), ctx.clone());
        });
        let dispatcher = data.dispatcher();
        let user_ctx = user_ctx.clone();
        yew_hooks::use_interval(
            move || {
                let dispatcher = dispatcher.clone();
                let user_ctx = user_ctx.clone();
                refresh(dispatcher, user_ctx);
            },
            31000,
        );
    }
    html! { <></> }
}

fn refresh(dispatcher: UseReducerDispatcher<crate::types::data::APIData>, user_ctx: Handle) {
    wasm_bindgen_futures::spawn_local(async move {
        let mut retries = 0;
        while let Some(c) = user_ctx.client() {
            match c.get_account_metadata().await {
                Ok(account) => {
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetAccount(Some(
                        account.clone(),
                    )));
                }
                Err(e) => {
                    if let Error::Limit = e {
                        warn!("Failed to fetch account metadata, retrying");
                        yew::platform::time::sleep(std::time::Duration::from_secs(5)).await;
                        if retries < 5 {
                            retries += 1;
                            continue;
                        }
                        warn!("Failed to fetch account metadata after 5 retries");
                        break;
                    }
                    error!("Failed to fetch account metadata: {:?}", e);
                    break;
                }
            }
        }
    });
}
