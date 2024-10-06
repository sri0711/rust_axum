// service level routes
pub mod routes {
    pub mod user_route;
}

// service level controllers
pub mod controllers {
    pub mod user_controller;
}

// service middlewares
pub mod middlewares {
    pub mod auth_middleware;
    pub mod common_middlewares;
    pub mod route_logger;
}

// app level configurations
pub mod app {
    pub mod responder;
}
