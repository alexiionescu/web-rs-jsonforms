// use actix_files as fs;
use actix_cors::Cors;
use actix_web::{
    http::header::{self}, web, App, HttpServer,
};
use app_common::app_state;
use flexi_logger::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::try_with_str("info, my::critical::module=trace")
        .unwrap()
        .start()
        .unwrap();
    let app_state = web::Data::new(app_state::Data::new());
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(app_state.clone())
            .service(app_common::rest_api)
            .service(user_app::rest_api)
        // .service(fs::Files::new("/", app_state.fs_root).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
