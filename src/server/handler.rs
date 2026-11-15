use crate::server::database::transactions::{
    get_transaction, get_transactions, get_transactions_paginated, insert_transaction,
};
use crate::server::errors::AppError;
use crate::server::features::common::{GenerateReportFactory, ReportFactory};
use crate::server::models::transaction::Transaction;
use crate::server::serializes::{
    CreateTransactionPayload, PaginatedTransactionResponse, PaginationInfo, ReportDataResponse,
    ReportTypeEnum,
};
use axum::extract::{Json, Path, Query};
use axum::Extension;
use sqlx::PgPool;
use sqlx_paginated::FlatQueryParams;
use uuid::Uuid;

#[utoipa::path(
    method(get, head),
    path = "/transactions",
    responses(
        (status = OK, body=PaginatedTransactionResponse, description = "Retrieve all transactions")
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number" ),
        ("page_size" = Option<i64>, Query, description = "Total items per page" ),
    ),
    tag = "transactions"
)]
pub async fn get_transctions(
    Extension(pool): Extension<PgPool>,
    params: Query<FlatQueryParams>,
) -> Result<Json<PaginatedTransactionResponse>, AppError> {
    let pagination = params.pagination.clone().unwrap();
    let paginated_items =
        get_transactions_paginated(pool, pagination.page, pagination.page_size).await;

    match paginated_items {
        Ok(paginated_result) => Ok(Json(PaginatedTransactionResponse::new(
            paginated_result.records,
            PaginationInfo::new(
                pagination.page,
                pagination.page_size,
                paginated_result.total,
                paginated_result.total_pages,
            ),
        ))),
        Err(_) => Err(AppError::BadRequest),
    }
}

#[utoipa::path(
    method(get,head),
    path = "/transactions/{id}",
    responses(
        (status = OK, body=Transaction, description = "Retrieve a transaction"),
        (status = BAD_REQUEST, description = "Transaction not found")
    ),
    tag = "transactions"
)]
pub async fn get_transctions_by(
    Extension(pool): Extension<PgPool>,
    id: Path<Uuid>,
) -> Result<Json<Transaction>, AppError> {
    let result_item = get_transaction(pool, id.0).await;
    match result_item {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(AppError::BadRequest),
    }
}

#[utoipa::path(
    post,
    path = "/transactions",
    responses(
        (status = OK, body=Uuid, description = "Insert a new transaction")
    ),
    tag = "transactions"
)]
pub async fn insert_transction(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTransactionPayload>,
) -> Result<Json<Uuid>, AppError> {
    let id = Uuid::new_v4();
    let result_item = insert_transaction(pool, payload.to_model(id)).await;
    match result_item {
        Ok(item_id) => Ok(Json(item_id)),
        Err(_) => Err(AppError::BadRequest),
    }
}

#[utoipa::path(
    method(get, head),
    path = "/reports/{report_type}",
    responses(
        (status = OK, body=ReportDataResponse, description = "Generate report by type")
    ),
    params(
        ("report_type" = ReportTypeEnum, Path, description = "Report Type Enum" )
    ),
    tag = "reports"
)]
pub async fn get_transactions_report(
    Extension(pool): Extension<PgPool>,
    report_type: Path<ReportTypeEnum>,
) -> Result<Json<ReportDataResponse>, AppError> {
    let result_items = get_transactions(pool).await.unwrap();
    let create_file_result = GenerateReportFactory::get_generator(report_type.0, result_items);

    match create_file_result {
        Ok(response) => Ok(Json(response)),
        Err(_) => todo!(),
    }
}
