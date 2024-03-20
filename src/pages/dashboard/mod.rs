use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;
use crate::pages::dashboard::exchanges::ExchangesCard;
use crate::pages::dashboard::instruments::InstrumentsCard;

mod cash;
pub mod exchange;
mod exchanges;
mod instruments;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html!(
        <div class="accordion">
            <CashCard />
            <ExchangesCard />
            <InstrumentsCard />
        </div>
    )
}
