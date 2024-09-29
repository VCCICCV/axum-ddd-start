use std::error::Error;
use std::env;

use bb8_redis::RedisConnectionManager;
use tracing::info;


pub async fn establish_redis_connection() -> Result<bb8::Pool<RedisConnectionManager>, Box<dyn Error>> {
    let redis_url = env::var("REDIS_URL").map_err(|e| format!("REDIS_URL is not set in.env file: {}", e))?;
    let manager = RedisConnectionManager::new(redis_url.as_str()).map_err(|e| format!("Failed to create Redis connection manager: {}", e))?;
    let pool = bb8::Pool::builder()
      .build(manager)
      .await
      .map_err(|e| format!("Failed to build Redis connection pool: {}", e))?;
    info!("Redis connected");
    Ok(pool)
}