use lambda_http::{service_fn, Body, Error, Request, Response};

mod merge_sort;
use merge_sort::merge_sort;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    match request.body() {
        Body::Text(body) => Ok(Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "OPTIONS, POST")
            .body(Body::Text(format!(
                "{:?}",
                merge_sort(&split_string_into_vec_of_ints(body))
            )))?),
        _ => Ok(Response::new(Body::Empty)),
    }
}

// Function to split a string on commas and create a vector of integers from it
fn split_string_into_vec_of_ints(string: &str) -> Vec<i32> {
    string
        .split(',')
        .into_iter()
        .filter_map(|c| match c.trim().parse::<i32>() {
            Ok(i) => Some(i),
            Err(_) => None,
        })
        .collect()
}
