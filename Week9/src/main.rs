use std::error::Error;

#[tokio::main]
async fn fetch() -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("https://finance.yahoo.com/quote/AMZN/")
        .await?
        .text().await?;

    //println!("body = {:?}", body);

    let substring_pos = body.find("data-symbol=\"AMZN\"").unwrap();
    // println!("{}", substring_pos);
    let data = &body[substring_pos..(substring_pos+200)];
    let value_pos = data.find("value=").unwrap();
    let amzn_value = &data[(value_pos+7)..(value_pos+12)];
    println!("Amazon (AMZN) stock price: {}", amzn_value);
    Ok(())
}

fn main(){
    println!("fetching Amazon (AMZN) stock price...");
    let res = fetch();
}
