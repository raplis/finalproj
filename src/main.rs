use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct InputData {
    text: String,
    max_length: Option<u32>,
}

#[derive(Serialize)]
struct Prediction {
    result: Value,
}

async fn predict(text_data: web::Json<InputData>) -> impl Responder {
    let max_length = text_data.max_length.unwrap_or(50).to_string();
    let output = Command::new("python3")
        .arg("app.py")
        .arg(&text_data.text)
        .arg(max_length)
        .output();

    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            match serde_json::from_str(&output_str) {
                Ok(prediction_result) => HttpResponse::Ok().json(Prediction {
                    result: prediction_result,
                }),
                Err(_) => HttpResponse::InternalServerError().json("Failed to parse JSON"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to execute command"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .route("/predict", web::post().to(predict))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_predict_endpoint() {
        let app = test::init_service(App::new().route("/predict", web::post().to(predict))).await;
        let req = test::TestRequest::post()
            .uri("/predict")
            .set_json(&InputData { text: "Hello, world!".to_string(), max_length: Some(50) })
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
