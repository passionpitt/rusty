use warp::Filter;
use crate::database::get_db_pool;
use crate::routes::user_routes;
use crate::handlers::user_handlers::handle_rejection;

pub async fn run_server() {
    let db_pool = get_db_pool();
    let routes = user_routes(db_pool).recover(handle_rejection);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
