extern crate api;
use self::api::devices::models::Device;

use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};

fn main() {
    lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    let id = request.path_parameters().get("id").unwrap().to_owned();
    if let Some(device) = Device::find(id) {
        Ok(json!({"data": { "device": device }}))
    } else {
        Ok(json!({"message": "Device not found.", "code": 404}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::default();
        // let mut request = Request::builder();
        // request.method("GET");
        // request.uri("https://www.example.com/");
        let expected = json!({
            "data": {
                "device": {
                    "id": "RP_1",
                    "vid": "VT_1",
                    "lid": "LT_1",
                    "name": "Device 1",
                    "type": "raspi",
                    "tags": [],
                    "serial": null,
                    "group": null,
                    "notes": null,
                    "created": 1000000000
                }
            }
        }).into_response();
        let response = handler(request, Context::default()).expect("expected Ok(_) value").into_response();
        assert_eq!(response.body(), expected.body())
    }
}
