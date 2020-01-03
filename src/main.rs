mod queries;
mod request;
mod response;
use actix_session::Session;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use queries::{check_credentials, create_tables, create_user, user_exists};
use request::LoginRequest;
use response::{CreateResponse, LoginResponse};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    
    create_tables();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/create").route(web::post().to(create))),
    );
}

async fn create(req: HttpRequest, params: web::Form<LoginRequest>) -> Result<HttpResponse> {
    if user_exists(params.get_username()) {
        return Ok(
            HttpResponse::Ok().json(CreateResponse::new("Username already exists".to_string()))
        );
    }
    create_user(
        params.get_username(),
        &(hash(params.get_password(), DEFAULT_COST).unwrap()),
    );
    Ok(HttpResponse::Ok().json(CreateResponse::new("User created".to_string())))
}

async fn login(
    session: Session,
    req: HttpRequest,
    params: web::Form<LoginRequest>,
) -> Result<HttpResponse> {
    if user_exists(params.get_username()) {
        if verify(
            params.get_password(),
            &check_credentials(params.get_username()),
        )
        .unwrap()
        {
            session.set("logged_in", true)?;
            return Ok(HttpResponse::Ok().json(LoginResponse::new("Login successful".to_string())));
        }
    }
    Ok(HttpResponse::Ok().json(LoginResponse::new("Login unsuccessful".to_string())))
}
