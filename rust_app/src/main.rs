use lambda_http::{service_fn, Body, Error, Request, Response};

#[derive(serde::Deserialize)]
struct RequestBody {
    message: String,
}
// struct RequestBody {
//     numbers: Vec<i32>,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    match request.body() {
        Body::Text(body) => {
            match serde_json::from_str::<RequestBody>(body) {
                Ok(body) => Ok(Response::builder()
                    .body(Body::Text(format!("{}", body.message.to_uppercase())))?),
                // .body(Body::Text(format!("{:?}", merge_sort(&body.numbers))))?),
                Err(error) => {
                    Ok(Response::builder().body(Body::Text(format!("ERROR: {:?}", error)))?)
                }
            }
        }
        _ => panic!(),
    }
}

// fn merge_sorted_arrays(left: &[i32], right: &[i32]) -> Vec<i32> {
//     let (mut i, mut j) = (0, 0);

//     let mut merged = Vec::with_capacity(left.len() + right.len());

//     while i < left.len() && j < right.len() {
//         if left[i] < right[j] {
//             merged.push(left[i]);
//             i += 1
//         } else {
//             merged.push(right[j]);
//             j += 1
//         }
//     }

//     merged.append(&mut match i < left.len() {
//         true => left[i..].to_vec(),
//         false => right[j..].to_vec(),
//     });

//     merged
// }

// fn merge_sort(arr: &[i32]) -> Vec<i32> {
//     if arr.len() == 1 {
//         return arr.to_vec();
//     }

//     let middle = arr.len() / 2;

//     let left = merge_sort(&arr[..middle]);
//     let right = merge_sort(&arr[middle..]);

//     merge_sorted_arrays(&left, &right)
// }
