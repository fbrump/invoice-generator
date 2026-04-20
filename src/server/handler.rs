use std::fs::write;

use crate::server::errors::AppError;
use crate::server::models::transaction::Transaction;
use crate::server::serializes::{
    CreateTransactionPayload, PaginatedTransactionResponse, PaginationInfo, ReportDataResponse,
    ReportTypeEnum,
};
use axum::extract::{Json, Path, Query};
use axum::Extension;
use csv::Writer;
use sqlx::{PgPool, Postgres};
use sqlx_paginated::{paginated_query_as, FlatQueryParams, QueryParamsBuilder, QuerySortDirection};
use uuid::Uuid;

pub async fn get_transctions(
    Extension(pool): Extension<PgPool>,
    params: Query<FlatQueryParams>,
) -> Result<Json<PaginatedTransactionResponse>, AppError> {
    const BASE_TRANSACTION_QUERY: &str = r#"
        SELECT * 
        FROM transactions 
    "#;
    let paginator = params.pagination.clone().unwrap();
    let parameters = QueryParamsBuilder::<Transaction>::new()
        .with_pagination(paginator.page, paginator.page_size)
        .with_sort("created_at", QuerySortDirection::Descending)
        .build();

    let paginated_items = paginated_query_as::<Transaction, Postgres>(BASE_TRANSACTION_QUERY)
        .with_params(parameters.clone())
        .fetch_paginated(&pool)
        .await;

    match paginated_items {
        Ok(paginated_result) => Ok(Json(PaginatedTransactionResponse::new(
            paginated_result.records,
            PaginationInfo::new(
                parameters.pagination.page,
                parameters.pagination.page_size,
                paginated_result.total,
                paginated_result.total_pages,
            ),
        ))),
        Err(_) => Err(AppError::BadRequest),
    }
}

pub async fn get_transctions_by(
    Extension(pool): Extension<PgPool>,
    id: Path<Uuid>,
) -> Result<Json<Transaction>, AppError> {
    const BASE_QUERY_STR: &str = r#"
        SELECT * 
        FROM transactions 
        WHERE id= $1
    "#;
    let result_item = sqlx::query_as::<_, Transaction>(BASE_QUERY_STR)
        .bind(&id.0)
        .fetch_one(&pool)
        .await;
    match result_item {
        Ok(item) => Ok(Json(item)),
        Err(_) => Err(AppError::BadRequest),
    }
}

pub async fn insert_transction(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTransactionPayload>,
) -> Result<Json<Uuid>, AppError> {
    let id = Uuid::new_v4();
    let transaction = payload.to_model(id);
    const BASE_QUERY_STR: &str = r#"
        INSERT INTO transactions 
        (id, description, merchant_name, value, currency, created_at)
        VALUES
        ($1,$2,$3,$4,$5,$6) 
        RETURNING id;
    "#;
    println!("{}", BASE_QUERY_STR);
    println!("{:?}", transaction);
    let result_item = sqlx::query_scalar::<_, Uuid>(BASE_QUERY_STR)
        .bind(&transaction.id)
        .bind(&transaction.description)
        .bind(&transaction.merchant_name)
        .bind(&transaction.value)
        .bind(&transaction.currency)
        .bind(&transaction.created_at)
        .fetch_one(&pool)
        .await;

    match result_item {
        Ok(item_id) => Ok(Json(item_id)),
        Err(_) => Err(AppError::BadRequest),
    }
}

pub async fn get_transactions_report(
    Extension(pool): Extension<PgPool>,
    report_type: Path<ReportTypeEnum>,
) -> Result<Json<ReportDataResponse>, AppError> {
    // Retrieve data to memory
    const BASE_QUERY_STR: &str = r#"
        SELECT * 
        FROM transactions 
    "#;
    let result_items = sqlx::query_as::<_, Transaction>(BASE_QUERY_STR)
        .fetch_all(&pool)
        .await
        .unwrap();
    let report_id = Uuid::new_v4();
    let file_name = format!("transactions-{report_id}");
    let path: &str = "/tmp/";
    let report_results = ReportDataResponse::new(
        report_type.0,
        report_id,
        file_name.to_string(),
        path.to_string(),
    );

    let create_file_result = match report_type.0 {
        ReportTypeEnum::CSV => create_csv(result_items, &file_name, path),
    };

    match create_file_result {
        Ok(_) => Ok(Json(report_results)),
        Err(_) => todo!(),
    }
    
}

fn create_csv(items: Vec<Transaction>, file_name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv_writer = Writer::from_path(format!("{}{}.csv", path, file_name));
    println!("{}", items.len());
    match csv_writer {
        Ok(mut file) => {
            // header
            file.write_record(&["id", "merchant", "value"]).unwrap();
            // rows
            for item in items {
                file.write_record(&[
                    item.id.to_string(),
                    item.merchant_name,
                    item.value.to_string(),
                ])
                .unwrap();
            }
            let _ = file.flush();
        }
        Err(_) => todo!(),
    };
    Ok(())
}
