#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
use rocket::{launch, get, build};
use serde::Serialize;

mod rand_int;
mod rand_float;
mod rand_string;

#[derive(Serialize)]
struct RandomResult<T> {
    result: T,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Random Generator</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                margin: 0;
                background-color: #f0f0f0;
            }
            .container {
                text-align: center;
                background: #fff;
                padding: 20px;
                border-radius: 10px;
                box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
            }
            h1 {
                margin-bottom: 20px;
            }
            button {
                padding: 10px 20px;
                margin: 10px;
                font-size: 16px;
                border: none;
                border-radius: 5px;
                cursor: pointer;
                transition: background-color 0.3s ease;
            }
            button:hover {
                background-color: #ddd;
            }
        </style>
        <script>
            async function fetchRandom(endpoint) {
                try {
                    const response = await fetch(endpoint);
                    const data = await response.json();
                    alert(data.result);
                } catch (error) {
                    console.error('Error fetching data:', error);
                    alert('Failed to fetch data.');
                }
            }
        </script>
    </head>
    <body>
        <div class="container">
            <h1>Random Generator</h1>
            <button onclick="fetchRandom('/random-int')">Generate Random Int</button>
            <button onclick="fetchRandom('/random-float')">Generate Random Float</button>
            <button onclick="fetchRandom('/random-string')">Generate Random String</button>
        </div>
    </body>
    </html>
    "#)
}

#[get("/random-int")]
fn random_int() -> Json<RandomResult<i32>> {
    Json(RandomResult { result: rand_int::generate_random_int() })
}

#[get("/random-float")]
fn random_float() -> Json<RandomResult<f64>> {
    Json(RandomResult { result: rand_float::generate_random_float() })
}

#[get("/random-string")]
fn random_string() -> Json<RandomResult<String>> {
    Json(RandomResult { result: rand_string::generate_random_string() })
}

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![index, random_int, random_float, random_string])
}
