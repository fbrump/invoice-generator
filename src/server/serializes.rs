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
    pub created_at: DateTime<Utc>,
}

impl CreateTransactionPayload {
    pub fn to_model(&self, id: Uuid) -> Transaction {
        Transaction::new(
            id,
            self.description.clone(),
            self.merchant_name.clone(),
            self.value,
            self.currency,
            self.created_at,
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    page: i64,
    page_size: i64,
    total_items: Option<i64>,
    total_pages: Option<i64>,
}

impl PaginationInfo {
    pub fn new(
        page: i64,
        page_size: i64,
        total_items: Option<i64>,
        total_pages: Option<i64>,
    ) -> Self {
        Self {
            page,
            page_size,
            total_items,
            total_pages,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedTransactionResponse {
    transactions: Vec<Transaction>,
    pagination: PaginationInfo,
}

impl PaginatedTransactionResponse {
    pub fn new(transactions: Vec<Transaction>, pagination: PaginationInfo) -> Self {
        Self {
            transactions,
            pagination,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportTypeEnum {
    CSV = 1,
}

#[derive(Debug, Serialize)]
pub struct ReportDataResponse {
    report_type: ReportTypeEnum,
    report_id: Uuid,
    file_name: String,
    path: String,
}

impl ReportDataResponse {
    pub fn new(
        report_type: ReportTypeEnum,
        report_id: Uuid,
        file_name: String,
        path: String,
    ) -> Self {
        Self {
            report_type,
            report_id,
            file_name,
            path,
        }
    }
}
