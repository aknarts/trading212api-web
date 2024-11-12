use crate::types::data::APIData;
use rust_decimal::prelude::FromPrimitive;
use serde::Serialize;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, TargetCast,
    UseReducerHandle,
};
use yew_custom_components::pagination::Pagination;
use yew_custom_components::table::types::{ColumnBuilder, TableData};
use yew_custom_components::table::{Options, Table};

#[function_component(PositionsTable)]
pub fn positions_table() -> Html {
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let search_term = use_state(|| None::<String>);
    let search = (*search_term).as_ref().cloned();
    let page = use_state(|| 0usize);
    let current_page = *page;

    let columns = vec![
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
        ColumnBuilder::new("ppl")
            .orderable(true)
            .short_name("P/L")
            .data_property("ppl")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("quantity")
            .orderable(true)
            .short_name("Quantity")
            .data_property("quantity")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("avg_price")
            .orderable(true)
            .short_name("AVG Price")
            .data_property("avg_price")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("price")
            .orderable(true)
            .short_name("Price")
            .data_property("price")
            .header_class("user-select-none")
            .build(),
        ColumnBuilder::new("ac")
            .orderable(true)
            .short_name("Under AVG")
            .data_property("ac")
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
    let positions = data.positions.clone();

    for position in &positions {
        let account_currency = data
            .account
            .clone()
            .unwrap_or_default()
            .currency_code
            .clone();
        let instrument = data.get_instrument_by_ticker(&position.ticker);
        let line = PositionLine {
            ticker: position.ticker.clone(),
            ppl: Ppl {
                ppl: position.ppl,
                fx_impact: position.fx_ppl,
            },
            quantity: position.quantity,
            avg_price: position.average_price,
            price: position.current_price,
            ac: position.average_price / position.current_price,
            instrument: instrument.clone(),
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
            <Table<PositionLine> {options} {search} limit={Some(10)} page={current_page} classes={classes!("table", "table-hover")} columns={columns} data={table_data.clone()} orderable={true}/>
            <Pagination total={table_data.len()} max_pages={5} limit={10} options={pagination_options} on_page={Some(handle_page)}/>
        </>)
}

#[derive(Clone, Serialize, Default, Debug)]
pub struct Ppl {
    pub ppl: f32,
    pub fx_impact: Option<f32>,
}

#[derive(Clone, Serialize, Debug, Default)]
struct PositionLine {
    pub ticker: String,
    pub ppl: Ppl,
    pub quantity: Option<f32>,
    pub avg_price: f32,
    pub price: f32,
    pub ac: f32,
    pub instrument: Option<trading212::models::tradeable_instrument::TradeableInstrument>,
    pub account_currency: String,
}

impl PartialEq<Self> for PositionLine {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker && self.instrument == other.instrument
    }
}

impl PartialOrd for PositionLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ticker.partial_cmp(&other.ticker)
    }
}

impl TableData for PositionLine {
    fn get_field_as_html(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<Html> {
        let html = match field_name {
            "ticker" => {
                let ticker = match &self.instrument {
                    None => self.ticker.clone(),
                    Some(i) => i.short_name.clone(),
                };
                html! { { ticker } }
            }
            "instrument" => {
                let instrument = match &self.instrument {
                    None => "".to_string(),
                    Some(i) => i.name.clone(),
                };
                html! { { instrument } }
            }
            "ppl" => {
                let currency = *rusty_money::iso::find(&self.account_currency)
                    .unwrap_or(rusty_money::iso::EUR);
                let ppl = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.ppl.ppl).unwrap_or_default(),
                    &currency,
                );
                let ppl_class = if ppl.is_positive() {
                    "text-bg-success"
                } else {
                    "text-bg-danger"
                };
                html! { <span class={classes!("badge", "rounded-pill", ppl_class)}>{ ppl.to_string() }</span> }
            }
            "quantity" => html! { { &self.quantity.unwrap_or_default() } },
            "avg_price" => {
                let currency = match &self.instrument {
                    None => *rusty_money::iso::EUR,
                    Some(i) => {
                        *rusty_money::iso::find(&i.currency_code).unwrap_or(rusty_money::iso::EUR)
                    }
                };
                let avg_price = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.avg_price).unwrap_or_default(),
                    &currency,
                );
                html! { { avg_price.to_string() } }
            }
            "price" => {
                let currency = match &self.instrument {
                    None => *rusty_money::iso::EUR,
                    Some(i) => {
                        *rusty_money::iso::find(&i.currency_code).unwrap_or(rusty_money::iso::EUR)
                    }
                };
                let price = rusty_money::Money::from_decimal(
                    rust_decimal::Decimal::from_f32(self.price).unwrap_or_default(),
                    &currency,
                );
                html! { { price.to_string() } }
            }
            "ac" => {
                let class = if self.ac > 1.0 {
                    "bg-success"
                } else {
                    "bg-danger"
                };
                html! {
                    html! { <span class={classes!("badge", "rounded-pill", class)}>{ format!("{:.2}",self.ac) }</span> }
                }
            }
            &_ => html!(),
        };
        Ok(html)
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<serde_value::Value> {
        let value = match field_name {
            "ticker" => serde_value::to_value(&self.ticker),
            "instrument" => {
                serde_value::to_value(&self.instrument.clone().unwrap_or_default().name)
            }
            "ppl" => serde_value::to_value(self.ppl.ppl.to_string()),
            "quantity" => serde_value::to_value(self.quantity),
            "avg_price" => serde_value::to_value(self.avg_price),
            "price" => serde_value::to_value(self.price),
            "ac" => serde_value::to_value(self.ac),
            &_ => serde_value::to_value(""),
        };
        Ok(value.unwrap())
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        needle.map_or(true, |search| {
            self.ticker.to_lowercase().contains(&search.to_lowercase())
                || self
                    .instrument
                    .clone()
                    .unwrap_or_default()
                    .name
                    .to_lowercase()
                    .contains(&search.to_lowercase())
        })
    }
}
