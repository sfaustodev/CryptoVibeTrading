#![forbid(unsafe_code)]

use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;
use cvt::App;

// =====================
// Shared API types
// =====================

#[derive(Debug, Clone, Deserialize)]
pub struct AiRequest {
    pub prompt: String,
    pub system_instruction: Option<String>,
    pub asset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub message: String,
}

// =====================
// Server Functions
// =====================

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    // Hardcoded admin credentials
    if username == "fenrir" && password == "$4taN" {
        Ok(LoginResponse {
            success: true,
            token: Some("fenrir_admin_token".to_string()),
            message: "Login successful!".to_string(),
        })
    } else {
        Ok(LoginResponse {
            success: false,
            token: None,
            message: "Invalid credentials".to_string(),
        })
    }
}

#[server(GrokAnalyze, "/api")]
pub async fn grok_analyze(
    prompt: String,
    selected_text: String,
    include_screenshot: bool,
) -> Result<String, ServerFnError> {
    use std::env;

    let api_key = env::var("XAI_API_KEY")
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        return Ok(format!(
            "🐺 Fenrir Grok Analysis\n\n⚠️ Please add XAI_API_KEY to .env file\nGet from: https://x.ai/\n\nSelected: {}\nPrompt: {}\n\n• Risk analysis requires real API\n• Please configure XAI_API_KEY\n\nDYOR!",
            selected_text, prompt
        ));
    }

    let system = "You're a traditional professional of risk analysis and on chain analyst using blockchain protocols and explorers official free apis and really calculating the risk and possible PnL. Provide detailed analysis with specific numbers, calculations, and risk assessments.";

    let user_prompt = if include_screenshot {
        format!("{}(Screenshot attached) Help me out, explain me wtf is all of this, what am I doing, should I continue? Analyze the screenshot and provide detailed risk assessment.", selected_text)
    } else {
        format!("Selected text: {}\nQuestion: {}", selected_text, prompt)
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
    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&grok_req)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("network error: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "provider error: {} {}",
            status, err
        )));
    }

    #[derive(Deserialize)]
    struct GrokResp {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        message: ChoiceMsg,
    }

    #[derive(Deserialize)]
    struct ChoiceMsg {
        content: String,
    }

    let parsed = resp
        .json::<GrokResp>()
        .await
        .map_err(|e| ServerFnError::new(format!("decode error: {}", e)))?;

    Ok(parsed
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "No response from Grok. DYOR!".to_string()))
}

// =====================
// Gemini API (for public page)
// =====================

#[server(AiAnalyze, "/api")]
pub async fn ai_analyze(
    prompt: String,
    asset: String,
    indicators: String,
) -> Result<String, ServerFnError> {
    use std::env;

    let api_key = env::var("GEMINI_API_KEY")
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        return Ok(format!(
            "🤖 Fenrir AI Analysis for {}\n\nIndicators: {}\n\n• Please add GEMINI_API_KEY to .env file for real AI analysis\n• Get free API key from: https://aistudio.google.com/app/apikey\n\nCurrent analysis for {} based on {}:\n\n• Price action showing momentum\n• Key support levels holding\n• Wait for confirmation before entry\n\nDYOR!",
            asset, indicators, asset, indicators
        ));
    }

    let system = format!(
        "You are Fenrir AI, a Senior Technical Analyst specializing in cryptocurrency markets. \
        Current asset: {}. \
        Indicators on screen: {}. \
        Provide a concise, direct technical analysis. \
        Use bullet points. \
        Be specific about support/resistance levels. \
        End with DYOR. \
        Keep response under 150 words for voice synthesis.",
        asset, indicators
    );

    let gemini_req = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": format!("{}\n\nUser: {}", system, prompt.trim())
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
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&gemini_req)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("network error: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "provider error: {} {}",
            status, err
        )));
    }

    #[derive(Deserialize)]
    struct GeminiResp {
        candidates: Vec<Candidate>,
    }

    #[derive(Deserialize)]
    struct Candidate {
        content: CandidateContent,
    }

    #[derive(Deserialize)]
    struct CandidateContent {
        parts: Vec<CandidatePart>,
    }

    #[derive(Deserialize)]
    struct CandidatePart {
        text: String,
    }

    let parsed = resp
        .json::<GeminiResp>()
        .await
        .map_err(|e| ServerFnError::new(format!("decode error: {}", e)))?;

    Ok(parsed
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "No response from Gemini. DYOR!".to_string()))
}

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
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    let addr = std::env::var("CVT_ADDR")
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));

    tracing::info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("server failed");
}
