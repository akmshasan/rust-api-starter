use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_fruits() {
    // Setup
    let client = Client::new();
    let fruit_value1 = common::create_test_fruit(&client);
    let fruit_value2 = common::create_test_fruit(&client);

    // Test
    let response = client.get("http://127.0.0.1:8000/fruits").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let fruit_array: Value = response.json().unwrap();
    assert!(fruit_array.as_array().unwrap().contains(&fruit_value1));
    assert!(fruit_array.as_array().unwrap().contains(&fruit_value2));

    // Cleanup
    common::delete_test_fruit(&client, fruit_value1);
    common::delete_test_fruit(&client, fruit_value2);
}

#[test]
fn test_create_fruit() {
    // Setup
    let client = Client::new();
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

    // Test
    let fruit_value: Value = response.json().unwrap();
    assert_eq!(
        fruit_value,
        json!({
            "id": fruit_value["id"],
            "name": "Orange",
            "color": "Orange",
            "price": 3.5,
            "created_at": fruit_value["created_at"],
            "updated_at": fruit_value["updated_at"]
        })
    );

    // Cleanup
    common::delete_test_fruit(&client, fruit_value);
}

#[test]
fn test_get_fruit() {
    // Setup
    let client = Client::new();
    let fruit_value = common::create_test_fruit(&client);

    // Test - Pass
    let response = client
        .get(format!(
            "http://127.0.0.1:8000/fruits/{}",
            fruit_value["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        fruit_value,
        json!({
            "id": fruit_value["id"],
            "name": "Orange",
            "color": "Orange",
            "price": 3.5,
            "created_at": fruit_value["created_at"],
            "updated_at": fruit_value["updated_at"]
        })
    );

    // Cleanup
    common::delete_test_fruit(&client, fruit_value);
}

#[test]
fn test_get_fruit_fail() {
    // Setup
    let client = Client::new();
    let fruit_value = common::create_test_fruit(&client);

    // Test - Pass
    let response = client
        .get(format!("http://127.0.0.1:8000/fruits/999999"))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Cleanup
    common::delete_test_fruit(&client, fruit_value);
}

#[test]
fn test_update_fruit() {
    // Setup
    let client = Client::new();
    let fruit_value = common::create_test_fruit(&client);

    // Test
    let response = client
        .put(format!(
            "http://127.0.0.1:8000/fruits/{}",
            fruit_value["id"]
        ))
        .json(&json!({
            "price": 4.5
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let fruit_value: Value = response.json().unwrap();
    assert_eq!(
        fruit_value,
        json!({
            "id": fruit_value["id"],
            "name": fruit_value["name"],
            "color": fruit_value["color"],
            "price": 4.5,
            "created_at": fruit_value["created_at"],
            "updated_at": fruit_value["updated_at"]
        })
    );

    // Cleanup
    common::delete_test_fruit(&client, fruit_value)
}

#[test]
fn test_update_fruit_fail() {
    // Setup
    let client = Client::new();
    let fruit_value = common::create_test_fruit(&client);

    // Test
    let response = client
        .put(format!("http://127.0.0.1:8000/fruits/99999",))
        .json(&json!({
            "price": 4.5
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Cleanup
    common::delete_test_fruit(&client, fruit_value)
}

#[test]
fn test_delete_fruit() {
    let client = Client::new();
    let fruit_value = common::create_test_fruit(&client);
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/fruits/{}",
            fruit_value["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
