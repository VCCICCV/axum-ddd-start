// use sea_orm::{DatabaseConnection, DbErr};
// use std::env;



// #[tokio::test]
// async fn test_establish_connection() {
//     // 获取连接
//     let result = infrastructure::database::connection::establish_connection().await;

//     // 检查连接是否成功
//     assert!(result.is_ok(), "Failed to establish connection");

//     let connection = result.unwrap();

//     // 可以进行一些简单的查询来进一步验证连接是否可用
//     let num_users_result = connection
//        .execute(|conn| async move {
//             let num_users = UserEntity::find().count(conn).await.unwrap();
//             num_users
//         })
//        .await;

//     assert!(num_users_result.is_ok(), "Failed to execute query");
//     let num_users = num_users_result.unwrap();
//     assert!(num_users >= 0, "Number of users should be non-negative");
// }