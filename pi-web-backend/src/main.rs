#![allow(non_camel_case_types, unused_variables, unused_imports)]
extern crate pi_library;
// #[macro_use]
// extern crate strum_macros;

pub mod graphql;
pub mod utils;
pub mod xml;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use graphql::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};
use listenfd::ListenFd;

async fn index(schema: web::Data<BooksSchema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: web::Data<BooksSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:3300");

    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                // Cors::new() // <- Construct CORS middleware builder
                //     .allowed_origin("*")
                //     .allowed_methods(vec!["GET", "POST"])
                //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                //     .allowed_header(header::CONTENT_TYPE)
                //     .max_age(3600)
                //     .finish(),
                Cors::new().supports_credentials().finish(),
            )
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
        // .service(web::resource("/").guard(guard::Post()).to(index))
        // .service(
        //     web::resource("/")
        //         .guard(guard::Get())
        //         .guard(guard::Header("upgrade", "websocket"))
        //         .to(index_ws),
        // )
        // .service(web::resource("/").guard(guard::Get()).to(index_playground))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind("0.0.0.0:3300")
            .expect("Can not bind to port 3300")
    };
    server.run().await
}
