use reqwest::{blocking::Client, StatusCode};

use serde_json::{json, Value};

pub fn create_test_fruit(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/fruits")
        .json(&json!({
            "name": "Orange",
            "color": "Orange",
            "price": 3.5
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_fruit(client: &Client, fruit_value: Value) {
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/fruits/{}",
            fruit_value["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
