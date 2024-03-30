use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
};

use crate::types::data::APIData;

#[function_component(TransactionsCard)]
pub fn transactions() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let transactions = data.transactions.transactions;

    if transactions.is_empty() {
        return html! {
            <div class="accordion-item">
                <div class="accordion-header">
                    <button class={classes!("accordion-button", active_class.1)} type="button">
                        <span class="fs-4 me-2">{ "Transactions "} </span>
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
                    <span class="fs-4 me-2">{ "Transactions "} </span>
                    <span class={classes!("d-inline", "badge","rounded-pill", "text-bg-secondary")}>{transactions.len()}</span>
                </button>
            </div>
            <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                <div class="accordion-body">
                    <crate::pages::dashboard::transactions_table::TransactionsTable />
                </div>
            </div>
        </div>
    )
}
