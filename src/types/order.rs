#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct OrderData {
    pub orders:
        std::collections::HashMap<i64, trading212::models::historical_order::HistoricalOrder>,
    pub cursor: Option<i64>,
    pub loaded: bool,
}

impl Default for OrderData {
    fn default() -> Self {
        Self {
            orders: Default::default(),
            cursor: Default::default(),
            loaded: false,
        }
    }
}

impl OrderData {
    pub fn add(&mut self, order: trading212::models::historical_order::HistoricalOrder) {
        self.orders.insert(order.id, order);
    }

    pub fn set_cursor(&mut self, cursor: Option<i64>) {
        self.cursor = cursor;
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }
}
