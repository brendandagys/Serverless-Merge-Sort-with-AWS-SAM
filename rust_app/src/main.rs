use lambda_http::{service_fn, Body, Error, Request, Response};

#[derive(serde::Deserialize)]
struct RequestBody {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    match request.body() {
        Body::Text(body) => match serde_json::from_str::<RequestBody>(body) {
            Ok(body) => Ok(Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "OPTIONS, POST")
                .body(Body::Text(format!("{}", body.message.to_uppercase())))?),
            Err(error) => Ok(Response::builder().body(Body::Text(format!("ERROR: {:?}", error)))?),
        },
        _ => panic!(),
    }
}
