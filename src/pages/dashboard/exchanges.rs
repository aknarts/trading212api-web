use std::cmp::Ordering;

use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
};

use crate::types::data::APIData;

#[function_component(ExchangesCard)]
pub fn exchanges() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let mut exchanges = data.exchanges;
    exchanges.sort_by(|a, b| match a.current_type().cmp(&b.current_type()) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.name.cmp(&b.name),
        Ordering::Greater => Ordering::Greater,
    });

    if exchanges.is_empty() {
        return html! {
            <div class="accordion-item">
                <div class="accordion-header">
                    <button class={classes!("accordion-button", active_class.1)} type="button">
                        <span class="fs-4 me-2">{ "Exchanges "} </span>
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
                    <span class="fs-4 me-2">{ "Exchanges "} </span>
                    <span class={classes!("d-inline", "badge","rounded-pill", "text-bg-secondary")}>{exchanges.len()}</span>
                </button>
            </div>
            <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                <div class="accordion-body">
                    <div class="accordion">
                        { for exchanges.iter().map(|exchange| {
                            html! {
                                <crate::pages::dashboard::exchange::Exchange exchange={exchange.clone()} />
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    )
}
