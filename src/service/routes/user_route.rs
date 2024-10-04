use axum::{routing::*, Router};

pub async fn user_routes() -> Router {
    let list_users = Router::new().route("/list", get(|| async { "users lists" }));
    let view_users = Router::new().route("/view", get(|| async { "view user" }));

    let merged_user_routes = list_users.merge(view_users);

    return Router::new().nest("/users", merged_user_routes);
}
