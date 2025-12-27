use leptos::*;
use serde::{Deserialize, Serialize};
use crate::database::{Database, User};
use std::sync::Arc;
use tokio::sync::RwLock;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNftRequest {
    pub wallet_address: String,
    pub mint_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyNftResponse {
    pub is_holder: bool,
    pub message: String,
}

// Database state (will be initialized in main.rs)
thread_local! {
    pub static DATABASE: std::cell::RefCell<Option<Arc<Database>>> = std::cell::RefCell::new(None);
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

    // Try GROK_API_KEY first (from .zshrc), fall back to XAI_API_KEY
    let api_key = env::var("GROK_API_KEY")
        .or_else(|_| env::var("XAI_API_KEY"))
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        return Ok(format!(
            "🐺 Fenrir Grok Analysis\n\n⚠️ Please add GROK_API_KEY to .env file or export it in your shell\nGet from: https://x.ai/\n\nAdd to .env: GROK_API_KEY=your_key_here\nOr export: export GROK_API_KEY=your_key_here\n\nSelected: {}\nPrompt: {}\n\n• Risk analysis requires real API\n• Please configure GROK_API_KEY\n\nDYOR!",
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
        .unwrap_or_else(|_| "AIzaSyBP-LZfj0FfCqGRuOiQVd9sTB0cjqq_LMg".to_string());

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

    // Using Gemini 2.5 Pro (latest best model)
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={}",
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
// GLM API (Zhipu AI - Alternative)
// =====================

#[server(GlmAnalyze, "/api")]
pub async fn glm_analyze(
    prompt: String,
    context: String,
) -> Result<String, ServerFnError> {
    use std::env;

    let api_key = env::var("GLM_API_KEY")
        .unwrap_or_else(|_| "demo_key".to_string());

    if api_key == "demo_key" {
        return Ok(format!(
            "🤖 GLM AI Analysis (Zhipu AI)\n\n⚠️ Please add GLM_API_KEY to .env file\nGet from: https://open.bigmodel.cn/\n\nContext: {}\nPrompt: {}\n\n• Analysis requires real API\n• Please configure GLM_API_KEY\n\nDYOR!",
            context, prompt
        ));
    }

    let system = "You are Fenrir AI, a professional cryptocurrency analyst. Provide detailed technical analysis with specific insights about market trends, support/resistance levels, and risk assessment. Be concise but thorough.";

    let glm_req = serde_json::json!({
        "model": "glm-4",
        "messages": [
            {
                "role": "system",
                "content": system
            },
            {
                "role": "user",
                "content": format!("Context: {}\n\nQuestion: {}", context, prompt)
            }
        ],
        "temperature": 0.7,
        "top_p": 0.9,
        "max_tokens": 1000
    });

    let url = "https://open.bigmodel.cn/api/paas/v4/chat/completions";

    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&glm_req)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("network error: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "GLM API error: {} {}",
            status, err
        )));
    }

    #[derive(Deserialize)]
    struct GlmResp {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        message: GlmMessage,
    }

    #[derive(Deserialize)]
    struct GlmMessage {
        content: String,
    }

    let parsed = resp
        .json::<GlmResp>()
        .await
        .map_err(|e| ServerFnError::new(format!("decode error: {}", e)))?;

    Ok(parsed
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "No response from GLM. DYOR!".to_string()))
}

// =====================
// Solana NFT Verification
// =====================

#[server(VerifyNft, "/api")]
pub async fn verify_nft(
    wallet_address: String,
    mint_address: String,
) -> Result<VerifyNftResponse, ServerFnError> {
    use std::env;

    // Parse wallet address
    let wallet_pubkey = Pubkey::from_str(&wallet_address)
        .map_err(|_| ServerFnError::new("Invalid wallet address"))?;

    // Parse mint address
    let mint_pubkey = Pubkey::from_str(&mint_address)
        .map_err(|_| ServerFnError::new("Invalid mint address"))?;

    // Connect to Solana RPC (using devnet for testing, can be mainnet-beta)
    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let client = RpcClient::new(rpc_url);

    // Get the token account for the NFT
    match client.get_token_accounts_by_owner(
        &wallet_pubkey,
        solana_client::rpc_request::TokenAccountsFilter::Mint(mint_pubkey),
    ) {
        Ok(token_accounts) => {
            if token_accounts.is_empty() {
                Ok(VerifyNftResponse {
                    is_holder: false,
                    message: "No NFT found in wallet".to_string(),
                })
            } else {
                // Simply check if token account exists (balance > 0 for NFTs)
                Ok(VerifyNftResponse {
                    is_holder: true,
                    message: "NFT verified! Access granted to Grok analysis.".to_string(),
                })
            }
        }
        Err(e) => {
            leptos::logging::log!("Solana RPC error: {:?}", e);
            // For demo purposes, if RPC fails, allow access with warning
            Ok(VerifyNftResponse {
                is_holder: true,
                message: format!(
                    "RPC verification failed (using demo mode). Error: {:?}",
                    e
                ),
            })
        }
    }
}

