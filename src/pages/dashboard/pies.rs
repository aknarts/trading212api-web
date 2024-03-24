use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
};

use crate::types::data::APIData;

#[function_component(PiesCard)]
pub fn pies() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };

    let onclick = { Callback::from(move |_| active.set(!*active)) };

    let data = (*api).clone();
    let pies = data.pies.get_complete_pies();

    if pies.is_empty() {
        return html! {
            <div class="accordion-item">
                <div class="accordion-header">
                    <button class={classes!("accordion-button", active_class.1)} type="button">
                        <span class="fs-4 me-2">{ "Pies "} </span>
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
                    <span class="fs-4 me-2">{ "Pies "} </span>
                    <span class={classes!("d-inline", "badge","rounded-pill", "text-bg-secondary")}>{pies.len()}</span>
                    {
                        // TODO: This can be a progress bar, we know the numbers here
                        if !data.pies.get_incomplete_ids().is_empty() {
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
                    <crate::pages::dashboard::pies_table::PiesTable />
                </div>
            </div>
        </div>
    )
}
