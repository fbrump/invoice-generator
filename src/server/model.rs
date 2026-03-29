use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CurrencyEnum {
    EURO,
    DOLLAR,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub description: String,
    pub merchant_name: String,
    pub value: f32,
    pub currency: CurrencyEnum,
    pub date_time: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        id: Uuid,
        description: String,
        merchant_name: String,
        value: f32,
        currency: CurrencyEnum,
        date_time: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            description,
            merchant_name,
            value,
            currency,
            date_time,
        }
    }
}

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
