#![forbid(unsafe_code)]

use leptos::prelude::*;
use leptos_meta::*;

use serde::{Deserialize, Serialize};

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

// =====================\n// Server Functions (Leptos)\n// =====================\n\n#[server(ConnectWallet, \"/api\")]\npub async fn connect_wallet() -> Result&lt;String, ServerFnError&gt; {\n    Ok(\"9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM\".to_string())\n}\n\n/// Calls the AI provider from the server.\n///\n/// Route will be under `/api/*fn_name` when SSR is enabled.\n#[server(AiAnalyze, \"/api\")]
pub async fn ai_analyze(prompt: String, asset: String) -> Result<String, ServerFnError> {
    use std::env;

    // IMPORTANT: Do not hardcode keys. Use env vars.
    let api_key = env::var("XAI_API_KEY")
        .map_err(|_| ServerFnError::ServerError("Missing XAI_API_KEY env var".to_string()))?;

    let base_url = env::var("XAI_BASE_URL").unwrap_or_else(|_| "https://api.x.ai/v1".to_string());
    let model = env::var("XAI_MODEL").unwrap_or_else(|_| "grok-4".to_string());

    let system = format!(
        "Você é um Analista Técnico Sênior (Persona: Fenrir AI). Ativo: {asset}. Seja direto e termine com DYOR."
    );

    // OpenAI-compatible Chat Completions payload.
    #[derive(Serialize)]
    struct ChatReq<'a> {
        model: &'a str,
        messages: Vec<Message<'a>>,
        temperature: f32,
    }

    #[derive(Serialize)]
    struct Message<'a> {
        role: &'a str,
        content: String,
    }

    let body = ChatReq {
        model: &model,
        messages: vec![
            Message {
                role: "system",
                content: system,
            },
            Message {
                role: "user",
                content: format!("Ativo: {asset}\n\nPergunta do usuário: {}", prompt.trim()),
            },
        ],
        temperature: 0.4,
    };

    let url = format!("{base_url}/chat/completions");

    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("network error: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::ServerError(format!(
            "provider error: {status} {err}"
        )));
    }

    // Parse minimal compatible response: choices[0].message.content
    #[derive(Deserialize)]
    struct ChatResp {
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
        .json::<ChatResp>()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("decode error: {e}")))?;

    Ok(parsed
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "Resposta vazia do provedor. DYOR.".to_string()))
}

// =====================
// Leptos UI (Rust)
// =====================

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        <Html lang="pt-BR"/>
        <Title text="Crypto Vibe Trade"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        // Minimal CSS inline (full Rust, no Tailwind CDN)
        <Style id="cvt-css">{r#"
            :root {
                color-scheme: dark;
                --darkBg: #050505;
                --panelBg: #121212;
                --solanaGreen: #14F195;
                --solanaPurple: #9945FF;
                --bitcoinOrange: #F7931A;
                --zcashGold: #F5B700;
                --rustOrange: #DEA584;
                --panelGray: #1a1a1a;
                --borderGray: #1f1f1f;
                --textGray: #9ca3af;
                --textMuted: #6b7280;
            }
            * { box-sizing: border-box; }
            body {
                margin: 0;
                padding: 0;
                font-family: 'SF Mono', ui-monospace, Menlo, Monaco, Consolas, monospace;
                background: var(--darkBg);
                color: #e5e7eb;
                overflow-x: hidden;
                line-height: 1.6;
            }
            a { color: inherit; text-decoration: none; }
            .container { max-width: 1280px; margin: 0 auto; padding: 0 1.5rem; }
            .nav { position: sticky; top: 0; z-index: 50; background: rgba(5,5,5,0.95); backdrop-filter: blur(20px); border-bottom: 1px solid var(--borderGray); }
            .bar { 
                font-size: 0.625rem; color: var(--textMuted); padding: 0.75rem 1rem; 
                display: flex; justify-content: space-between; align-items: center; 
                font-family: var(--mono);
            }
            .brand { display: flex; gap: 1rem; align-items: center; padding: 1rem 1.5rem; }
            .logo { 
                width: 2.5rem; height: 2.5rem; border-radius: 0.625rem; 
                background: linear-gradient(135deg, var(--solanaPurple), var(--solanaGreen)); 
                display: grid; place-items: center; font-weight: 900; color: #000; 
                box-shadow: 0 0 1rem rgba(20,241,149,0.5); animation: pulse-slow 3s infinite;
            }
            .hero { padding: 5rem 0 3rem; text-align: center; }
            h1 { 
                font-size: clamp(3rem, 8vw, 6rem); font-weight: 900; margin: 0 0 1.5rem; 
                letter-spacing: -0.05em; background: linear-gradient(135deg, white, var(--solanaGreen)); 
                -webkit-background-clip: text; background-clip: text; color: transparent; 
                filter: drop-shadow(0 0 1rem rgba(255,255,255,0.2)); animation: glitch 2s infinite;
            }
            @keyframes glitch {
                0%, 100% { transform: translate(0); }
                10% { transform: translate(-2px, 2px); }
                20% { transform: translate(2px, -2px); }
                30% { transform: translate(-1px, 1px); }
            }
            .sub { color: var(--textGray); max-width: 48rem; margin: 0 auto 3rem; font-size: 1.25rem; opacity: 0.9; }
            .grid { display: grid; gap: 1.5rem; }
            @media (min-width: 768px) { .grid-cols-3 { grid-template-columns: repeat(3, 1fr); } }
            .asset-card { 
                border: 1px solid var(--borderGray); background: var(--panelBg); border-radius: 1rem; 
                padding: 1.5rem; cursor: pointer; transition: all 0.3s cubic-bezier(0.4,0,0.2,1); 
                position: relative; overflow: hidden;
            }
            .asset-card:hover { 
                transform: translateY(-0.5rem) rotateX(5deg); box-shadow: 0 1.5rem 4rem -1rem rgba(153,69,255,0.3); 
                border-color: var(--solanaPurple);
            }
            .asset-card.active { border-color: var(--solanaGreen); box-shadow: 0 0 2rem rgba(20,241,149,0.3); }
            .mono { font-family: inherit; }
            .section { padding: 3rem 0; scroll-margin-top: 4rem; }
            .section h2 { font-size: 2rem; font-weight: 800; margin: 0 0 0.5rem; letter-spacing: -0.025em; }
            .muted { color: var(--textMuted); }
            .panel { border: 1px solid var(--borderGray); border-radius: 1rem; background: var(--panelBg); overflow: hidden; box-shadow: 0 1rem 3rem rgba(0,0,0,0.5); }
            .panel-head { padding: 0.75rem 1rem; border-bottom: 1px solid var(--borderGray); display: flex; justify-content: space-between; align-items: center; background: rgba(0,0,0,0.4); font-size: 0.75rem; }
            .chart-container { position: relative; height: 20rem; background: radial-gradient(ellipse at center, rgba(20,241,149,0.03) 0%, transparent 70%); }
            .chart-scanline::after { 
                content: ''; position: absolute; inset: 0; background: linear-gradient(transparent 49%, rgba(20,241,149,0.05) 50%, transparent 51%); 
                background-size: 100% 3px; pointer-events: none; animation: scan 4s linear infinite; 
            }
            @keyframes scan { 0% { transform: translateY(-100%); } 100% { transform: translateY(100%); } }
            .btn { 
                border: 1px solid rgba(153,69,255,0.4); background: rgba(153,69,255,0.1); color: var(--solanaPurple); 
                padding: 0.75rem 1.25rem; border-radius: 0.75rem; font-size: 0.875rem; cursor: pointer; transition: all 0.3s; 
                backdrop-filter: blur(10px);
            }
            .btn:hover { background: rgba(153,69,255,0.2); box-shadow: 0 0 1.5rem rgba(153,69,255,0.3); transform: translateY(-1px); }
            .chat { display: grid; grid-template-rows: 1fr auto; height: 32rem; }
            @media (min-width: 1024px) { .chat { grid-template-columns: 18rem 1fr; } }
            .sidebar { border-right: 1px solid var(--borderGray); padding: 1.5rem; background: rgba(0,0,0,0.3); overflow-y: auto; }
            .chip { 
                display: block; width: 100%; text-align: left; padding: 1rem; border-radius: 0.75rem; border: 1px solid var(--borderGray); 
                background: rgba(15,15,15,0.8); color: #d1d5db; cursor: pointer; transition: all 0.2s; margin-bottom: 0.75rem;
            }
            .chip:hover { border-color: var(--solanaGreen); box-shadow: 0 0 1rem rgba(20,241,149,0.2); transform: translateX(4px); }
            .chatmain { display: flex; flex-direction: column; background: rgba(5,5,5,0.8); }
            .msgs { flex: 1; overflow-y: auto; padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem; }
            .msg { max-width: 80%; padding: 1rem 1.25rem; border-radius: 1.25rem; border: 1px solid var(--borderGray); background: #111827; white-space: pre-wrap; }
            .msg.user { align-self: flex-end; background: rgba(153,69,255,0.15); border-color: rgba(153,69,255,0.4); }
            .inputbar { padding: 1.5rem; border-top: 1px solid var(--borderGray); display: flex; gap: 1rem; background: var(--panelBg); }
            input { flex: 1; padding: 1rem; border-radius: 0.75rem; border: 1px solid var(--borderGray); background: #000; color: #fff; font-family: inherit; font-size: 0.875rem; }
            input:focus { outline: none; border-color: var(--solanaPurple); box-shadow: 0 0 1rem rgba(153,69,255,0.2); }
            footer { border-top: 1px solid var(--borderGray); padding: 4rem 0; color: var(--textMuted); text-align: center; position: relative; }
            footer::before { content: ''; position: absolute; top: 0; left: 0; right: 0; height: 1px; background: linear-gradient(to right, transparent, var(--solanaGreen), transparent); }
            /* Custom Scrollbar Neon */
            ::-webkit-scrollbar { width: 8px; height: 8px; }
            ::-webkit-scrollbar-track { background: var(--panelGray); }
            ::-webkit-scrollbar-thumb { background: #333; border-radius: 4px; }
            ::-webkit-scrollbar-thumb:hover { background: var(--solanaPurple); box-shadow: 0 0 8px var(--solanaPurple); }
            /* Animations */
            @keyframes pulse-slow { 0%, 100% { opacity: 1; } 50% { opacity: 0.7; } }
            @keyframes float { 0%, 100% { transform: translateY(0); } 50% { transform: translateY(-0.5rem); } }
            @keyframes typing { 0%, 80%, 100% { transform: scale(0); opacity: 0.5; } 40% { transform: scale(1); opacity: 1; } }
            .typing-dot { display: inline-block; width: 0.5rem; height: 0.5rem; border-radius: 50%; background: var(--solanaGreen); margin: 0 0.125rem; animation: typing 1.4s infinite ease-in-out; }
            .typing-dot:nth-child(1) { animation-delay: -0.32s; }
            .typing-dot:nth-child(2) { animation-delay: -0.16s; }
            .animate-float { animation: float 6s ease-in-out infinite; }
            /* Matrix Rain Subtle BG */
            body::before { content: ''; position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: -1; background: radial-gradient(circle at 20% 80%, rgba(120,119,198,0.3) 0%, transparent 50%), radial-gradient(circle at 80% 20%, rgba(255,119,198,0.3) 0%, transparent 50%), radial-gradient(circle at 40% 40%, rgba(120,219,255,0.3) 0%, transparent 50%); opacity: 0.1; animation: matrix-rain 20s linear infinite; }
            @keyframes matrix-rain { 0% { background-position: 0 0, 100% 100%, 50% 50%; } 100% { background-position: 100px 100px, -100px -100px, -50px 150px; } }
        "#}</Style>

        <div class="nav">
            <div class="bar">
                <div class="container">"Powered by Grok (server) • Protected by Fenrir"</div>
            </div>
            <div class="container brand">
                <div class="logo">"CVT"</div>
                <div>
                    <div style="font-weight:800">"Crypto Vibe Trade"</div>
                    <div class="mono muted" style="font-size:10px; letter-spacing:.22em; text-transform:uppercase">"Terminal V.9.0"</div>
                </div>
            </div>
        </div>

        <main class="container">
            <Hero/>
            <Ichimoku/>
            <AiTerminal/>
        </main>

        <footer>
            <div class="container">
                <div style="font-weight:900; letter-spacing:.2em; color:#9ca3af">"NAO DAMOS DICAS DE INVESTIMENTO."</div>
                <div style="margin-top:10px">"DYOR — Do Your Own Research."</div>
            </div>
        </footer>
    }
}

#[component]
fn Hero(cx: Scope) -> impl IntoView {
    let (asset, set_asset) = create_signal::<String>("BTC".to_string());

    let mk_card = move |code: &'static str, title: &'static str, subtitle: &'static str| {
        let code_s = code.to_string();
        view! {
                <div class="asset-card"
                    class=move || {
                        let a = asset.get();
                        if a == code_s { "active border-solanaGreen shadow-solana-glow" } else { "" }
                    }
                    on:click=move |_| set_asset.set(code.to_string())
                >
                <div style="display:flex; justify-content:space-between; align-items:flex-start; gap:12px;">
                    <div>
                        <div style="font-weight:800; font-size:18px;">{title}</div>
                        <div class="mono muted" style="font-size:10px; letter-spacing:.18em; text-transform:uppercase">{subtitle}</div>
                    </div>
                    <div class="mono" style="font-size:12px; padding:6px 8px; border-radius:10px; border:1px solid #1f2937; color:#d1d5db;">{code}</div>
                </div>
                <div class="mono" style="margin-top:14px; font-size:22px; font-weight:800;">"$ —"</div>
                <div class="mono muted" style="margin-top:6px; font-size:11px;">"(preço real entra depois)"</div>
            </div>
        }
    };

    view! {
        <section class="hero">
            <h1 class="animate-float glitch-hero text-gradient"> "Crypto Vibe Trade" </h1>
            <p class="sub">
                "Visualize tendências invisíveis com Ichimoku Cloud, Bollinger Bands e Stochastic RSI. "
                "Apoiado por Inteligência Artificial para Bitcoin, Solana e ZCash."
            </p>

            <div class="section" id="assets">
                <div class="grid grid-3">
                    {mk_card("BTC", "Bitcoin", "CRYPTOCURRENCY")}
                    {mk_card("SOL", "Solana", "SMART CONTRACT")}
                    {mk_card("ZEC", "ZCash", "PRIVACY COIN")}
                </div>
            </div>
        </section>
    }
}

#[component]
fn Ichimoku(cx: Scope) -> impl IntoView {
    view! {
        <section class="section" id="ichimoku">
            <div style="display:flex; justify-content:space-between; align-items:center; gap:12px; flex-wrap:wrap;">
                <div>
                    <h2>"Nuvem de Ichimoku"</h2>
                    <div class="mono muted" style="font-size:11px;">"INDICADOR DE TENDÊNCIA E SUPORTE"</div>
                </div>
                <a href="#ai-analyst" class="btn">"Analisar com IA"</a>
            </div>

            <div class="panel chart-container chart-scanline" style="margin-top:14px;">
                <div class="panel-head">
                    <span class="mono font-bold text-white bg-panelGray px-2 py-1 rounded text-xs border border-panelGray">"BTC/USD"</span>
                    <span class="mono text-xs text-muted">"1D TIMEFRAME"</span>
                </div>
                <div class="h-80 bg-gradient-to-b from-darkBg to-panelBg flex items-center justify-center text-muted text-mono">
                    <div>"📊 Ichimoku Cloud Chart Placeholder (Rust SVG next)"</div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AiTerminal(cx: Scope) -> impl IntoView {
    let (asset, set_asset) = create_signal::<String>("BTC".to_string());
    let (input, set_input) = create_signal::<String>("".to_string());
    let (messages, set_messages) = create_signal::<Vec<(bool, String)>>(vec![(
        false,
        "Saudações. Calibrado para BTC/SOL/ZEC. O que queres analisar hoje?".to_string(),
    )]);

    // Action that calls the Leptos server function.
    let send = create_action(move |q: &String| {
        let q = q.clone();
        let asset = asset.get();
        async move { ai_analyze(q, asset).await }
    });

    // When action resolves, push the assistant message.
    create_effect(cx, move |_| {
        if let Some(result) = send.value().get() {
            match result {
                Ok(text) => set_messages.update(|m| m.push((false, text))),
                Err(e) => {
                    set_messages.update(|m| m.push((false, format!("Erro do backend: {e}. DYOR."))))
                }
            }
        }
    });

    let on_submit = move |_| {
        let q = input.get().trim().to_string();
        if q.is_empty() {
            return;
        }

        set_input.set("".to_string());
        set_messages.update(|m| m.push((true, q.clone())));
        send.dispatch(q);
    };

    view! {
        <section class="section" id="ai-analyst">
            <div>
                <h2>"Terminal IA Dedicado"</h2>
                <div class="mono muted" style="font-size:11px;">"GROK (SERVER) • CONTEXTO: MERCADO CRYPTO"</div>
            </div>

            <div class="panel" style="margin-top:14px;">
                <div class="chat">
                    <div class="sidebar">
                        <div class="mono muted" style="font-size:10px; letter-spacing:.22em; text-transform:uppercase; margin-bottom:12px;">"Ativo"</div>
                        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
                            <button class="btn" on:click=move |_| set_asset.set("BTC".to_string())>"BTC"</button>
                            <button class="btn" on:click=move |_| set_asset.set("SOL".to_string())>"SOL"</button>
                            <button class="btn" on:click=move |_| set_asset.set("ZEC".to_string())>"ZEC"</button>
                        </div>
                    </div>

                    <div class="chatmain">
                        <div class="msgs">
                            <For
                                each=move || messages.get()
                                key=|(_, s)| s.clone()
                                children=move |(is_user, text)| {
                                    view! {
                                        <div class=move || if is_user { "msg user" } else { "msg" }>
                                            {text}
                                        </div>
                                    }
                                }
                            />
                        </div>

                        <div class="inputbar">
                            <input
                                prop:value=move || input.get()
                                on:input=move |ev| set_input.set(event_target_value(&ev))
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        on_submit(());
                                    }
                                }
                                placeholder="Pergunte ao Oráculo (ex: cenário BTC / Ichimoku / RSI...)"
                            />
                            <button class="btn" on:click=on_submit>"Enviar"</button>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

// =====================
// Entrypoint (SSR server)
// =====================

#[tokio::main]
async fn main() {
    use axum::{Router, routing::post};
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use std::net::SocketAddr;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conf = leptos::get_configuration(None).await.expect("Failed to establish configuration");\n    let app_fn = |cx: Scope| view! { cx, <App/> };\n    let routes = generate_route_list(app_fn.clone());

    // This route powers Leptos server functions (including ai_analyze).
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::render_app_to_stream(&amp;conf, app_fn))
        .with_state(conf);

    let addr = std::env::var("CVT_ADDR")
        .ok()
        .and_then(|s| s.parse::<SocketAddr>().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));

    tracing::info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server failed");
}
