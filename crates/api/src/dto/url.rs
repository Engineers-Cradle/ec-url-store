use deadpool_postgres::Client;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use serde::Deserialize;
use chrono::prelude::*;
use hyperflake_rs::snowflake;
use seed_gen::generate_seed;

use crate::utils::grpc_client;
use crate::libs::http::AppState;
use crate::libs::redis::{get_value, set_value, set_value_with_cache};
use crate::utils::errors::AppError;
use crate::{libs, repository};

fn extract_header_value<'a>(req: &'a HttpRequest, key: &String) -> Option<&'a str> {
    req.headers().get(key)?.to_str().ok()
}

#[derive(Deserialize)]
pub struct URLSlug {
    uid: String,
}

#[get("/{uid}")]
pub async fn open_url_from_slug(req: HttpRequest, info: web::Path<URLSlug>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let p_name: String = "slug".to_string();
    let read_ip_country = grpc_client::read_ip_grpc(req.connection_info().peer_addr().unwrap_or("").to_string()).await;

    println!("{:?}", read_ip_country);

    // Check if slug exists in cache
    let url_cache: String = get_value(&mut data.redis_client.get_multiplexed_async_connection().await.unwrap(), &info.uid).await;

    if url_cache != "" {
        // url_cache = 3275873752344166400:https://google.com
        let url_parts: Vec<&str> = url_cache.split(":").collect();
        let url_id: i64 = url_parts[0].parse::<i64>().unwrap();
        let url_value: String = url_parts[1..].join(":");

        let mut snowflake: snowflake::SnowflakeId = snowflake::SnowflakeId::new();

        let url_analytics_info: repository::models::URLStoreAnalytics = repository::models::URLStoreAnalytics {
            id: snowflake.generate().parse::<i64>().unwrap(),
            url_id: url_id,
            user_agent: extract_header_value(&req, &"user-agent".to_string()).unwrap_or("").to_string(),
            referer: extract_header_value(&req, &"referrer".to_string()).unwrap_or("").to_string(),
            ip_address: req.connection_info().peer_addr().unwrap_or("").to_string(),
            country: read_ip_country.unwrap().country,
            created_at: Utc::now(),
        };

        let _ = repository::db::add_to_url_analytics(&client, url_analytics_info).await?;

        // 2 days
        let exp: u64 = 60 * 60 * 24 * 2;

        let _ = set_value_with_cache(
            &mut data.redis_client.get_multiplexed_async_connection().await.unwrap(),
            &p_name,
            &url_cache,
            exp
        ).await;

        return Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", url_value))
            .finish());
    }

    let urls: Vec<repository::models::URLStore> = repository::db::get_url_info(&client, p_name, info.uid.clone()).await?;

    if urls.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let url: &repository::models::URLStore = &urls[0];

    // Add to analytics
    let mut snowflake: snowflake::SnowflakeId = snowflake::SnowflakeId::new();

    let url_analytics_info: repository::models::URLStoreAnalytics = repository::models::URLStoreAnalytics {
        id: snowflake.generate().parse::<i64>().unwrap(),
        url_id: url.id,
        user_agent: extract_header_value(&req, &"user-agent".to_string()).unwrap_or("").to_string(),
        referer: extract_header_value(&req, &"referrer".to_string()).unwrap_or("").to_string(),
        ip_address: req.connection_info().peer_addr().unwrap_or("").to_string(),
        country: read_ip_country.unwrap().country,
        created_at: Utc::now(),
    };

    let _ = repository::db::add_to_url_analytics(&client, url_analytics_info).await?;

    Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", url.url.clone()))
        .finish())
}

#[get("/{uid}/info")]
pub async fn url_info_from_slug(req: HttpRequest, info: web::Path<URLSlug>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let access_token: Option<&str> = libs::jwt::get_auth_token(&req);

    if access_token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let is_verified = libs::jwt::verify_jwt(access_token.unwrap()).await;

    if !is_verified {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let p_name: String = "slug".to_string();
    let urls: Vec<repository::models::URLStore> = repository::db::get_url_info(&client, p_name, info.uid.clone()).await?;

    if urls.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let url: &repository::models::URLStore = &urls[0];

    Ok(HttpResponse::Ok().json(url))
}

#[get("/{uid}/analytics")]
pub async fn url_analytics_from_slug(req:HttpRequest, info: web::Path<URLSlug>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let access_token: Option<&str> = libs::jwt::get_auth_token(&req);

    if access_token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let is_verified = libs::jwt::verify_jwt(access_token.unwrap()).await;

    if !is_verified {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;
    let p_name: String = "slug".to_string();
    let urls: Vec<repository::models::URLStore> = repository::db::get_url_info(&client, p_name,info.uid.clone()).await?;

    if urls.len() == 0 {
        return Ok(HttpResponse::NotFound().finish());
    }

    let url: &repository::models::URLStore = &urls[0];

    let url_analytics: Vec<repository::models::URLStore> = repository::db::get_url_analytics_info(&client, url.id).await?;

    Ok(HttpResponse::Ok().json(url_analytics))
}


#[derive(Deserialize)]
pub struct URLData {
    url: String,
    user_id: Option<i64>,
}

pub async fn generate_slug(
    client: &Client,
    redis_multiplex_connection: &mut redis::aio::MultiplexedConnection,
) -> String {
    Box::pin(async move {
        let url_counter: String = get_value(redis_multiplex_connection, "url_store_counter").await;
        let url_counter: u32 = url_counter.parse::<u32>().unwrap() + 1;

        let _ = set_value(redis_multiplex_connection, "url_store_counter", &url_counter.to_string()).await;

        let slug: String = generate_seed(url_counter);

        let slug_exists: bool = repository::db::check_slug_exists(
            client,
            slug.clone()
        ).await.unwrap();

        if slug_exists {
            return generate_slug(client, redis_multiplex_connection).await;
        }

        slug
    }).await
}

#[post("/")]
pub async fn add_url_to_store(
    req: HttpRequest,
    url_data: web::Json<URLData>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let access_token: Option<&str> = libs::jwt::get_auth_token(&req);

    if access_token.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let is_verified = libs::jwt::verify_jwt(access_token.unwrap()).await;

    if !is_verified {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let raw_url_info: URLData = url_data.into_inner();
    let mut snowflake: snowflake::SnowflakeId = snowflake::SnowflakeId::new();

    let client: Client = data.pool.get().await.map_err(AppError::PoolError)?;

    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = data.redis_client.get_multiplexed_async_connection().await.unwrap();

    let slug: String = generate_slug(&client, &mut redis_multiplex_connection).await;

    let id: i64 = snowflake.generate().parse::<i64>().unwrap();

    let url_info: repository::models::URLStore = repository::models::URLStore {
        id: id.clone(),
        url: raw_url_info.url.clone(),
        slug: slug.clone(),
        user_id: Some(raw_url_info.user_id.unwrap_or(0)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let new_url: repository::models::URLStore = repository::db::add_url_to_store(&client, url_info).await?;

    // 2 days
    let exp: u64 = 60 * 60 * 24 * 2;
    let kv: String = format!("{}:{}", id, raw_url_info.url.clone());

    let _ = set_value_with_cache(
        &mut redis_multiplex_connection,
        &slug,
        &kv,
        exp
    ).await;

    Ok(HttpResponse::Ok().json(new_url))
}

pub fn init_url_routes(config: &mut web::ServiceConfig) {
    config.service(add_url_to_store);
    config.service(url_analytics_from_slug);
    config.service(url_info_from_slug);
    config.service(open_url_from_slug);
}