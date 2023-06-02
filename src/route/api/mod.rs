use actix_web::{web, Scope};

// pub mod admin;
// pub mod device;
// pub mod user;
pub fn make_scope() -> Scope {
    return web::scope("/api");
    // .service(user::make_scope())
    // .service(device::make_scope())
    // .service(admin::make_scope());
}
