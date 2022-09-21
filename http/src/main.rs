mod endpoints;
mod error;
mod schema;

use error::Error;
type Result<T> = core::result::Result<T, Error>;

use actix_cors::Cors;
use actix_web::{http::header, web::Data, App, HttpServer};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "TiDB example server running options")]
struct Opts {
    #[structopt(
        short = "d",
        long,
        default_value = "mysql://root:@127.0.0.1:4000/test?charset=utf8mb4",
        help = "TiDB database url"
    )]
    database_url: String,
    #[structopt(
        short = "a",
        long,
        default_value = "0.0.0.0:8080",
        help = "listen address"
    )]
    listen_address: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let options: Opts = Opts::from_args();
    let connection_pool = store::new_pool(&options.database_url, 4)
        .expect("TiDB connection pool must be build successfully...");
    println!("server start listening with {}", &options.listen_address);
    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);
        App::new()
            .app_data(Data::new(connection_pool.clone()))
            .wrap(cors)
            .configure(endpoints::routes)
    })
    .bind(&options.listen_address)?
    .run()
    .await
}
