
/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
//import the parse result function from the lib.rs file
use hugoweather::parse_result;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Hugo Weather Site!")
}

//create a function that returns a random fruit
#[get("/weather/{a}")]
async fn weather(pos: web::Path<String>) -> impl Responder {
    // convert position to string
    let pos = pos.as_str();
    // return the result of the parse_result function
    let result : String = parse_result(pos).await;
    println!("{}", result);
    HttpResponse::Ok().body(result)
}

//create a function that returns a 200 status code
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(weather)
            .service(health)
            .service(version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

