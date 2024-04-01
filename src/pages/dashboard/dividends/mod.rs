use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
    UseStateHandle,
};

use crate::types::data::APIData;
mod plot;
mod ticker_table;

#[function_component(DividendsCard)]
pub fn dividends() -> Html {
    let theme = use_context::<UseStateHandle<crate::types::theme::Theme>>().expect("no ctx found");
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_tab = use_state(|| 0);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let active_tab_id = *active_tab;
    let active_tab_class = match active_tab_id {
        0 => (("active", vec!["show", "active"]), ("", vec![])),
        1 => (("", vec![]), ("active", vec!["show", "active"])),
        _ => {
            unreachable!()
        }
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let dividends = data.dividends.dividends.clone();

    let handle_tabs = Callback::from(move |id: i32| {
        active_tab.set(id);
    });

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
                    <crate::components::tabs::Tabs tabs={vec!["Overview".to_string(), "Plot".to_string()]}>
                        <ticker_table::DividendsTickerTable/>
                        <plot::DividendsPlot/>
                    </crate::components::tabs::Tabs>
                </div>
            </div>
        </div>
    )
}
