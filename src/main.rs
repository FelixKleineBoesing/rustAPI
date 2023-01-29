mod data;

use std::{env, io};
use env_logger::init;


#[actix_rt::main]
async fn main() -> io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug");
    init();

    let data = data::read_articles("..data/articles.csv")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(|| async { "Hello world!" }))
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
