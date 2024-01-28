use std::sync::Arc;

pub async fn get_router() -> Router {
    let pool = get_db_pool().await;
    let app_state = Arc::new(AppState { db: pool.clone() });

    let routes_api =
        api::routes::get_routes(app_state.clone()).route_layer(from_fn(auth_required::<Body>));

    Router::new()
        .merge(api::routes::get_login(app_state.clone()))
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(get_chat_layer())
}

async fn get_db_pool() -> Pool<MySql> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            info!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

async fn main_response_mapper(res: Response) -> Response {
    info!("->> main_response_mapper");

    res
}
