pub mod routes {
    pub mod user_route;
}

pub mod controllers {
    pub mod user_controller;
}

pub mod middlewares {
    pub mod route_logger;
    pub mod common_middlewares;
}

pub mod app {
    pub mod responder;
}