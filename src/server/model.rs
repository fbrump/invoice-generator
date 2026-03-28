use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize)]
pub enum CurrencyEnum { 
    EURO,
    DOLLAR,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id : Uuid,
    pub description: String,
    pub merchant_name: String,
    pub value: f32,
    pub currency: CurrencyEnum,
    pub date_time: DateTime<Utc>, 
}
