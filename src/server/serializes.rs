use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::models::transaction::{CurrencyEnum, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionPayload {
    pub description: String,
    pub merchant_name: String,
    pub value: f32,
    pub currency: CurrencyEnum,
    pub date_time: DateTime<Utc>,
}

impl CreateTransactionPayload {
    pub fn to_model(&self, id: Uuid) -> Transaction {
        Transaction::new(
            id,
            self.description.clone(),
            self.merchant_name.clone(),
            self.value,
            self.currency,
            self.date_time,
        )
    }
}
