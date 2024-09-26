use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::utils::errors::AppError;

use crate::repository::models::{URLStore, URLStoreAnalytics, URLStoreAnalyticsInfo, URLStoreInfo};

pub async fn get_url_info(client: &Client, p_name: String, p_value: String) -> Result<Vec<URLStore>, AppError> {
    let stmt = include_str!("./sql/select_url.sql");
    let stmt = stmt.replace("$table_fields", &URLStore::sql_table_fields());
    let stmt = stmt.replace("$p_name", &p_name);
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, 
            &[
                &p_value
            ]
        )
        .await?
        .iter()
        .map(|row| URLStore::from_row_ref(row).unwrap())
        .collect::<Vec<URLStore>>();

    Ok(results)
}

pub async fn add_url_to_store(client: &Client, url_info: URLStore) -> Result<URLStore, AppError> {
    let _stmt = include_str!("./sql/insert_url.sql");
    let _stmt = _stmt.replace("$table_fields", &URLStore::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &url_info.id,
                &url_info.user_id,
                &url_info.url,
                &url_info.slug,
                &url_info.created_at,
                &url_info.updated_at,
            ],
        )
        .await?
        .iter()
        .map(|row| URLStore::from_row_ref(row).unwrap())
        .collect::<Vec<URLStore>>()
        .pop()
        .ok_or(AppError::NotFound)
}

pub async fn add_to_url_analytics(client: &Client, url_analytics_info: URLStoreAnalytics) -> Result<URLStoreAnalytics, AppError> {
    let _stmt = include_str!("./sql/insert_analytics.sql");
    let _stmt = _stmt.replace("$table_fields", &URLStoreAnalytics::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &url_analytics_info.id,
                &url_analytics_info.url_id,
                &url_analytics_info.ip_address,
                &url_analytics_info.country,
                &url_analytics_info.user_agent,
                &url_analytics_info.referer,
                &url_analytics_info.created_at,
            ],
        )
        .await?
        .iter()
        .map(|row| URLStoreAnalytics::from_row_ref(row).unwrap())
        .collect::<Vec<URLStoreAnalytics>>()
        .pop()
        .ok_or(AppError::NotFound) // more applicable for SELECTs
}

pub async fn get_url_analytics_info(client: &Client, url_id: i64) -> Result<Vec<URLStore>, AppError> {
    let stmt = include_str!("./sql/stats_analytics.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, 
            &[
                &url_id
            ]
        )
        .await?
        .iter()
        .map(|row| URLStore::from_row_ref(row).unwrap())
        .collect::<Vec<URLStore>>();

    Ok(results)
}

pub async fn retrieve_analytics_stats(client: &Client) -> Result<Vec<URLStoreAnalyticsInfo>, AppError> {
    let stmt_analytics = include_str!("./sql/stats_analytics_all.sql");
    let stmt_analytics = client.prepare(&stmt_analytics).await.unwrap();

    let results_analytics: Vec<URLStoreAnalyticsInfo> = client
        .query(&stmt_analytics, 
            &[]
        )
        .await?
        .iter()
        .map(|row| URLStoreAnalyticsInfo::from_row_ref(row).unwrap())
        .collect::<Vec<URLStoreAnalyticsInfo>>();

    Ok(results_analytics)
}

pub async fn retrieve_url_stats(client: &Client) -> Result<Vec<URLStoreInfo>, AppError> {
    let stmt_url = include_str!("./sql/stats_url_all.sql");
    let stmt_url = client.prepare(&stmt_url).await.unwrap();

    let results_url: Vec<URLStoreInfo> = client
        .query(&stmt_url, 
            &[]
        )
        .await?
        .iter()
        .map(|row| URLStoreInfo::from_row_ref(row).unwrap())
        .collect::<Vec<URLStoreInfo>>();

    Ok(results_url)
}

pub async fn check_slug_exists(client: &Client, slug: String) -> Result<bool, AppError> {
    let stmt = include_str!("./sql/select_url.sql");
    let stmt = stmt.replace("$table_fields", &URLStore::sql_table_fields());
    let p_name = "slug".to_string();
    let stmt = stmt.replace("$p_name", &p_name);
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, 
            &[
                &slug
            ]
        )
        .await?
        .iter()
        .map(|row| URLStore::from_row_ref(row).unwrap())
        .collect::<Vec<URLStore>>();

    Ok(results.len() > 0)
}