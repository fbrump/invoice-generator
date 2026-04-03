use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, sqlx::Type, Clone, Copy, Serialize, Deserialize)]
#[sqlx(type_name = "currency")]
#[sqlx(rename_all = "lowercase")]
pub enum CurrencyEnum {
    UNDEFINED,
    EURO,
    DOLLAR,
}

impl From<String> for CurrencyEnum {
    fn from(value: String) -> Self {
        match value.as_str() {
            "euro" => CurrencyEnum::EURO,
            "dollar" => CurrencyEnum::DOLLAR,
            _ => CurrencyEnum::UNDEFINED,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub description: String,
    pub merchant_name: String,
    pub value: f32,
    pub currency: CurrencyEnum,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        id: Uuid,
        description: String,
        merchant_name: String,
        value: f32,
        currency: CurrencyEnum,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            description,
            merchant_name,
            value,
            currency,
            created_at,
        }
    }
}
