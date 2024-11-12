use tracing::{error, warn};
use trading212::error::Error;
use yew::prelude::*;

use crate::hooks::use_user_context::Handle;

#[function_component(PiesRefresher)]
pub fn pies_refresher() -> Html {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    let data =
        use_context::<UseReducerHandle<crate::types::data::APIData>>().expect("no ctx found");
    {
        let dispatcher = data.dispatcher();
        let ctx = user_ctx.clone();

        yew_hooks::use_mount(move || {
            refresh(dispatcher.clone(), ctx.clone());
        });
        {
            let dispatcher = data.dispatcher();
            let user_ctx = user_ctx.clone();
            yew_hooks::use_interval(
                move || {
                    let dispatcher = dispatcher.clone();
                    let user_ctx = user_ctx.clone();
                    refresh(dispatcher, user_ctx);
                },
                38000,
            );
        }

        {
            let data = data.clone();
            let dispatcher = data.dispatcher();
            let user_ctx = user_ctx.clone();
            yew_hooks::use_interval(
                move || {
                    let dispatcher = dispatcher.clone();
                    let user_ctx = user_ctx.clone();
                    match data.pies.get_incomplete_ids().first() {
                        Some(id) => {
                            load_details(dispatcher, user_ctx, *id);
                        }
                        None => {
                            if let Some(id) = data.pies.get_oldest_updated_id() {
                                load_details(dispatcher, user_ctx, id);
                            }
                        }
                    };
                },
                9000,
            );
        }
    }
    html! { <></> }
}

fn refresh(dispatcher: UseReducerDispatcher<crate::types::data::APIData>, user_ctx: Handle) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(c) = user_ctx.client() {
            match c.get_all_pies().await {
                Ok(pies) => {
                    for pie in pies.iter() {
                        dispatcher.dispatch(crate::types::data::APIDataAction::AddPie(pie.clone()));
                    }
                }
                Err(e) => {
                    if let Error::Limit = e {
                        warn!("Failed to fetch pies, limit");
                    } else {
                        error!("Failed to fetch pies: {:?}", e);
                    }
                }
            }
        }
    });
}

fn load_details(
    dispatcher: UseReducerDispatcher<crate::types::data::APIData>,
    user_ctx: Handle,
    id: i64,
) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(c) = user_ctx.client() {
            match c.get_pie(id).await {
                Ok(pie) => {
                    dispatcher.dispatch(crate::types::data::APIDataAction::AddPieDetails(
                        id,
                        pie.clone(),
                    ));
                }
                Err(e) => {
                    error!("Failed to fetch pie details: {:?}", e);
                }
            }
        }
    });
}
