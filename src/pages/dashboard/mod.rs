use yew::{function_component, html, Html};

use crate::pages::dashboard::cash::CashCard;
use crate::pages::dashboard::dividends::DividendsCard;
use crate::pages::dashboard::exchanges::ExchangesCard;
use crate::pages::dashboard::instruments::InstrumentsCard;
use crate::pages::dashboard::orders::OrdersCard;
use crate::pages::dashboard::pies::PiesCard;
use crate::pages::dashboard::positions::PositionsCard;
use crate::pages::dashboard::transactions::TransactionsCard;

mod cash;
mod dividends;
pub mod exchange;
mod exchanges;
mod instruments;
mod orders;
pub mod pies;
pub mod pies_table;
mod positions;
pub mod positions_table;
mod transactions;
mod transactions_table;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html!(
        <div class="accordion">
            <CashCard />
            <PositionsCard />
            <DividendsCard />
            <PiesCard />
            <TransactionsCard />
            <OrdersCard />
            <ExchangesCard />
            <InstrumentsCard />

        </div>
    )
}
