pub mod transactions {
    use sqlx::{Error, Pool, Postgres};
    use sqlx_paginated::{
        paginated_query_as, PaginatedResponse, QueryParamsBuilder, QuerySortDirection,
    };
    use uuid::Uuid;

    use crate::server::models::transaction::Transaction;

    mod queries {
        pub const TRANSACTION_GET_ALL: &str = r#"
            SELECT *
            FROM transactions
        "#;

        pub const TRANSACTION_GET_BY_ID: &str = r#"
            SELECT *
            FROM transactions
            WHERE id= $1
        "#;
        pub const TRANSACTION_INSERT: &str = r#"
                INSERT INTO transactions
                (id, description, merchant_name, value, currency, created_at)
                VALUES
                ($1,$2,$3,$4,$5,$6)
                RETURNING id;
            "#;
    }

    pub async fn get_transactions_paginated(
        pool: Pool<Postgres>,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResponse<Transaction>, Error> {
        let parameters = QueryParamsBuilder::<Transaction>::new()
            .with_pagination(page, page_size)
            .with_sort("created_at", QuerySortDirection::Descending)
            .build();

        paginated_query_as::<Transaction, Postgres>(queries::TRANSACTION_GET_ALL)
            .with_params(parameters.clone())
            .fetch_paginated(&pool)
            .await
    }

    pub async fn get_transactions(pool: Pool<Postgres>) -> Result<Vec<Transaction>, Error> {
        let items = sqlx::query_as::<_, Transaction>(queries::TRANSACTION_GET_ALL)
            .fetch_all(&pool)
            .await;
        items
    }
    pub async fn get_transaction(pool: Pool<Postgres>, id: Uuid) -> Result<Transaction, Error> {
        let item = sqlx::query_as::<_, Transaction>(queries::TRANSACTION_GET_BY_ID)
            .bind(&id)
            .fetch_one(&pool)
            .await;
        item
    }

    pub async fn insert_transaction(
        pool: Pool<Postgres>,
        transaction: Transaction,
    ) -> Result<Uuid, Error> {
        let result_item = sqlx::query_scalar::<_, Uuid>(queries::TRANSACTION_INSERT)
            .bind(&transaction.id)
            .bind(&transaction.description)
            .bind(&transaction.merchant_name)
            .bind(&transaction.value)
            .bind(&transaction.currency)
            .bind(&transaction.created_at)
            .fetch_one(&pool)
            .await;
        result_item
    }
}
