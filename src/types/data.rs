use trading212::models::exchange::Exchange;

#[derive(serde::Serialize, Clone, Debug, Default, PartialEq)]
pub struct APIData {
    pub cash: Option<trading212::models::cash::Cash>,
    pub timeouts: std::collections::HashMap<String, time::OffsetDateTime>,
    pub account: Option<trading212::models::account::Account>,
    pub exchanges: Vec<Exchange>,
}

pub enum APIDataAction {
    Init,
    SetCash(Option<trading212::models::cash::Cash>),
    SetAccount(Option<trading212::models::account::Account>),
    SetExchanges(Vec<Exchange>),
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
        }
        new.into()
    }
}
