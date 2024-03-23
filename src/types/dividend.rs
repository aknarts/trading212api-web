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

impl DividendData {
    pub fn add_dividend(
        &mut self,
        dividend: trading212::models::history_dividend_item::HistoryDividendItem,
    ) {
        self.dividends.insert(dividend.reference, dividend);
    }

    pub fn set_dividends_cursor(&mut self, cursor: Option<i64>) {
        self.dividends_cursor = cursor;
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn sum_dividends(&self) -> f32 {
        self.dividends.values().map(|d| d.amount).sum()
    }

    pub fn sum_dividends_in_euro(&self) -> f32 {
        self.dividends
            .values()
            .filter_map(|d| d.amount_in_euro)
            .sum()
    }

    /// Get sum of dividends grouped by ticker
    pub fn sum_dividends_by_ticker(&self) -> std::collections::HashMap<String, f32> {
        let mut sum = std::collections::HashMap::new();
        for dividend in self.dividends.values() {
            let ticker = &dividend.ticker;
            let amount = dividend.amount;
            let entry = sum.entry(ticker.clone()).or_insert(0.0);
            *entry += amount;
        }
        sum
    }
}
