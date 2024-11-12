use rust_decimal::prelude::FromPrimitive;
use serde::Serialize;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, TargetCast,
    UseReducerHandle,
};

use crate::types::data::APIData;
use yew_custom_components::table::types::{ColumnBuilder, TableData};
use yew_custom_components::table::{Options, Table};

#[function_component(PiesTable)]
pub fn pies_table() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();

    let columns = vec![
        ColumnBuilder::new("name")
            .orderable(true)
            .short_name("Name")
            .data_property("name")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("cash")
            .orderable(true)
            .short_name("Cash")
            .data_property("cash")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("value")
            .orderable(true)
            .short_name("Value")
            .data_property("value")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("ppl")
            .orderable(true)
            .short_name("P/L")
            .data_property("ppl")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("dividends")
            .orderable(true)
            .short_name("Dividends")
            .data_property("dividends")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("mininvest")
            .orderable(true)
            .short_name("Min. Investment")
            .data_property("mininvest")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("progress")
            .orderable(true)
            .short_name("Goal")
            .data_property("progress")
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
    let pies = data.pies.get_complete_pies();

    for pie in &pies {
        let account_currency = data
            .account
            .clone()
            .unwrap_or_default()
            .currency_code
            .clone();

        let pie_mininvest = pie
            .details
            .clone()
            .unwrap_or_default()
            .instruments
            .iter()
            .map(|i| i.expected_share)
            .fold(f32::MAX, |a, b| a.min(b));
        let line = PieLine {
            name: pie
                .details
                .clone()
                .unwrap_or_default()
                .settings
                .name
                .clone(),
            icon: pie.details.clone().unwrap_or_default().settings.icon,
            cash: pie.data.cash,
            mininvest: 1.0 / pie_mininvest,
            result: pie.data.result.clone(),
            dividend: pie.data.dividend_details.clone(),
            progress: pie.data.progress,
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
            <Table<PieLine> {options} {search} classes={classes!("table", "table-hover")} columns={columns} data={table_data} orderable={true}/>
        </>)
}

#[derive(Clone, Serialize, Default, Debug)]
pub struct Ppl {
    pub ppl: f32,
    pub fx_impact: Option<f32>,
}

#[derive(Clone, Serialize, Debug, Default)]
struct PieLine {
    pub name: String,
    pub icon: Option<trading212::models::icon::Icon>,
    pub cash: f32,
    pub mininvest: f32,
    pub result: trading212::models::investment_result::InvestmentResult,
    pub dividend: trading212::models::dividend_details::DividendDetails,
    pub progress: Option<f32>,
    pub account_currency: String,
}

impl PartialEq<Self> for PieLine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for PieLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl TableData for PieLine {
    fn get_field_as_html(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<Html> {
        let html = match field_name {
            "name" => {
                html! { { &self.name } }
            }
            "cash" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let cash = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.cash).unwrap_or_default(),
                    &currency,
                );
                html! { { cash.to_string() } }
            }
            "ppl" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let ppl = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.result.result).unwrap_or_default(),
                    &currency,
                );
                let ppl_class = if ppl.is_positive() {
                    "text-bg-success"
                } else {
                    "text-bg-danger"
                };
                html! { <span class={classes!("badge", "rounded-pill", ppl_class)}>{ ppl.to_string() }</span> }
            }
            "value" => html! { { &self.result.value } },
            "dividends" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let dividends = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.dividend.gained).unwrap_or_default(),
                    &currency,
                );
                html! { { dividends.to_string() } }
            }
            "mininvest" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let mininvest = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.mininvest).unwrap_or_default(),
                    &currency,
                );
                html! { { mininvest.to_string() } }
            }
            "progress" => match self.progress {
                None => {
                    html! { <></> }
                }
                Some(progress) => {
                    html! { { format!("{:.2}%", progress*100.0) } }
                }
            },
            &_ => html!(),
        };
        Ok(html)
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<serde_value::Value> {
        let value = match field_name {
            "name" => serde_value::to_value(&self.name),
            "cash" => serde_value::to_value(self.cash),
            "ppl" => serde_value::to_value(self.result.result),
            "value" => serde_value::to_value(self.result.value),
            "dividends" => serde_value::to_value(self.dividend.gained),
            "mininvest" => serde_value::to_value(self.mininvest),
            "progress" => serde_value::to_value(self.progress.unwrap_or_default()),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.name.to_lowercase().contains(&search.to_lowercase())
        })
    }
}
