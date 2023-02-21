extern crate reqwest;

/*A library that returns back random fruit */
use json::JsonValue;


// Parse the weather response and return the result
pub async fn parse_weather_result(location: &str) -> String {
    let response = get_response(location);
    let parsed = response.await;
    let lon = (&parsed["coord"]["lon"]).to_owned().as_f64().unwrap();
    let lat = (&parsed["coord"]["lat"]).to_owned().as_f64().unwrap();
    let city_name: &str = &parsed["name"].to_owned().to_string();
    let country_name: &str = &parsed["sys"]["country"].to_owned().to_string();
    let place = [city_name, ",", country_name].concat();
    let temp = ((&parsed["main"]["temp"]).to_owned().as_f64().unwrap() - 273.15) as i64;

    let ret : String = format!("The weather in {} located at ({}, {}) now is: {}°C.\n", place, lon, lat, temp).to_string();
    return ret;
}

// Parse the air quality response and return the result
pub async fn parse_airquality_result(location: &str) -> String {
    let response = get_response_airquality(location);
    let parsed = response.await;
    
    // get pm2.5 value in the last 365 days
    // iterate from 0 to 365
    let mut i = 0;
    let mut air_quality = Vec::new();
    let mut out = false;
    while i < 365{
        let mut j = 0;
        let mut sum = 0.0;
        // get the average pm2.5 value in each day
        while j < 24 {
            let pm2_5 = aqi_pm25((&parsed["list"][i * 24 + j]["components"]["pm2_5"]).to_owned().as_f64().unwrap());
            println!("{}: {}: {}", i, j, pm2_5);
            sum = sum + pm2_5;
            j = j + 1;
            if i == 360 && j == 5 { out = true; break };
        }
        if out { air_quality.push(sum / 5.0); break };
        air_quality.push(sum / 24.0);
        i = i + 1;
    }

    let ret : String = format!("The 365 average daytimes Air Quality Index (AQI) in {} are:\n (0-50: Good, 51-100: Moderate, 101-150: Unhealthy for sensitive people, 151-200: Unhealthy, 201-300: Very unhealthy, 301-500: Hazardous)\n {:.2?}.\n", location, air_quality).to_string();
    return ret;
}

// Get response from weather api
async fn get_response(location: &str) -> JsonValue {
    let url1 = "http://api.openweathermap.org/data/2.5/weather?q=";
    let url2 = "&appid=1f565f89dbe934dc5916a2cc3f943d76";

    let url = [url1, location, url2].concat();

    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    json::parse(&resp).unwrap()
}

// Get response from air quality api
async fn get_response_airquality(location: &str) -> JsonValue {
    let url1 = "http://api.openweathermap.org/data/2.5/air_pollution/history?lat=";
    let url2 = "&lon=";
    let url3 = "&start=";
    let url4 = "&end=";
    let url5 = "&appid=1f565f89dbe934dc5916a2cc3f943d76";

    // get current timestamp
    let timestamp2 = chrono::Utc::now().timestamp();
    // get timestamp 365 days ago
    let timestamp1 = timestamp2 - 31556926;

    // get coordinates from location
    let response = get_coordinates(location);
    
    let parsed = response.await;

    let lon = (&parsed[0]["lon"]).to_owned().to_string();
    let lat = (&parsed[0]["lat"]).to_owned().to_string();


    let url = [url1, &lat, url2, &lon, url3, &timestamp1.to_string(), url4, &timestamp2.to_string(), url5].concat();

    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    json::parse(&resp).unwrap()
}

// Get coordinates from location
async fn get_coordinates(location: &str) -> JsonValue {
    let url1 = "http://api.openweathermap.org/geo/1.0/direct?q=";
    let url2 = "&appid=1f565f89dbe934dc5916a2cc3f943d76";

    let url = [url1, location, url2].concat();

    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    json::parse(&resp).unwrap()
}

fn aqi_pm25(aqi: f64) -> f64 {
    let a = aqi.round();
    let c = if a < 0.0 {
        0.0 // values below 0 are considered beyond AQI
    } else if a <= 50.0 {
        lerp(0.0, 12.0, 0.0, 50.0, a)
    } else if a <= 100.0 {
        lerp(12.1, 35.4, 51.0, 100.0, a)
    } else if a <= 150.0 {
        lerp(35.5, 55.4, 101.0, 150.0, a)
    } else if a <= 200.0 {
        lerp(55.5, 150.4, 151.0, 200.0, a)
    } else if a <= 300.0 {
        lerp(150.5, 250.4, 201.0, 300.0, a)
    } else if a <= 400.0 {
        lerp(250.5, 350.4, 301.0, 400.0, a)
    } else if a <= 500.0 {
        lerp(350.5, 500.4, 401.0, 500.0, a)
    } else {
        500.0 // values above 500 are considered beyond AQI
    };
    (c * 10.0).floor() / 10.0
}

fn lerp(x1: f64, x2: f64, y1: f64, y2: f64, x: f64) -> f64 {
    y1 + (x - x1) * (y2 - y1) / (x2 - x1)
}

