use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;
use common::AppState;

mod common;
mod error;
mod routes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = "calclib web service")]
struct Args {
    /// IP address to use
    #[clap(long, short, default_value = "127.0.0.1")]
    ip: String,
    /// Server port to use
    #[clap(long, short, default_value = "4711")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let data = web::Data::new(AppState {
        calc: Mutex::new(calclib::Calc::new(0.0)),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(data.clone())
            .service(routes::calc::post_add)
            .service(routes::calc::post_sub)
            .service(routes::calc::get_result)
    })
    .bind((args.ip, args.port))?
    .run()
    .await
}
