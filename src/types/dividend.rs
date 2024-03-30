use time::format_description;
use uuid::Uuid;

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct DividendData {
    pub dividends: std::collections::HashMap<
        Uuid,
        trading212::models::history_dividend_item::HistoryDividendItem,
    >,
    pub cursor: Option<i64>,
    pub loaded: bool,
}

impl Default for DividendData {
    fn default() -> Self {
        Self {
            dividends: Default::default(),
            cursor: Default::default(),
            loaded: false,
        }
    }
}

impl DividendData {
    pub fn add(
        &mut self,
        dividend: trading212::models::history_dividend_item::HistoryDividendItem,
    ) {
        self.dividends.insert(dividend.reference, dividend);
    }

    pub fn set_cursor(&mut self, cursor: Option<i64>) {
        self.cursor = cursor;
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn sum(&self) -> f32 {
        self.dividends.values().map(|d| d.amount).sum()
    }

    #[allow(dead_code)]
    pub fn sum_in_euro(&self) -> f32 {
        self.dividends
            .values()
            .filter_map(|d| d.amount_in_euro)
            .sum()
    }

    /// Get sum of dividends grouped by ticker
    pub fn sum_by_ticker(&self) -> std::collections::HashMap<String, f32> {
        let mut sum = std::collections::HashMap::new();
        for dividend in self.dividends.values() {
            let ticker = &dividend.ticker;
            let amount = dividend.amount;
            let entry = sum.entry(ticker.clone()).or_insert(0.0);
            *entry += amount;
        }
        sum
    }

    /// get sum of dividends not older than time::Duration
    #[allow(dead_code)]
    pub fn sum_by_ticker_not_older_than(
        &self,
        duration: time::Duration,
    ) -> std::collections::HashMap<String, f32> {
        let now = time::OffsetDateTime::now_utc();
        let mut sum = std::collections::HashMap::new();
        for dividend in self.dividends.values() {
            let ticker = &dividend.ticker;
            let amount = dividend.amount;
            let entry = sum.entry(ticker.clone()).or_insert(0.0);
            if now - dividend.paid_on <= duration {
                *entry += amount;
            }
        }
        sum
    }

    pub fn sum_by_month(&self) -> std::collections::HashMap<String, f32> {
        let mut sum = std::collections::HashMap::new();
        for dividend in self.dividends.values() {
            let month = dividend
                .paid_on
                .format(&format_description::parse("[year]-[month]").unwrap())
                .unwrap_or_default()
                .to_string();
            let amount = dividend.amount;
            let entry = sum.entry(month).or_insert(0.0);
            *entry += amount;
        }
        sum
    }
}
