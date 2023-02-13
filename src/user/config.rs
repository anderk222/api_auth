use actix_web::web;
use super::controller;

pub fn config(cfg : &mut web::ServiceConfig){
    
cfg.service(
    web::scope("/user")
    .route("/",web::get().to(controller::get_users))
    .route("/",web::post().to(controller::create_user))
    .route("/{user_id}", web::get().to(controller::find_user))
    .route("/{user+_id}", web::delete().to(controller::delete_user))
    .route("/{user_id}", web::put().to(controller::update_user))
);
}