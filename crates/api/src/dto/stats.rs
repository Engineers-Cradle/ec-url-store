use deadpool_postgres::Client;
use actix_web::{get, web, Error, HttpResponse};

use crate::libs::http::AppState;
use crate::utils::errors::AppError;
use crate::repository;

#[get("/stats/urls")]
pub async fn fetch_url_stats(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let stats = repository::db::retrieve_url_stats(&client).await?;

    if stats.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let stat = &stats[0];

    Ok(HttpResponse::Ok().json(stat))
}

#[get("/stats/analytics")]
pub async fn fetch_analytic_stats(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let stats: Vec<repository::models::URLStoreAnalyticsInfo> = repository::db::retrieve_analytics_stats(&client).await?;

    if stats.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let stat: &repository::models::URLStoreAnalyticsInfo = &stats[0];

    Ok(HttpResponse::Ok().json(stat))
}

pub fn init_stats_routes(config: &mut web::ServiceConfig) {
    config.service(fetch_url_stats);
    config.service(fetch_analytic_stats);
}