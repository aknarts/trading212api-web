use rust_decimal::prelude::FromPrimitive;
use yew::{
    Callback, classes, function_component, html, Html, use_context, use_state, UseReducerHandle,
};

use crate::types::data::APIData;

#[function_component(CashCard)]
pub fn cash() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let html_result = match data.cash {
        Some(cash) => {
            let currency = rusty_money::iso::find(&data.account.unwrap_or_default().currency_code)
                .unwrap_or(rusty_money::iso::EUR);
            let all_free = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.free).unwrap_or_default(),
                currency,
            );
            let pie_free = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.pie_cash).unwrap_or_default(),
                currency,
            );
            let total = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.total).unwrap_or_default(),
                currency,
            );

            let invested = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.invested).unwrap_or_default(),
                currency,
            );
            let ppl = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.ppl).unwrap_or_default(),
                currency,
            );
            let blocked = rusty_money::Money::from_decimal(
                rust_decimal::Decimal::from_f32(cash.blocked.unwrap_or_default())
                    .unwrap_or_default(),
                currency,
            );
            let ppl_class = if ppl.is_positive() {
                "text-bg-success"
            } else {
                "text-bg-danger"
            };
            let available = all_free - pie_free.clone();
            html!(
                <div class="accordion-item">
                    <div class="accordion-header">
                        <button class={classes!("accordion-button", active_class.1)} type="button" {onclick}>
                            <span class="fs-4 me-2">{total.to_string()}</span>
                            <span class={classes!("d-inline", "badge","rounded-pill", ppl_class)}>{ppl.to_string()}</span>
                        </button>
                    </div>
                    <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                        <div class="accordion-body">
                            <div class="row">
                                <div class="col-6 col-sm-4">
                                    <div class="row">
                                        <div class="col-3">{"Invested"}</div>
                                        <div class="col-2 col-sm-2">
                                            <span class="badge text-bg-secondary p-2">{invested.to_string()}</span>
                                        </div>
                                    </div>
                                    <div class="row">
                                        <div class="col-3">{"Return"}</div>
                                        <div class="col-2 col-sm-2"><span class="badge text-bg-secondary p-2">{ppl.to_string()}</span></div>
                                    </div>
                                    <div class="row">
                                        <div class="col-3">{"Result"}</div>
                                        <div class="col-2 col-sm-2">
                                            <span class="badge text-bg-secondary p-2">{cash.result}</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-6 col-sm-4">
                                    <div class="row">
                                        <div class="col-4">{"Free cash"}</div>
                                        <div class="col-2 col-sm-2">
                                            <span class="badge text-bg-secondary p-2">{available.to_string()}</span>
                                        </div>
                                    </div>
                                    <div class="row">
                                        <div class="col-4">{"Pies cash"}</div>
                                        <div class="col-2 col-sm-2">
                                            <span class="badge text-bg-secondary p-2">{pie_free.to_string()}</span>
                                        </div>
                                    </div>
                                    <div class="row">
                                        <div class="col-4">{"Blocked"}</div>
                                        <div class="col-2 col-sm-2">
                                            <span class="badge text-bg-secondary p-2">{blocked.to_string()}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            )
        }

        None => {
            html!(
                <div class="card">
                    <div class="card-body">
                        <div class="d-flex justify-content-center">
                            <div class="spinner-border" role="status">
                                <span class="visually-hidden">{ "Loading..." }</span>
                            </div>
                        </div>
                    </div>
                </div>
            )
        }
    };

    html_result
}
