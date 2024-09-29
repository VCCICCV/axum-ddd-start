use infrastructure::database::redis_connection::establish_redis_connection;
use redis::AsyncCommands;
#[tokio::test]
async fn test_establish_db_connection() {
    // 加载.env 文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();
    let pool = establish_redis_connection().await;

    match pool {
        Ok(pool) => {
            let mut conn = pool.get().await.unwrap();
            conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
            let result: String = conn.get("foo").await.unwrap();
            assert_eq!(result, "bar");
        }
        Err(e) => panic!("Failed to establish Redis connection: {}", e),
    }
}
