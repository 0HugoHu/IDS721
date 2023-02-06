// Find city by altitude
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};

use actix_web::http::{header, Method, StatusCode};

use actix_files::NamedFile;

use serde::Deserialize;

use std::{env, io};


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

// Parse statistics
#[get("/sta/{a}")]
async fn cal_statistics(info: web::Path<String>) -> impl Responder {

    let mut sum : f32 = 0.0;
    let mut num : i32 = 0;
    let mut mean : f32 = 0.0;
    let mut min : f32 = 0.0;
    let mut max : f32 = 0.0;
    
    
    // create a vector to store all the altitudes
    let mut altitudes : Vec<f32> = Vec::new();

    // Obtain data and compare with given altitude
    let rdr = csv::Reader::from_path("WorldCityLocations/World_Cities_Location_table.csv");

    let mut result = String::new();

    for record in rdr.expect("").deserialize::<Data>() {
        match record {
            Err(why) => {}
            Ok(record) => {
                sum += record.altitude;
                num += 1;
                if record.altitude < min {
                    min = record.altitude;
                }
                if record.altitude > max {
                    max = record.altitude;
                }
                altitudes.push(record.altitude);
            }
        }
    }

    // convert info to string
    let info = info.as_str();
    
    match info {
        "mean" => {
            // calculate mean
            mean = sum / num as f32;
            // convert mean to string
            result = format!("The mean of all the altitudes is {}m.\n", mean).to_string();
        }
        "median" => {
            // sort altitudes
            altitudes.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut median : f32 = 0.0;
            // find the midium altitude
            if num % 2 == 0 {
                median = (altitudes[num as usize / 2] + altitudes[num as usize / 2 - 1]) / 2.0;
            } else {
                median = altitudes[num as usize / 2];
            }
            // conver median to string
            result = format!("The median of all the altitudes is {}m.\n", median).to_string();
        }
        "range" => {
            // convert range to string
            result = format!("The range of all the altitudes is {}m. While the min is {}m, and the max is {}m\n", max - min, min, max).to_string();
        }
        "stddev" => {
            let mut variance : f32 = 0.0;
            // calculate variance
            for i in 0..num {
                variance += (altitudes[i as usize] - mean).powf(2.0);
            }
            variance /= num as f32;
            // calculate stddev
            let stddev : f32 = variance.sqrt();
            // convert stddev to string
            result = format!("The standard deviation of all the altitudes is {}m.\n", stddev).to_string();
        }
        "variance" => {
            let mut variance : f32 = 0.0;
            // calculate variance
            for i in 0..num {
                variance += (altitudes[i as usize] - mean).powf(2.0);
            }
            variance /= num as f32;
            // convert variance to string
            result = format!("The variance of all the altitudes is {}m.\n", variance).to_string();
        }
        _ => {
            result = format!("Please input the correct statistics!\n").to_string();
        }
    }

    HttpResponse::Ok().body(result)
}


// // Generate graph
// #[get("/figure/{a}")]
// async fn return_figure(info: web::Path<String>) -> Result<HttpResponse> {

//     // Y = a + b·X + c·X2 + d·X3 + e·X4
//     // a = 3060.38628172611
//     // b = -511.724627659983
//     // c = 29.7645556966008
//     // d = -0.713129242306818
//     // e = 0.00600206802860897
    
//     // 3060 + -511.7246·X + 29.7645 + -0.7131·X3 + 0.006·X4

//     let mut result = String::new();
//     result = format!("
    //                                                 Wrold's City Attitude Frequency
    //                               y = 3060 + -511.7246*x + 29.7645*x^2 + -0.7131*x^3 + 0.006*x^4
    // ⣣                                                                                                                     3060 cities
    // ⠌⢆
    // ⠂⠈⢆
    // ⡁ ⠈⢆
    // ⠄  ⠈⢆
    // ⠂   ⠈⢆
    // ⡁     ⢣
    // ⠄      ⠱⡀
    // ⠂       ⠘⢄
    // ⡁         ⠱⡀
    // ⠄          ⠈⠢⡀
    // ⠂            ⠈⠢⣀
    // ⡁               ⠑⠤⡀
    // ⠄                 ⠈⠑⠢⢄⣀
    // ⡂⢀ ⡀⢀ ⡀⢀ ⡀⢀ ⡀⢀ ⡀⢀ ⡀⢀⢉⢉⢉⠉⡑⢒⠢⡤⢤⢄⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⢤⠤⡤⢤⠤⡤⢒⠒⡒⢒⠒⡒⢒⠊⡉⢉⠉⡉⢉⠉⡉⢉⠉⡉⢒⠒⡒⢒⠒⡒⢢⠤⡤⢤⠤⣄⣀⣀⣀⣀⡀⡀⢀ ⡀⢀ ⡀⢀ ⡀
    // ⠁                                                                               ⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉
    // 0                         1000                           2000                           3000                           4000 m"
//     ).to_string();


//     let image_content = web::block(|| std::fs::read("/generated_figure.png".to_string())).await;
//     // let image_content =  actix_web::web::Bytes::from(std::fs::read("/generated_figure.png".to_string()));
//     // Ok(HttpResponse::build(StatusCode::OK)
//     // .content_type("image/jpeg")
//     // .body(actix_web::dev::AnyBody::Bytes(image_content)))

    
//     // HttpResponse::Ok()
//     //                 .content_type("image/jpeg")
//     //                 .body(image_content)

//     HttpResponse::Ok(NamedFile::open("/generated_figure.png"))

//     // let file = actix_files::NamedFile::open_async("/generated_figure.png").await.unwrap();

//     // file.into_response()

// }

// Generate graph
#[get("/figure/{a}")]
async fn return_html() -> HttpResponse {

    let html = r#"
        <html>
            <head>
                <title>My Page</title>
            </head>
            <body>
                <div> 
&nbsp;&nbsp;&nbsp;&nbsp;Wrold's City Attitude Frequency<br><br>
&nbsp;&nbsp;&nbsp;&nbsp;y&nbsp;=&nbsp;3060&nbsp;+&nbsp;-511.7246*x&nbsp;+&nbsp;29.7645*x^2&nbsp;+&nbsp;-0.7131*x^3&nbsp;+&nbsp;0.006*x^4<br><br>
&nbsp;&nbsp;&nbsp;&nbsp;⣣&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;3060&nbsp;cities<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠌⢆<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠂⠈⢆<br>
&nbsp;&nbsp;&nbsp;&nbsp;⡁&nbsp;⠈⢆<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠄&nbsp;&nbsp;⠈⢆<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠂&nbsp;&nbsp;&nbsp;⠈⢆<br>
&nbsp;&nbsp;&nbsp;&nbsp;⡁&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⢣<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠄&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠱⡀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠂&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠘⢄<br>
&nbsp;&nbsp;&nbsp;&nbsp;⡁&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠱⡀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠄&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠈⠢⡀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠂&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠈⠢⣀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⡁&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠑⠤⡀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠄&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠈⠑⠢⢄⣀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⡂⢀&nbsp;⡀⢀&nbsp;⡀⢀&nbsp;⡀⢀&nbsp;⡀⢀&nbsp;⡀⢀&nbsp;⡀⢀⢉⢉⢉⠉⡑⢒⠢⡤⢤⢄⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⢤⠤⡤⢤⠤⡤⢒⠒⡒⢒⠒⡒⢒⠊⡉⢉⠉⡉⢉⠉⡉⢉⠉⡉⢒⠒⡒⢒⠒⡒⢢⠤⡤⢤⠤⣄⣀⣀⣀⣀⡀⡀⢀&nbsp;⡀⢀&nbsp;⡀⢀&nbsp;⡀<br>
&nbsp;&nbsp;&nbsp;&nbsp;⠁&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉<br>
0&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;1000&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2000&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;3000&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;4000&nbsp;m<br>
                </div>
            </body>
        </html>
    "#;

    HttpResponse::Ok().content_type("text/html; charset=UTF-8").body(html)
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(run).service(cal_statistics).service(return_html))
        // .resource("/figure", |r| r.f(return_html))
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
}
