use gloo_storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use std::sync::RwLock;
use tracing::warn;
use trading212::models::history_dividend_item::Type;

const CACHE_KEY: &str = "trading212api.cache";

lazy_static! {
    pub static ref CACHE: RwLock<Option<APIData>> = {
        LocalStorage::get(CACHE_KEY)
            .map_or_else(|_| RwLock::new(None), |data| RwLock::new(Some(data)))
    };
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct APIData {
    pub cash: Option<trading212::models::cash::Cash>,
    pub timeouts: std::collections::HashMap<String, time::OffsetDateTime>,
    pub account: Option<trading212::models::account::Account>,
    pub exchanges: Vec<trading212::models::exchange::Exchange>,
    pub instruments: Vec<trading212::models::tradeable_instrument::TradeableInstrument>,
    pub positions: Vec<trading212::models::position::Position>,
    pub dividends: crate::types::dividend::DividendData,
    pub transactions: crate::types::transaction::TransactionData,
    pub pies: crate::types::pie::PiesData,
    pub orders: crate::types::order::OrderData,
}

pub enum APIDataAction {
    Init,
    Save,
    SetCash(Option<trading212::models::cash::Cash>),
    SetAccount(Option<trading212::models::account::Account>),
    SetExchanges(Vec<trading212::models::exchange::Exchange>),
    SetInstruments(Vec<trading212::models::tradeable_instrument::TradeableInstrument>),
    SetPositions(Vec<trading212::models::position::Position>),
    AddDividend(trading212::models::history_dividend_item::HistoryDividendItem),
    SetDividendsCursor(Option<i64>),
    SetDividendsLoaded(bool),
    AddPie(trading212::models::account_bucket_result_response::AccountBucketResultResponse),
    AddPieDetails(
        i64,
        trading212::models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse,
    ),
    AddTransaction(trading212::models::history_transaction_item::HistoryTransactionItem),
    SetTransactionsCursor(Option<(String, String)>),
    SetTransactionsLoaded(bool),
    AddOrder(trading212::models::historical_order::HistoricalOrder),
    SetOrdersCursor(Option<i64>),
    SetOrdersLoaded(bool),
}

impl Default for APIData {
    fn default() -> Self {
        if let Ok(d) = CACHE.read() {
            if let Some(data) = d.clone() {
                return data;
            }
        };
        Self {
            cash: Default::default(),
            timeouts: Default::default(),
            account: Default::default(),
            exchanges: Default::default(),
            instruments: Default::default(),
            positions: Default::default(),
            dividends: Default::default(),
            transactions: Default::default(),
            pies: Default::default(),
            orders: Default::default(),
        }
    }
}

impl APIData {
    pub fn save(&self) {
        if let Ok(mut d) = CACHE.write() {
            *d = Some(self.clone());
            LocalStorage::set(CACHE_KEY, self).expect("Failed to save cache");
        }
    }

    pub fn get_instrument_by_ticker(
        &self,
        ticker: &str,
    ) -> Option<trading212::models::tradeable_instrument::TradeableInstrument> {
        self.instruments
            .iter()
            .find(|i| i.ticker.eq(ticker))
            .cloned()
    }

    pub fn get_position_by_ticker(
        &self,
        ticker: &str,
    ) -> Option<trading212::models::position::Position> {
        self.positions.iter().find(|p| p.ticker.eq(ticker)).cloned()
    }
}

impl yew::Reducible for APIData {
    type Action = APIDataAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new = (*self).clone();
        match action {
            APIDataAction::Init => {
                new = Self::default();
            }
            APIDataAction::Save => {
                (*self).save();
                return self;
            }
            APIDataAction::SetCash(cash) => {
                new.cash = cash;
                new.timeouts
                    .insert("cash".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetAccount(account) => {
                new.account = account;
                new.timeouts
                    .insert("account".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetExchanges(exchanges) => {
                new.exchanges = exchanges;
                new.timeouts
                    .insert("account".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetInstruments(instruments) => {
                new.instruments = instruments;
                new.timeouts
                    .insert("instruments".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetPositions(positions) => {
                new.positions = positions;
                new.timeouts
                    .insert("positions".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::AddDividend(item) => {
                if let Type::Unknown = item.r#type {
                    warn!("Unknown dividend type, most likely a new type: {:?}", item)
                }
                new.dividends.add(item);
                new.timeouts
                    .insert("dividends".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetDividendsCursor(cursor) => {
                new.dividends.set_cursor(cursor);
            }
            APIDataAction::SetDividendsLoaded(loaded) => {
                new.dividends.set_loaded(loaded);
            }
            APIDataAction::AddPie(pie) => {
                new.pies.add_pie(pie);
                new.timeouts
                    .insert("pies".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::AddPieDetails(id, details) => {
                new.pies.add_detail(id, details);
                new.timeouts
                    .insert("pie_details".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::AddTransaction(transaction) => {
                new.transactions.add(transaction);
                new.timeouts
                    .insert("transactions".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetTransactionsCursor(cursor) => {
                new.transactions.set_cursor(cursor);
            }
            APIDataAction::SetTransactionsLoaded(loaded) => {
                new.transactions.set_loaded(loaded);
            }
            APIDataAction::AddOrder(order) => {
                new.orders.add(order);
                new.timeouts
                    .insert("orders".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetOrdersCursor(cursor) => {
                new.orders.set_cursor(cursor);
            }
            APIDataAction::SetOrdersLoaded(loaded) => {
                new.orders.set_loaded(loaded);
            }
        }
        new.into()
    }
}
