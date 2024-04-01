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

    let incomplete_num = data.pies.get_incomplete_ids().len();
    let complete_num = data.pies.pies.len();
    let progress = (complete_num - incomplete_num) as f64 / complete_num as f64 * 100.0;

    html!(
        <div class="accordion-item">
            <div class="accordion-header">
                <button class={classes!("accordion-button", active_class.1)} type="button" {onclick}>
                    <span class="fs-4 me-2">{ "Pies "} </span>
                    <span class={classes!("d-inline", "badge","rounded-pill", "text-bg-secondary")}>{
                        if pies.len() == complete_num {
                            format!("{}", pies.len())
                        } else {
                            format!("{}/{}",pies.len(), complete_num)
                        }
                    }</span>
                </button>
                {
                    if incomplete_num > 0 {
                        html!{
                            <div class="progress" style="height: 2px">
                              <div class="progress-bar progress-bar-striped progress-bar-animated" style={format!("width: {}%", progress)}></div>
                            </div>
                        }
                    } else {
                        html!{<></>}
                    }
                }
            </div>
            <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                <div class="accordion-body">
                    <ul class="nav nav-tabs">
                        <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">{"Overview"}</a>
                        </li>
                    </ul>
                    <div class="tab-content">
                        <div class="tab-pane fade show active" id="overview">
                            <crate::pages::dashboard::pies_table::PiesTable />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
