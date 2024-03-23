use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
};

use crate::types::data::APIData;

#[function_component(DividendsCard)]
pub fn dividends() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let dividends = data.dividends.dividends;

    if dividends.is_empty() {
        return html! {
            <div class="accordion-item">
                <div class="accordion-header">
                    <button class={classes!("accordion-button", active_class.1)} type="button">
                        <span class="fs-4 me-2">{ "Dividends "} </span>
                        <span class="spinner-border" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                        </span>
                    </button>
                </div>
            </div>
        };
    }

    html!(
        <div class="accordion-item">
            <div class="accordion-header">
                <button class={classes!("accordion-button", active_class.1)} type="button" {onclick}>
                    <span class="fs-4 me-2">{ "Dividends "} </span>
                    <span class={classes!("d-inline", "badge","rounded-pill", "text-bg-secondary")}>{dividends.len()}</span>
                    {
                        if !data.dividends.loaded {
                            html!{
                                <span class="spinner-border" role="status">
                                     <span class="visually-hidden">{ "Loading..." }</span>
                                </span>
                            }
                        } else {
                            html!{<></>}
                        }
                    }
                </button>
            </div>
            <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                <div class="accordion-body">
                    <crate::pages::dashboard::dividends_ticker_table::DividendsTickerTable/>
                </div>
            </div>
        </div>
    )
}
