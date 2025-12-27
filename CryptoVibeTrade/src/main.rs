#![forbid(unsafe_code)]

use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;

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
// Leptos UI Components
// =====================

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    // Simple auth state - in production, use proper session management
    let (is_logged_in, set_logged_in) = create_signal(false);

    view! {
        <Title text="Crypto Vibe Trade - Fenrir Admin"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Meta charset="UTF-8"/>

        <Style id="cvt-css">{r#"
            :root {
                --neon-red: #ff3333;
                --neon-orange: #ff6b35;
                --neon-gray: #4a4a4a;
                --bg-black: #000000;
                --bg-dark: #0a0a0a;
                --border-dim: #1a1a1a;
                --text-primary: #ffffff;
                --text-secondary: #888888;
            }

            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            body {
                margin: 0;
                font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
                background: var(--bg-black);
                color: var(--text-primary);
                overflow: hidden;
                height: 100vh;
            }

            .container {
                max-width: 1400px;
                margin: 0 auto;
                padding: 0 32px;
            }

            /* Navbar */
            .nav {
                position: sticky;
                top: 0;
                background: rgba(0, 0, 0, 0.95);
                backdrop-filter: blur(20px);
                border-bottom: 1px solid var(--border-dim);
                z-index: 1000;
            }

            .bar {
                font-size: 9px;
                color: var(--text-secondary);
                padding: 8px 0;
                border-bottom: 1px solid var(--border-dim);
                letter-spacing: 0.3em;
                text-transform: uppercase;
            }

            .brand {
                display: flex;
                gap: 16px;
                align-items: center;
                padding: 16px 0;
            }

            .logo {
                width: 44px;
                height: 44px;
                border-radius: 12px;
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                display: grid;
                place-items: center;
                color: #000;
                font-weight: 900;
                font-size: 14px;
                box-shadow: 0 0 20px rgba(255, 51, 51, 0.3);
                animation: pulse 3s ease-in-out infinite;
            }

            @keyframes pulse {
                0%, 100% { box-shadow: 0 0 20px rgba(255, 51, 51, 0.3); }
                50% { box-shadow: 0 0 30px rgba(255, 107, 53, 0.5); }
            }

            .brand-text {
                font-weight: 800;
                font-size: 18px;
                letter-spacing: -0.02em;
                background: linear-gradient(135deg, var(--text-primary), var(--neon-orange));
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
            }

            .brand-sub {
                font-size: 9px;
                letter-spacing: 0.35em;
                text-transform: uppercase;
                color: var(--text-secondary);
                margin-top: 2px;
            }

            /* Buttons */
            .btn {
                border: 1px solid var(--neon-gray);
                background: transparent;
                color: var(--text-primary);
                padding: 12px 20px;
                border-radius: 10px;
                cursor: pointer;
                font-family: inherit;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                position: relative;
                overflow: hidden;
            }

            .btn:hover {
                border-color: var(--neon-orange);
                color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
                transform: translateY(-2px);
            }

            .btn-primary {
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                border: none;
                color: #000;
            }

            .btn-primary:hover {
                box-shadow: 0 0 30px rgba(255, 51, 51, 0.5);
                color: #000;
            }

            /* Login Page */
            .login-container {
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: linear-gradient(135deg, #000 0%, #0a0a0a 100%);
            }

            .login-box {
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.8), rgba(10, 10, 10, 0.9));
                border: 1px solid var(--border-dim);
                border-radius: 20px;
                padding: 48px;
                min-width: 400px;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
            }

            .login-title {
                font-size: 32px;
                font-weight: 800;
                margin-bottom: 8px;
                text-align: center;
            }

            .login-subtitle {
                color: var(--text-secondary);
                text-align: center;
                margin-bottom: 32px;
                font-size: 12px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
            }

            .form-group {
                margin-bottom: 20px;
            }

            .form-group label {
                display: block;
                margin-bottom: 8px;
                font-size: 11px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
                color: var(--text-secondary);
            }

            .form-group input {
                width: 100%;
                padding: 14px 18px;
                border-radius: 10px;
                border: 1px solid var(--border-dim);
                background: rgba(0, 0, 0, 0.8);
                color: var(--text-primary);
                font-family: inherit;
                font-size: 14px;
                transition: all 0.3s;
            }

            .form-group input:focus {
                outline: none;
                border-color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.2);
            }

            .error-msg {
                color: var(--neon-red);
                font-size: 12px;
                margin-top: 12px;
                text-align: center;
            }

            /* Dashboard */
            .dashboard {
                height: calc(100vh - 120px);
                display: flex;
                flex-direction: column;
                padding: 20px 0;
            }

            .iframes-container {
                flex: 1;
                display: flex;
                flex-direction: column;
                gap: 8px;
                height: 100%;
            }

            .iframe-row {
                flex: 1;
                display: flex;
                gap: 8px;
                min-height: 0;
            }

            .iframe-wrapper {
                flex: 1;
                border: 1px solid var(--border-dim);
                border-radius: 12px;
                overflow: hidden;
                background: #000;
                position: relative;
                min-width: 0;
            }

            .iframe-wrapper iframe {
                width: 100%;
                height: 100%;
                border: none;
            }

            .iframe-label {
                position: absolute;
                top: 8px;
                left: 8px;
                background: rgba(0, 0, 0, 0.8);
                padding: 4px 12px;
                border-radius: 6px;
                font-size: 10px;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                color: var(--neon-orange);
                border: 1px solid var(--neon-orange);
                z-index: 10;
            }

            /* Context Menu */
            .context-menu {
                position: fixed;
                background: rgba(20, 20, 20, 0.98);
                border: 1px solid var(--neon-orange);
                border-radius: 12px;
                padding: 8px 0;
                min-width: 250px;
                z-index: 10000;
                box-shadow: 0 10px 40px rgba(255, 107, 53, 0.3);
                backdrop-filter: blur(10px);
            }

            .context-menu-item {
                padding: 12px 16px;
                cursor: pointer;
                display: flex;
                align-items: center;
                gap: 12px;
                font-size: 12px;
                transition: all 0.2s;
            }

            .context-menu-item:hover {
                background: rgba(255, 107, 53, 0.1);
            }

            .context-menu-divider {
                height: 1px;
                background: var(--border-dim);
                margin: 8px 0;
            }

            .grok-wtf {
                color: var(--neon-red);
                font-weight: 800;
                letter-spacing: 0.1em;
            }

            /* Grok Modal */
            .grok-modal {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.9);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10001;
            }

            .grok-modal-content {
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.95), rgba(10, 10, 10, 0.98));
                border: 1px solid var(--border-dim);
                border-radius: 20px;
                padding: 32px;
                max-width: 600px;
                width: 90%;
                max-height: 80vh;
                overflow-y: auto;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
            }

            .grok-modal-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 24px;
            }

            .grok-modal-title {
                font-size: 20px;
                font-weight: 800;
                color: var(--neon-orange);
            }

            .grok-close {
                background: none;
                border: none;
                color: var(--text-secondary);
                font-size: 24px;
                cursor: pointer;
                padding: 0;
                width: 32px;
                height: 32px;
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .grok-close:hover {
                color: var(--text-primary);
            }

            .grok-response {
                white-space: pre-wrap;
                line-height: 1.6;
                font-size: 14px;
            }

            .hidden {
                display: none !important;
            }
        "#}</Style>

        <div class="nav">
            <div class="bar">
                <div class="container">"Fenrir Admin Terminal • Protected by Wolfram"</div>
            </div>
            <div class="container brand">
                <div class="logo">"CVT"</div>
                <div>
                    <div class="brand-text">"Crypto Vibe Trade"</div>
                    <div class="brand-sub">"Admin Dashboard"</div>
                </div>
            </div>
        </div>

        <div class:hidden=move || !is_logged_in.get()>
            <div class="container">
                <Dashboard set_logged_in=set_logged_in.clone()/>
            </div>
        </div>

        <div class:hidden=move || is_logged_in.get()>
            <LoginPage set_logged_in=set_logged_in.clone()/>
        </div>
    }
}

#[component]
fn LoginPage(
    set_logged_in: WriteSignal<bool>
) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);

    let handle_login = create_action(move |_: &()| {
        let username = username.get();
        let password = password.get();
        set_is_loading.set(true);
        set_error.set(String::new());

        async move {
            let result = login(username, password).await;
            set_is_loading.set(false);

            match result {
                Ok(response) => {
                    if response.success {
                        set_logged_in.set(true);
                        Ok(())
                    } else {
                        set_error.set(response.message);
                        Err(ServerFnError::new("Login failed"))
                    }
                }
                Err(e) => {
                    set_error.set(format!("Error: {}", e));
                    Err(e)
                }
            }
        }
    });

    view! {
        <div class="login-container">
            <div class="login-box">
                <div class="login-title">"🐺 Fenrir Admin"</div>
                <div class="login-subtitle">"Authentication Required"</div>

                <div class="form-group">
                    <label>"Username"</label>
                    <input
                        type="text"
                        prop:value=username
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        placeholder="Enter username"
                    />
                </div>

                <div class="form-group">
                    <label>"Password"</label>
                    <input
                        type="password"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        placeholder="Enter password"
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                handle_login.dispatch(());
                            }
                        }
                    />
                </div>

                {move || if !error.get().is_empty() {
                    view! {
                        <div class="error-msg">{error.get()}</div>
                    }
                } else {
                    view! { <div/> }
                }}

                <button
                    class="btn btn-primary"
                    style="width:100%; margin-top:16px;"
                    on:click=move |_| handle_login.dispatch(())
                    disabled=is_loading
                >
                    {move || if is_loading.get() { "Authenticating..." } else { "Login" }}
                </button>
            </div>
        </div>
    }
}

#[component]
fn Dashboard(
    set_logged_in: WriteSignal<bool>
) -> impl IntoView {
    view! {
        <div class="dashboard">
            <div class="iframes-container">
                // Row 1: Orca (full width)
                <div class="iframe-row" style="flex:0.6;">
                    <div class="iframe-wrapper">
                        <div class="iframe-label">"ORCA - Whirlpools"</div>
                        <iframe src="https://orca.so/pools"></iframe>
                    </div>
                </div>

                // Row 2: Raydium and Meteora (split)
                <div class="iframe-row" style="flex:0.4;">
                    <div class="iframe-wrapper">
                        <div class="iframe-label">"RAYDIUM - Liquidity"</div>
                        <iframe src="https://raydium.io/liquidity-pools/"></iframe>
                    </div>
                    <div class="iframe-wrapper">
                        <div class="iframe-label">"METEORA - DEX"</div>
                        <iframe src="https://meteora.ag/?tab=top"></iframe>
                    </div>
                </div>
            </div>

            <div style="margin-top:16px; display:flex; justify-content:center;">
                <button class="btn" on:click=move |_| set_logged_in.set(false)>"Logout"</button>
            </div>
        </div>
    }
}

// Global state for selected text (simplified for demo)
#[server]
pub async fn get_selected_text() -> Result<String, ServerFnError> {
    Ok(String::new())
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
