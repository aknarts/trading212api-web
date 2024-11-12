use rust_decimal::prelude::FromPrimitive;
use serde::Serialize;
use time::OffsetDateTime;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, TargetCast,
    UseReducerHandle,
};
use yew_custom_components::pagination::Pagination;

use crate::types::data::APIData;
use yew_custom_components::table::types::{ColumnBuilder, TableData};
use yew_custom_components::table::{Options, Table};

#[function_component(DividendsHistoryTable)]
pub fn dividends_history_table() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();
    let page = use_state(|| 0usize);
    let current_page = *page;

    let columns = vec![
        ColumnBuilder::new("date")
            .orderable(true)
            .short_name("Date")
            .data_property("date")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("ticker")
            .orderable(true)
            .short_name("Ticker")
            .data_property("ticker")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("instrument")
            .orderable(true)
            .short_name("Name")
            .data_property("instrument")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("amount")
            .orderable(true)
            .short_name("Amount")
            .data_property("amount")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("quantity")
            .orderable(true)
            .short_name("Quantity")
            .data_property("quantity")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("per_share")
            .orderable(true)
            .short_name("Per Share")
            .data_property("per_share")
            .header_class("user-select-none")
            .build(),
    ];

    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };

    let mut table_data = Vec::new();

    let data = (*api).clone();

    for item in data.dividends.dividends.values() {
        let ticker = item.ticker.clone();
        let account_currency = data
            .account
            .clone()
            .unwrap_or_default()
            .currency_code
            .clone();
        let instrument = match data.get_instrument_by_ticker(&ticker) {
            Some(i) => i,
            None => {
                let mut t = trading212::models::tradeable_instrument::TradeableInstrument::new();
                t.short_name = ticker.clone();
                t
            }
        };

        table_data.push(DividendLine {
            date: item.paid_on,
            ticker: instrument.short_name,
            instrument: instrument.name,
            amount: item.amount,
            quantity: item.quantity,
            per_share: item.gross_amount_per_share,
            account_currency,
        });
    }

    table_data.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap().reverse());

    let oninput_search = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if input.value().is_empty() {
                search_term.set(None);
            } else {
                search_term.set(Some(input.value()));
            }
        })
    };

    let pagination_options = yew_custom_components::pagination::Options::default()
        .show_prev_next(true)
        .show_first_last(true)
        .list_classes(vec![String::from("pagination")])
        .item_classes(vec![String::from("page-item")])
        .link_classes(vec![String::from("page-link")])
        .active_item_classes(vec![String::from("active")])
        .disabled_item_classes(vec![String::from("disabled")]);

    let handle_page = {
        let page = page.clone();
        Callback::from(move |id: usize| {
            page.set(id);
        })
    };

    html!(<>
            <div class="flex-grow-1 p-2 input-group mb-2">
                <span class="input-group-text">
                    <i class="fas fa-search"></i>
                </span>
                <input class="form-control" type="text" id="search" placeholder="Search" oninput={oninput_search} />
            </div>
            <Table<DividendLine> options={options.clone()} limit={Some(10)} page={current_page} search={search.clone()} classes={classes!("table", "table-hover")} columns={columns.clone()} data={table_data.clone()} orderable={true}/>
            <Pagination total={table_data.len()} max_pages={5} limit={10} options={pagination_options} on_page={Some(handle_page)}/>
        </>)
}

#[derive(Clone, Serialize, Debug)]
struct DividendLine {
    pub date: OffsetDateTime,
    pub ticker: String,
    pub instrument: String,
    pub amount: f32,
    pub account_currency: String,
    pub quantity: Option<f32>,
    pub per_share: Option<f32>,
}

impl Default for DividendLine {
    fn default() -> Self {
        Self {
            date: OffsetDateTime::UNIX_EPOCH,
            ticker: String::new(),
            instrument: String::new(),
            amount: 0.0,
            account_currency: String::new(),
            quantity: None,
            per_share: None,
        }
    }
}

impl PartialEq<Self> for DividendLine {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker && self.instrument == other.instrument
    }
}

impl PartialOrd for DividendLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ticker.partial_cmp(&other.ticker)
    }
}

impl TableData for DividendLine {
    fn get_field_as_html(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<Html> {
        let value = match field_name {
            "date" => {
                let format = time::format_description::parse(
                    "[year]-[month]-[day] [hour]:[minute]:[second]",
                )
                .unwrap();
                html! { <span>{&self.date.format(&format).unwrap()}</span> }
            }
            "ticker" => html! { <span>{&self.ticker}</span> },
            "instrument" => html! { <span>{&self.instrument.clone()}</span> },
            "quantity" => match self.quantity {
                None => {
                    html! { <></> }
                }
                Some(quantity) => {
                    html! { <span>{quantity}</span> }
                }
            },
            "amount" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let sum = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.amount).unwrap_or_default(),
                    &currency,
                );
                html! { <span>{sum.to_string()}</span> }
            }
            "per_share" => match self.per_share {
                None => {
                    html! { <></> }
                }
                Some(per_share) => {
                    html! { <span>{per_share}</span> }
                }
            },
            &_ => html! { <></> },
        };
        Ok(value)
    }
    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<serde_value::Value> {
        let value = match field_name {
            "date" => serde_value::to_value(self.date),
            "ticker" => serde_value::to_value(&self.ticker),
            "instrument" => serde_value::to_value(&self.instrument),
            "amount" => serde_value::to_value(self.amount),
            "quantity" => match self.quantity {
                Some(quantity) => serde_value::to_value(quantity),
                None => serde_value::to_value(0.0),
            },
            "per_share" => serde_value::to_value(self.per_share.unwrap_or_default()),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.ticker.to_lowercase().contains(&search.to_lowercase())
                || self
                    .instrument
                    .to_lowercase()
                    .contains(&search.to_lowercase())
        })
    }
}
