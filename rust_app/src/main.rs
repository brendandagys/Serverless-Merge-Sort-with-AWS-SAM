use lambda_http::{service_fn, Body, Error, Request, Response};

#[derive(serde::Deserialize)]
struct RequestBody {
    numbers: String,
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
                .body(Body::Text(format!(
                    "{:?}",
                    merge_sort(&split_string_to_ints(&body.numbers))
                )))?),
            Err(error) => Ok(Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "OPTIONS, POST")
                .body(Body::Text(format!("ERROR: {error}")))?),
        },
        _ => panic!(),
    }
}

// Function to take a string, split it on the commas, remove whitespace, and create a vector of integers
fn split_string_to_ints(string: &str) -> Vec<i32> {
    string
        .split(',')
        .into_iter()
        .filter_map(|c| {
            let trimmed = c.trim();
            if trimmed.len() > 0 {
                return match trimmed.parse::<i32>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
                };
            }

            None
        })
        .collect()
}

fn merge_sorted_arrays(left: &[i32], right: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);

    let mut merged = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1
        } else {
            merged.push(right[j]);
            j += 1
        }
    }

    merged.append(&mut match i < left.len() {
        true => left[i..].to_vec(),
        false => right[j..].to_vec(),
    });

    merged
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return arr.to_vec();
    }

    let middle = arr.len() / 2;

    let left = merge_sort(&arr[..middle]);
    let right = merge_sort(&arr[middle..]);

    merge_sorted_arrays(&left, &right)
}
