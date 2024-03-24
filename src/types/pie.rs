use time::OffsetDateTime;

#[derive(serde::Serialize, Clone, Debug, Default, PartialEq)]
pub struct PiesData {
    pub pies: std::collections::HashMap<i64, Pie>,
}

impl PiesData {
    pub fn add_pie(
        &mut self,
        pie: trading212::models::account_bucket_result_response::AccountBucketResultResponse,
    ) {
        match self.pies.get_mut(&pie.id) {
            None => {
                self.pies.insert(
                    pie.id,
                    Pie {
                        data: pie,
                        details: None,
                        updated: OffsetDateTime::now_utc(),
                    },
                );
            }
            Some(p) => {
                (*p).data = pie;
            }
        };
    }

    pub fn add_detail(
        &mut self,
        id: i64,
        details: trading212::models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse,
    ) {
        if let Some(p) = self.pies.get_mut(&id) {
            (*p).details = Some(details);
            (*p).updated = OffsetDateTime::now_utc();
        };
    }

    pub fn get_incomplete_ids(&self) -> Vec<i64> {
        self.pies
            .values()
            .filter(|p| p.details.is_none())
            .map(|p| p.data.id)
            .collect()
    }

    pub fn get_oldest_updated_id(&self) -> Option<i64> {
        self.pies
            .values()
            .filter(|p| p.details.is_some())
            .min_by_key(|p| p.updated)
            .map(|p| p.data.id)
    }

    pub fn get_complete_pies(&self) -> Vec<&Pie> {
        self.pies.values().filter(|p| p.details.is_some()).collect()
    }
}

#[derive(serde::Serialize, Clone, Debug, PartialEq)]
pub struct Pie {
    pub data: trading212::models::account_bucket_result_response::AccountBucketResultResponse,
    pub details: Option<trading212::models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse>,
    updated: OffsetDateTime
}
