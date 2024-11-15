use tracing::error;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(InstrumentRefresher)]
pub fn instrument_refresher() -> Html {
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
            130000,
        );
    }
    html! { <></> }
}

fn refresh(dispatcher: UseReducerDispatcher<crate::types::data::APIData>, user_ctx: Handle) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(c) = user_ctx.client() {
            match c.get_instruments().await {
                Ok(instruments) => {
                    dispatcher.dispatch(crate::types::data::APIDataAction::SetInstruments(
                        instruments.clone(),
                    ));
                }
                Err(e) => {
                    error!("Failed to fetch instruments: {:?}", e);
                }
            }
        }
    });
}
