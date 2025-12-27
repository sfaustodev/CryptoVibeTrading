use leptos::*;
use serde::{Deserialize, Serialize};
use crate::database::{Database, User};
use std::sync::Arc;
use tokio::sync::RwLock;

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
    pub is_admin: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
}

// Database state (will be initialized in main.rs)
thread_local! {
    pub static DATABASE: std::cell::RefCell<Option<Arc<Database>>> = std::cell::cell::RefCell::new(None);
}

pub fn set_database(db: Arc<Database>) {
    DATABASE.with(|cell| cell.replace(Some(db)));
}

fn get_database() -> Result<Arc<Database>, ServerFnError> {
    DATABASE.with(|cell| {
        cell.borrow()
            .as_ref()
            .cloned()
            .ok_or_else(|| ServerFnError::new("Database not initialized"))
    })
}

// =====================
// Server Functions
// =====================

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    use std::env;

    // Hardcoded admin credentials (fallback)
    if username == "fenrir" && password == "$4taN" {
        return Ok(LoginResponse {
            success: true,
            token: Some("fenrir_admin_token".to_string()),
            message: "Login successful!".to_string(),
            is_admin: Some(true),
        });
    }

    // Database-backed login
    let db = get_database()?;

    match db.verify_credentials(&username, &password).await {
        Ok(Some(user)) => {
            let token = db.create_session(&user.id, 24).await
                .map_err(|e| ServerFnError::new(format!("Session creation failed: {}", e)))?;

            Ok(LoginResponse {
                success: true,
                token: Some(token),
                message: format!("Welcome back, {}!", user.username),
                is_admin: Some(user.is_admin),
            })
        }
        Ok(None) => Ok(LoginResponse {
            success: false,
            token: None,
            message: "Invalid username or password".to_string(),
            is_admin: None,
        }),
        Err(e) => Err(ServerFnError::new(format!("Database error: {}", e))),
    }
}

#[server(RegisterUser, "/api")]
pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<RegisterResponse, ServerFnError> {
    let db = get_database()?;

    // Validate input
    if username.len() < 3 {
        return Ok(RegisterResponse {
            success: false,
            message: "Username must be at least 3 characters".to_string(),
        });
    }

    if password.len() < 8 {
        return Ok(RegisterResponse {
            success: false,
            message: "Password must be at least 8 characters".to_string(),
        });
    }

    if !email.contains('@') || !email.contains('.') {
        return Ok(RegisterResponse {
            success: false,
            message: "Invalid email address".to_string(),
        });
    }

    // Check if user already exists
    if let Ok(Some(_)) = db.get_user_by_username(&username).await {
        return Ok(RegisterResponse {
            success: false,
            message: "Username already taken".to_string(),
        });
    }

    if let Ok(Some(_)) = db.get_user_by_email(&email).await {
        return Ok(RegisterResponse {
            success: false,
            message: "Email already registered".to_string(),
        });
    }

    // Create user
    match db.create_user(&username, &email, &password, false).await {
        Ok(_) => Ok(RegisterResponse {
            success: true,
            message: "Registration successful! Please login.".to_string(),
        }),
        Err(e) => Err(ServerFnError::new(format!("Registration failed: {}", e))),
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
