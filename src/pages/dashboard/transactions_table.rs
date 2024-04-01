use rust_decimal::prelude::FromPrimitive;
use serde::Serialize;
use time::OffsetDateTime;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, TargetCast,
    UseReducerHandle,
};

use crate::components::table::types::{ColumnBuilder, TableData};
use crate::components::table::{Options, Table};
use crate::types::data::APIData;

#[function_component(TransactionsTable)]
pub fn transactions_table() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();

    let columns = vec![
        ColumnBuilder::new("date")
            .orderable(true)
            .short_name("Date")
            .data_property("date")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("type")
            .orderable(true)
            .short_name("Type")
            .data_property("type")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("amount")
            .orderable(true)
            .short_name("Amount")
            .data_property("amount")
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
    let transactions = data.transactions.clone();

    for (_, transaction) in &transactions.transactions {
        let account_currency = data
            .account
            .clone()
            .unwrap_or_default()
            .currency_code
            .clone();
        let line = TransactionLine {
            date: transaction.date_time,
            r#type: transaction.r#type,
            amount: transaction.amount,
            account_currency,
        };
        table_data.push(line);
    }

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

    html!(<>
            <div class="flex-grow-1 p-2 input-group mb-2">
                <span class="input-group-text">
                    <i class="fas fa-search"></i>
                </span>
                <input class="form-control" type="text" id="search" placeholder="Search" oninput={oninput_search} />
            </div>
            <Table<TransactionLine> {options} {search} classes={classes!("table", "table-hover")} columns={columns} data={table_data} orderable={true}/>
        </>)
}

#[derive(Clone, Serialize, Default, Debug)]
pub struct Ppl {
    pub ppl: f32,
    pub fx_impact: Option<f32>,
}

#[derive(Clone, Serialize, Debug)]
struct TransactionLine {
    pub date: OffsetDateTime,
    pub r#type: trading212::models::history_transaction_item::Type,
    pub amount: f32,
    pub account_currency: String,
}

impl Default for TransactionLine {
    fn default() -> Self {
        Self {
            date: OffsetDateTime::now_utc(),
            r#type: trading212::models::history_transaction_item::Type::Unknown,
            amount: 0.0,
            account_currency: "EUR".to_string(),
        }
    }
}

impl PartialEq<Self> for TransactionLine {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.r#type == other.r#type
    }
}

impl PartialOrd for TransactionLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl TableData for TransactionLine {
    fn get_field_as_html(&self, field_name: &str) -> crate::components::table::error::Result<Html> {
        let html = match field_name {
            "date" => {
                html! { { self.date.to_string() } }
            }
            "type" => {
                html! { { format!("{:?}", self.r#type) } }
            }
            "amount" => {
                let currency = rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR)
                    .clone();
                let amount = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.amount).unwrap_or_default(),
                    &currency,
                );
                html! { amount.to_string() }
            }
            &_ => html!(),
        };
        Ok(html)
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> crate::components::table::error::Result<serde_value::Value> {
        let value = match field_name {
            "date" => serde_value::to_value(self.date.to_string()),
            "type" => serde_value::to_value(format!("{:?}", self.r#type)),
            "amount" => serde_value::to_value(self.amount),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.date
                .to_string()
                .to_lowercase()
                .contains(&search.to_lowercase())
                || format!("{:?}", self.r#type)
                    .to_lowercase()
                    .contains(&search.to_lowercase())
        })
    }
}
