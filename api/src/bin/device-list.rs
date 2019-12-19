extern crate api;
use self::api::devices::models::Device;

use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{json};

fn main() {
    lambda!(handler)
}

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!({"data": { "devices": Device::all() }}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request = Request::default();
        let expected = json!({
            "data": {
                "devices": [
                    {
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
                    },
                    {
                        "id": "IP_2",
                        "vid": "VT_2",
                        "lid": "LT_2",
                        "name": "Device 2",
                        "type": "ip-camera",
                        "tags": ["ip-camera", "hallway"],
                        "serial": "IP_2_SERIAL",
                        "group": null,
                        "notes": null,
                        "created": 1000000000
                    }
                ]
            }
        }).into_response();
        let response = handler(request, Context::default()).expect("expected Ok(_) value").into_response();
        assert_eq!(response.body(), expected.body())
    }
}
