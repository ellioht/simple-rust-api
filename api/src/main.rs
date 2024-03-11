mod users;

#[tokio::main]
async fn main() {
    if dotenv::dotenv().is_err() {
        eprintln!(".env file not found");
    }

    let user_routes = users::routes::routes();
    let routes = user_routes;

    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;
}