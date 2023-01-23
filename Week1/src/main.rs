// Find city by altitude
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Data {
    index: i32,
    country: String,
    city: String,
    latitude: f32,
    longitude: f32,
    altitude: f32,
}

// Welcome
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is #Find-City-By-Altitude Web, Please input the altitude!\n")
}

// Parse given altitude and return result
#[get("/alt/{a}")]
async fn run(info: web::Path<String>) -> impl Responder {
    let info = info.into_inner();
    let al = info.parse::<f32>().unwrap();

    // Obtain data and compare with given altitude
    let rdr = csv::Reader::from_path("WorldCityLocations/World_Cities_Location_table.csv");
    let mut diff = f32::MAX;
    let mut res = Data {
        index: 0,
        country: "".to_string(),
        city: "".to_string(),
        latitude: 0.0,
        longitude: 0.0,
        altitude: 0.0,
    };

    let mut result = String::new();

    for record in rdr.expect("").deserialize::<Data>() {
        match record {
            Err(why) => {}
            Ok(record) => {
                let data: Data = record;
                if f32::abs(data.altitude - al) < diff {
                    res = data.clone();
                    diff = f32::abs(data.altitude - al);
                }
                result = format!("The city closest to the given altitude is {} (lat: {}, long: {}), in {}, with altitude of {}m.\n", res.city, res.latitude, res.longitude, res.country, res.altitude).to_string();
            }
        }
    }

    HttpResponse::Ok().body(result)
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(run))
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
}
