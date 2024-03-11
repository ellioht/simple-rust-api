use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::users::handler;

pub fn routes() -> BoxedFilter<(impl Reply+Sized,)> {
    let get_users_route = warp::get().and(warp::path!("users"))
        .and_then(handler::get_users_handler);

    let create_user_route = warp::post().and(warp::path!("users"))
        .and(warp::body::json())
        .and_then(handler::create_user_handler);

    let routes = get_users_route.or(create_user_route);
    routes.boxed()
}