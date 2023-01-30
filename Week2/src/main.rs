use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let name = event.payload.name;
    // get the max frequency character in the string
    let mut max_freq = 1;
    let mut max_char = name.chars().nth(0).unwrap();
    for c in name.chars() {
        let freq = name.matches(c).count();
        if freq > max_freq {
            max_freq = freq;
            max_char = c;
        }
    }
    // let logic be the max frequency character + its frequency
    let logic = format!("{} that appears {} time(s)!", max_char.to_string(), max_freq);

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!(
            "The string:{}, has the highest frequency of {}",
            name, logic
        ),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
