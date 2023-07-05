
mod router;
mod common;
mod handle;
mod database;
mod model;
mod middleware;


use common::AppState;



pub async fn server_run() {

    //  创建数据库连接池
    let pool = database::database_pool_create().await;

    //  初始化app_state
    let app_state = AppState{ pool };

    //  创建router
    let router = router::create_router(app_state);

    //  启动服务
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(router.into_make_service())
    .await
    .expect("Running error");
}