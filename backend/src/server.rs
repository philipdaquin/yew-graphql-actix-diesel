use actix_web::HttpResponse;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, http::header};
use actix_cors::Cors;
use crate::db::{establish_connection, DatabaseKind};
// use crate::graphql_modules::context::{graphql, playground};
use crate::graphql_modules::{schema::build_schema,
    routes::{graphql, graphql_playground},
};

pub async fn new_server(port: u32) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    //  Database connection pool
    let db_pool = establish_connection(DatabaseKind::Example);
    //  Grapqhl schema builder
    let schema = build_schema().finish();


    HttpServer::new(move || {
        //  App Routes
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(schema.clone()))
            .wrap(Logger::default())
            //  Allowed Methods
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
                .max_age(3600),
            )
            //  GraphQl Services
            .service(graphql)
            .service(graphql_playground)
            

    })
    .workers(2)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}




