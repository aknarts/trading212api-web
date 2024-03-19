use tracing::debug;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(ExchangeRefresher)]
pub fn exchange_refresher() -> Html {
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
        if let Some(c) = user_ctx.client() {
            debug!("fetching exchanges");
            if let Ok(exchanges) = c.get_exchanges().await {
                dispatcher.dispatch(crate::types::data::APIDataAction::SetExchanges(
                    exchanges.clone(),
                ));
                debug!("fetched exchanges: {:?}", exchanges);
            }
        }
    });
}
