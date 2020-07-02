use std::env;
use std::io;
use std::num::ParseIntError;

use actix_web::{server, App, Error, HttpRequest, HttpResponse};
use sentry_actix::SentryMiddleware;
use sentry::integrations::failure::capture_error;

use actix_web::middleware::cors::Cors;
use actix_web::http::header;
use actix_web::http;


fn failing(_req: &HttpRequest) -> Result<String, Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Something went really wrong here").into())
}

fn multiply_new(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number: i32 = first_number_str.parse()?;
    let second_number: i32 = second_number_str.parse()?;
    Ok(first_number * second_number)
}

fn handled(_req: &HttpRequest) -> Result<String, Error> {
    let first = "t";
    let second = "2";
    let result = match multiply_new(first, second) {
        Ok(result) => result,
        Err(err) => {
            // Foo is the ParseIntError turned into a failure::Error.
            let foo = err.into();
            capture_error(&foo);
            return Ok("try again".to_string());
        }
    };
    Ok(format!("{} * {} => {}", first, second, result))
}

fn checkout(_req: &HttpRequest) -> Result<String, Error> {

    // TODO
    // 1 request paylod from _req

    // Err(io::Error::new(io::ErrorKind::Other, "An error happens here").into())
    Ok(format!("{}", "checked out"))
}

fn main() {
    let _guard = sentry::init("https://92f3ece3dfb04b978f5de29777e456ee@o87286.ingest.sentry.io/5259785");
    env::set_var("RUST_BACKTRACE", "1");

    // NEW 07/01/20
    let app = server::new(|| { App::new()
        .configure(|app| Cors::for_app(app)
            .allowed_origin("http://localhost:5000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
            .resource("/index.html", |r| {
                r.method(http::Method::GET).f(|_| HttpResponse::Ok());
                r.method(http::Method::HEAD).f(|_| HttpResponse::MethodNotAllowed());
            })
            .resource("/handled", |r| r.f(handled))
            .resource("/checkout", |r| r.f(checkout))
            .register())
    })
    .bind("127.0.0.1:3001")
    .unwrap()
    .run();

    // OLD 06/30/20
    // let _cors = cors::Cors::build()
    // .allowed_origin("127.0.0.1")
    // .allowed_methods(vec!["GET", "POST"])
    // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    // .allowed_header(header::CONTENT_TYPE)
    // .max_age(3600)
    // .finish();

    // OLD 06/30/20
    // server::new(|| {
    //     App::new()
    //         .middleware(SentryMiddleware::new())
    //         .resource("/", |r| r.f(failing))
            // .resource("/handled", |r| r.f(handled))
            // .resource("/checkout", |r| r.f(checkout))
    // })
    // .bind("127.0.0.1:3001")
    // .unwrap()
    // .run();
}

// .middleware(SentryMiddleware::builder().emit_header(true).finish())
