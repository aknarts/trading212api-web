use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;
use crate::pages::dashboard::exchanges::ExchangesCard;

pub mod cash;
pub mod exchange;
pub mod exchanges;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html!(
        <div class="accordion">
            <CashCard />
            <ExchangesCard />
        </div>
    )
}
