use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;


#[derive(Serialize)]
struct WeatherForecast {
    location: String,
    temperature: f64,
    description: String,
}

async fn get_forecast() -> impl Responder {
    let forecast = WeatherForecast {
        location: "Rome".to_string(),
        temperature: 28.0,
        description: "Sunny".to_string(),
    };

    HttpResponse::Ok().json(forecast)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/forecast", web::get().to(get_forecast))
    })
    .bind("192.168.1.58:8080")?
        .run()
        .await
}

