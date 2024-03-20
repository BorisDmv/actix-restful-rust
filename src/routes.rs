use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u32,
    city: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/data")]
pub async fn show_data() -> impl Responder {
    // Create example Person objects
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("New York"),
    };

    let person2 = Person {
        name: String::from("Bob"),
        age: 25,
        city: String::from("Los Angeles"),
    };

    // Create a vector of Person objects
    let people = vec![person1, person2];

    // Serialize the vector to JSON
    let json_data = serde_json::to_string(&people).unwrap();

    // Return the serialized JSON as the response body
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_data)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // Parse the JSON string into a JSON object
    let parsed_json: Value = serde_json::from_str(&req_body)
        .unwrap_or_else(|_| json!({"error": "Invalid JSON"}));

    // Construct a JSON object indicating successful submission
    let response_json = json!({
        "message": "Submit successful",
        "data": parsed_json
    });

    // Respond with the JSON object
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response_json)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
