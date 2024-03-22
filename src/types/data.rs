use tracing::warn;
use trading212::models::history_dividend_item::Type;
use uuid::Uuid;

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct DividendData {
    pub dividends: std::collections::HashMap<
        Uuid,
        trading212::models::history_dividend_item::HistoryDividendItem,
    >,
    pub dividends_cursor: Option<i64>,
    pub loaded: bool,
}

impl Default for DividendData {
    fn default() -> Self {
        Self {
            dividends: Default::default(),
            dividends_cursor: Default::default(),
            loaded: false,
        }
    }
}

#[derive(serde::Serialize, Clone, Debug, Default, PartialEq)]
pub struct APIData {
    pub cash: Option<trading212::models::cash::Cash>,
    pub timeouts: std::collections::HashMap<String, time::OffsetDateTime>,
    pub account: Option<trading212::models::account::Account>,
    pub exchanges: Vec<trading212::models::exchange::Exchange>,
    pub instruments: Vec<trading212::models::tradeable_instrument::TradeableInstrument>,
    pub positions: Vec<trading212::models::position::Position>,
    pub dividends: DividendData,
}

pub enum APIDataAction {
    Init,
    SetCash(Option<trading212::models::cash::Cash>),
    SetAccount(Option<trading212::models::account::Account>),
    SetExchanges(Vec<trading212::models::exchange::Exchange>),
    SetInstruments(Vec<trading212::models::tradeable_instrument::TradeableInstrument>),
    SetPositions(Vec<trading212::models::position::Position>),
    AddDividend(trading212::models::history_dividend_item::HistoryDividendItem),
    SetDividendsCursor(Option<i64>),
}

impl APIData {
    pub fn get_instrument_by_ticker(
        &self,
        ticker: &str,
    ) -> Option<trading212::models::tradeable_instrument::TradeableInstrument> {
        self.instruments
            .iter()
            .find(|i| i.ticker.eq(ticker))
            .cloned()
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
                new.dividends.dividends.insert(item.reference, item);
                new.timeouts
                    .insert("dividends".to_string(), time::OffsetDateTime::now_utc());
            }
            APIDataAction::SetDividendsCursor(cursor) => {
                new.dividends.dividends_cursor = cursor;
            }
        }
        new.into()
    }
}
