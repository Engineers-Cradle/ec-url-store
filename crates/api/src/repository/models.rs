use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "url_store_data")]
pub struct URLStore {
    pub id : i64,
    pub user_id: Option<i64>,
    pub url: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "url_store_analytics_data")]
pub struct URLStoreAnalytics {
    pub id : i64,
    pub url_id: i64,
    pub ip_address: String,
    pub country: String,
    pub user_agent: String,
    pub referer: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "url_store_analytics_data")]
pub struct URLStoreAnalyticsInfo {
    pub unique_visitors: i64,
    pub total_visits: i64,
    pub countries: i64,
    pub referrers: i64,
    pub user_agents: i64,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "url_store_data")]
pub struct URLStoreInfo {
    pub total_urls: i64,
    pub total_users: i64,
}