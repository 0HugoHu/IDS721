extern crate reqwest;

/*A library that returns back random fruit */
use json::JsonValue;


// Parse the response and return the result
pub async fn parse_result(location: &str) -> String {
    let response = get_response(location);
    let parsed = response.await;
    let lon = (&parsed["coord"]["lon"]).to_owned().as_f64().unwrap();
    let lat = (&parsed["coord"]["lat"]).to_owned().as_f64().unwrap();
    let city_name: &str = &parsed["name"].to_owned().to_string();
    let country_name: &str = &parsed["sys"]["country"].to_owned().to_string();
    let place = [city_name, ",", country_name].concat();
    let temp = ((&parsed["main"]["temp"]).to_owned().as_f64().unwrap() - 273.15) as i64;

    let ret : String = format!("The weather of {} located at ({}, {}) now is: {}F.\n", place, lon, lat, temp).to_string();
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