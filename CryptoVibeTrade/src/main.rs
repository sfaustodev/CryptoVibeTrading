#![forbid(unsafe_code)]

use leptos::*;
use axum::response::IntoResponse;
use cryptovibetrading::App;

// =====================
// Grok API Handler for Direct API Calls
// =====================

async fn grok_handler(
    axum::Json(payload): axum::Json<serde_json::Value>,
) -> impl IntoResponse {
    use std::env;

    let prompt = payload.get("prompt")
        .and_then(|p| p.as_str())
        .unwrap_or("Analyze this");

    let selected_text = payload.get("selectedText")
        .and_then(|t| t.as_str())
        .unwrap_or("");

    let include_screenshot = payload.get("includeScreenshot")
        .and_then(|i| i.as_bool())
        .unwrap_or(false);

    let api_key = env::var("XAI_API_KEY")
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        let response = serde_json::json!({
            "response": format!(
                "🐺 Fenrir Grok Analysis\n\n⚠️ Demo Mode\nAdd XAI_API_KEY to .env\n\nSelected: {}\n{}",
                selected_text,
                if include_screenshot { "(Screenshot would be attached)" } else { "" }
            )
        });
        return axum::Json(response);
    }

    let system = "You're a traditional professional of risk analysis and on chain analyst using blockchain protocols and explorers official free apis and really calculating the risk and possible PnL.";

    let user_prompt = if include_screenshot {
        format!("{} Help me out, explain me wtf is all of this, what am I doing, should I continue?", selected_text)
    } else {
        format!("Selected: {}\nQuestion: {}", selected_text, prompt)
    };

    let grok_req = serde_json::json!({
        "model": "grok-beta",
        "messages": [
            {
                "role": "system",
                "content": system
            },
            {
                "role": "user",
                "content": user_prompt
            }
        ],
        "temperature": 0.7
    });

    let url = "https://api.x.ai/v1/chat/completions";

    let client = reqwest::Client::new();
    match client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&grok_req)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => {
                        if let Some(text) = json["choices"][0]["message"]["content"].as_str() {
                            axum::Json(serde_json::json!({ "response": text }))
                        } else {
                            axum::Json(serde_json::json!({ "response": "No response from Grok. DYOR!" }))
                        }
                    }
                    Err(e) => {
                        axum::Json(serde_json::json!({ "error": format!("Parse error: {}", e) }))
                    }
                }
            } else {
                axum::Json(serde_json::json!({ "error": format!("Grok API error: {}", resp.status()) }))
            }
        }
        Err(e) => {
            axum::Json(serde_json::json!({ "error": format!("Network error: {}", e) }))
        }
    }
}

// =====================
// Server Entry Point
// =====================

async fn gemini_handler(
    axum::Json(payload): axum::Json<serde_json::Value>,
) -> impl IntoResponse {
    use std::env;

    let prompt = payload.get("prompt")
        .and_then(|p| p.as_str())
        .unwrap_or("Analyze this market");

    let asset = payload.get("asset")
        .and_then(|a| a.as_str())
        .unwrap_or("BTC");

    let indicators = payload.get("indicators")
        .and_then(|i| i.as_str())
        .unwrap_or("Ichimoku Cloud, RSI, MACD");

    let api_key = env::var("GEMINI_API_KEY")
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        let response = serde_json::json!({
            "response": format!(
                "🤖 Fenrir AI Analysis for {}\n\nIndicators: {}\n\n⚠️ Please add GEMINI_API_KEY to .env file\nGet free key: https://aistudio.google.com/app/apikey\n\nAnalysis:\n• Price action showing {}\n• Support levels holding\n• Momentum indicators bullish\n• Wait for confirmation\n\nDYOR!",
                asset, indicators, asset
            )
        });
        return axum::Json(response);
    }

    let system = format!(
        "You are Fenrir AI, a Senior Technical Analyst for cryptocurrency markets. \
        Asset: {}. Indicators: {}. \
        Provide concise analysis with bullet points. \
        End with DYOR. Keep under 150 words.",
        asset, indicators
    );

    let gemini_req = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": format!("{}\n\nUser: {}", system, prompt)
            }]
        }],
        "generationConfig": {
            "temperature": 0.4,
            "maxOutputTokens": 500
        }
    });

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    match client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&gemini_req)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => {
                        if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                            axum::Json(serde_json::json!({ "response": text }))
                        } else {
                            axum::Json(serde_json::json!({ "response": "No response from Gemini. DYOR!" }))
                        }
                    }
                    Err(e) => {
                        axum::Json(serde_json::json!({ "error": format!("Parse error: {}", e) }))
                    }
                }
            } else {
                axum::Json(serde_json::json!({ "error": format!("Gemini API error: {}", resp.status()) }))
            }
        }
        Err(e) => {
            axum::Json(serde_json::json!({ "error": format!("Network error: {}", e) }))
        }
    }
}

#[tokio::main]
async fn main() {
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        routing::post,
        Router,
    };
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    use cryptovibetrading::database::Database;
    use cryptovibetrading::server::set_database;

    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database (optional - will fail gracefully if not available)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/cryptovibetrading".to_string());

    tracing::info!("Attempting to connect to database...");
    match Database::new(&database_url).await {
        Ok(db) => {
            // Run migrations
            if let Err(e) = db.run_migrations().await {
                tracing::warn!("Database migrations failed: {}", e);
            } else {
                tracing::info!("Database migrations completed");
            }

            // Seed admin user
            if let Err(e) = db.seed_admin_user().await {
                tracing::warn!("Admin user seeding failed: {}", e);
            } else {
                tracing::info!("Admin user seeded successfully");
            }

            // Make database available to server functions
            let db = Arc::new(db);
            set_database(db);
            tracing::info!("Database connected and initialized");
        }
        Err(e) => {
            tracing::warn!("Database connection failed: {}. App will run in limited mode without auth.", e);
            tracing::warn!("To enable auth, start PostgreSQL: docker-compose up -d");
        }
    }

    let leptos_options = LeptosOptions::default();
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/gemini", post(gemini_handler))
        .route("/api/grok", post(grok_handler))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(|| async {
            (StatusCode::NOT_FOUND, "Not Found").into_response()
        })
        .with_state(leptos_options);

    let addr = std::env::var("CRYPTOVIBETRADING_ADDR")
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));

    tracing::info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("server failed");
}
