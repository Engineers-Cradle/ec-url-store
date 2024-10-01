use deadpool_postgres::Client;
use actix_web::{get, web, Error, HttpResponse, HttpRequest};

use crate::libs::http::AppState;
use crate::utils::errors::AppError;
use crate::{libs, repository};

#[get("/stats/urls")]
pub async fn fetch_url_stats(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let access_token: Option<&str> = libs::jwt::get_auth_token(&req);

    if access_token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let is_verified = libs::jwt::verify_jwt(access_token.unwrap()).await;

    if !is_verified {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let stats = repository::db::retrieve_url_stats(&client).await?;

    if stats.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let stat = &stats[0];

    Ok(HttpResponse::Ok().json(stat))
}

#[get("/stats/analytics")]
pub async fn fetch_analytic_stats(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let access_token: Option<&str> = libs::jwt::get_auth_token(&req);

    if access_token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let is_verified = libs::jwt::verify_jwt(access_token.unwrap()).await;

    if !is_verified {
        return Ok(HttpResponse::Unauthorized().finish());
    }

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