use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;
use crate::pages::dashboard::dividends::DividendsCard;
use crate::pages::dashboard::exchanges::ExchangesCard;
use crate::pages::dashboard::instruments::InstrumentsCard;
use crate::pages::dashboard::positions::PositionsCard;

mod cash;
mod dividends;
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
            <DividendsCard />
            <ExchangesCard />
            <InstrumentsCard />
        </div>
    )
}
