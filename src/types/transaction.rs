use time::format_description;
use uuid::Uuid;

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct TransactionData {
    pub transactions: std::collections::HashMap<
        Uuid,
        trading212::models::history_transaction_item::HistoryTransactionItem,
    >,
    pub cursor: Option<i64>,
    pub loaded: bool,
}

impl Default for TransactionData {
    fn default() -> Self {
        Self {
            transactions: Default::default(),
            cursor: Default::default(),
            loaded: true,
        }
    }
}

impl TransactionData {
    pub fn add(
        &mut self,
        transaction: trading212::models::history_transaction_item::HistoryTransactionItem,
    ) {
        self.transactions.insert(transaction.reference, transaction);
    }

    pub fn set_cursor(&mut self, cursor: Option<i64>) {
        self.cursor = cursor;
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn sum(&self) -> f32 {
        self.transactions.values().map(|d| d.amount).sum()
    }

    pub fn sum_by_type(
        &self,
    ) -> std::collections::HashMap<trading212::models::history_transaction_item::Type, f32> {
        let mut sum = std::collections::HashMap::new();
        for transaction in self.transactions.values() {
            let ticker = &transaction.r#type;
            let amount = transaction.amount;
            let entry = sum.entry(ticker.clone()).or_insert(0.0);
            *entry += amount;
        }
        sum
    }

    /// get sum of dividends not older than time::Duration
    pub fn sum_by_type_not_older_than(
        &self,
        duration: time::Duration,
    ) -> std::collections::HashMap<trading212::models::history_transaction_item::Type, f32> {
        let now = time::OffsetDateTime::now_utc();
        let mut sum = std::collections::HashMap::new();
        for transaction in self.transactions.values() {
            let ticker = &transaction.r#type;
            let amount = transaction.amount;
            let entry = sum.entry(ticker.clone()).or_insert(0.0);
            if now - transaction.date_time <= duration {
                *entry += amount;
            }
        }
        sum
    }

    pub fn sum_by_month(&self) -> std::collections::HashMap<String, f32> {
        let mut sum = std::collections::HashMap::new();
        for transaction in self.transactions.values() {
            let month = transaction
                .date_time
                .format(&format_description::parse("[year]-[month]").unwrap())
                .unwrap_or_default()
                .to_string();
            let amount = transaction.amount;
            let entry = sum.entry(month).or_insert(0.0);
            *entry += amount;
        }
        sum
    }
}
