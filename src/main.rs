mod queries;
mod request;
mod response;
use actix_http::error::Error;
use actix_session::{CookieSession, Session};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use log::info;
use queries::{check_credentials, create_post, create_tables, create_user, get_feed, user_exists};
use request::{CreatePostRequest, LoginRequest};
use response::{CreatePostResponse, CreateUserResponse, GetPostResponse, LoginResponse};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    create_tables();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false),
            )
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
            .service(web::resource("/create").route(web::post().to(create)))
            .service(web::resource("/create-post").route(web::post().to(create_posts)))
            .service(web::resource("/get-post").route(web::post().to(get_post))),
    );
}

async fn create(req: HttpRequest, params: web::Form<LoginRequest>) -> Result<HttpResponse> {
    if user_exists(params.get_username()) {
        return Ok(HttpResponse::Ok().json(CreateUserResponse::new(
            "Username already exists".to_string(),
        )));
    }
    create_user(
        params.get_username(),
        &(hash(params.get_password(), DEFAULT_COST).unwrap()),
    );
    Ok(HttpResponse::Ok().json(CreateUserResponse::new("User created".to_string())))
}

async fn create_posts(
    session: Session,
    req: HttpRequest,
    params: web::Form<CreatePostRequest>,
) -> Result<HttpResponse> {
    println!("{:?}", session.get::<bool>("logged_in"));
    // if let Some(logged_in) = session.get::<bool>("logged_in")? {
    if true {
        create_post(
            // &session.get::<String>("username")?.unwrap(),
            "satvik",
            params.get_body(),
        );
        return Ok(HttpResponse::Ok().json(CreatePostResponse::new("Post created".to_string())));
    }
    Ok(HttpResponse::Ok().json(CreatePostResponse::new("Need to log in".to_string())))
}

async fn get_post(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    // if let Some(logged_in) = session.get::<bool>("logged_in")? {
    println!("{:?}", "trying get_post");
    if true {
        return Ok(HttpResponse::Ok().json(GetPostResponse::new(get_feed(
            // &session.get::<String>("username")?.unwrap(),
            "satvik"
        ))));
    }
    println!("{:?}", "We died before finishing");
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello")))
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
            session.set("username", params.get_username());
            return Ok(HttpResponse::Ok().json(LoginResponse::new("Login successful".to_string())));
        }
    }
    Ok(HttpResponse::Ok().json(LoginResponse::new("Login unsuccessful".to_string())))
}
