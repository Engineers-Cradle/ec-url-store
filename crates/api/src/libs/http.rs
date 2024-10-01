use crate::dto::stats;
use crate::dto::url;
use crate::libs::redis::get_value;

use listenfd::ListenFd;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use tokio_postgres::NoTls;
use tracing_actix_web::TracingLogger;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub pool: deadpool_postgres::Pool,
}

#[actix_web::main]
pub async fn start_web_server(
) -> std::io::Result<()> {
    let env_config: crate::utils::config::AppConfig = crate::env_config();

    let redis_client: redis::Client = crate::libs::redis::connection_to_redis(
        &env_config.redis_url
    ).await;

    let pool = env_config.pg.create_pool(None, NoTls).unwrap();

    let app_state: AppState = AppState {
        redis_client: redis_client.clone(),
        pool: pool.clone(),
    };

    let mut redis_multiplex_connection: redis::aio::MultiplexedConnection = redis_client.get_multiplexed_async_connection().await.unwrap();
    let url_counter: String = get_value(&mut redis_multiplex_connection, "url_store_counter").await;

    match url_counter.parse::<i32>() {
        Ok(_) => (),
        Err(_) => {
            let _ = crate::libs::redis::set_value(&mut redis_multiplex_connection, "url_store_counter", "0").await;
        }
    }

    env_logger::init_from_env(Env::default().default_filter_or(
        env_config.log_level
    ));

    let mut listenfd: ListenFd = ListenFd::from_env();

    let mut server = HttpServer::new(
        move || App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(TracingLogger::default())
            .configure(stats::init_stats_routes)
            .configure(url::init_url_routes)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host: &str = "0.0.0.0";
            let port: u16 = env_config.web_server_port;
            
            println!("Web Server started at http://{}:{}", host, port);

            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.workers(
        env_config.num_workers
    ).run().await
}