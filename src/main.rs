use actix_web::{ HttpServer, web,App };

mod user;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new()
    .service(
        web::scope("/api/v1/auth").
        configure(user::config::config)
        )
    })
    
    .keep_alive(None)
    .shutdown_timeout(34)
    .bind(("127.0.0.2", 3000))?
    .run()
    .await
}
