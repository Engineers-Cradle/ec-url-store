use redis::AsyncCommands;

pub async fn connection_to_redis(
    redis_url: &str,
) -> redis::Client {
    let client: redis::Client = redis::Client::open(redis_url).unwrap();
    client
}

pub async fn get_value(connection: &mut redis::aio::MultiplexedConnection, key: &str) -> String {
    let value = connection.get(key).await;
   
    match value {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }
}

pub async fn set_value(connection: &mut redis::aio::MultiplexedConnection, key: &str, value: &str) -> String {
    let value = connection.set(key, value).await;
   
    match value {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }
}