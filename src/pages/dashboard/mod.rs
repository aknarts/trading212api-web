use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;

pub mod cash;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html!(
        <div class="accordion">
            <CashCard />
        </div>
    )
}
