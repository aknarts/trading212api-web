use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;
use crate::pages::dashboard::exchanges::ExchangesCard;
use crate::pages::dashboard::instruments::InstrumentsCard;
use crate::pages::dashboard::positions::PositionsCard;

mod cash;
pub mod exchange;
mod exchanges;
mod instruments;
mod positions;
pub mod positions_table;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html!(
        <div class="accordion">
            <CashCard />
            <PositionsCard />
            <ExchangesCard />
            <InstrumentsCard />
        </div>
    )
}
