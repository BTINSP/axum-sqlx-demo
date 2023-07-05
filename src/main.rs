

use axum_sqlx_demo::server_run;



#[tokio::main]
async fn main() {
    server_run().await;
}
