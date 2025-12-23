#![forbid(unsafe_code)]

use leptos::prelude::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;

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

// =====================
// Server Function (Leptos)
// =====================

#[server(AiAnalyze, "/api")]
pub async fn ai_analyze(
    prompt: String,
    asset: String,
) -> Result<String, ServerFnError<NoCustomError>> {
    use std::env;

    let api_key = env::var("XAI_API_KEY")
        .map_err(|_| ServerFnError::ServerError("Missing XAI_API_KEY env var".to_string()))?;

    let base_url = env::var("XAI_BASE_URL").unwrap_or_else(|_| "https://api.x.ai/v1".to_string());
    let model = env::var("XAI_MODEL").unwrap_or_else(|_| "grok-4".to_string());

    let system = format!(
        "Você é um Analista Técnico Sênior (Persona: Fenrir AI). Ativo: {asset}. Seja direto e termine com DYOR."
    );

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
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Crypto Vibe Trade"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Style id="cvt-css">{r#"
            :root{color-scheme:dark;}
            body{margin:0;font-family:ui-sans-serif,system-ui,-apple-system;background:#050505;color:#e5e7eb}
            .container{max-width:1100px;margin:0 auto;padding:0 24px}
            .nav{position:sticky;top:0;background:rgba(5,5,5,.92);backdrop-filter:blur(12px);border-bottom:1px solid #1f2937;z-index:10}
            .bar{font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,monospace;font-size:10px;color:#6b7280;padding:6px 0;border-bottom:1px solid #111827}
            .brand{display:flex;gap:12px;align-items:center;padding:14px 0}
            .logo{width:40px;height:40px;border-radius:10px;background:linear-gradient(135deg,#9945FF,#14F195);display:grid;place-items:center;color:#000;font-weight:900}
            .hero{padding:72px 0 40px}
            h1{font-size:56px;letter-spacing:-.04em;margin:0 0 18px}
            .sub{color:#9ca3af;max-width:760px;line-height:1.6}
            .grid{display:grid;gap:16px}
            @media(min-width:800px){.grid-3{grid-template-columns:repeat(3,1fr)}}
            .asset-card{border:1px solid #1f2937;background:#121212;border-radius:14px;padding:16px;cursor:pointer}
            .asset-card.active{outline:1px solid #14F195;border-color:rgba(20,241,149,.35);background:#0b0b0b}
            .mono{font-family:ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,monospace}
            .section{padding:42px 0}
            .section h2{margin:0 0 8px;font-size:26px;letter-spacing:-.02em}
            .muted{color:#6b7280}
            .panel{border:1px solid #1f2937;border-radius:14px;background:#0b0b0b;overflow:hidden}
            .panel-head{padding:12px 14px;border-bottom:1px solid #1f2937;display:flex;justify-content:space-between;align-items:center;background:rgba(0,0,0,.45)}
            .btn{border:1px solid rgba(153,69,255,.35);background:rgba(153,69,255,.08);color:#c4b5fd;padding:10px 14px;border-radius:10px;cursor:pointer}
            .btn:hover{background:rgba(153,69,255,.18)}
            .chat{display:grid}
            @media(min-width:900px){.chat{grid-template-columns:300px 1fr}}
            .sidebar{border-right:1px solid #1f2937;padding:14px;background:rgba(0,0,0,.35)}
            .chatmain{min-height:540px;display:flex;flex-direction:column}
            .msgs{flex:1;overflow:auto;padding:14px;display:flex;flex-direction:column;gap:12px}
            .msg{max-width:820px;padding:12px 14px;border-radius:14px;border:1px solid #1f2937;background:#111827;white-space:pre-wrap}
            .msg.user{align-self:flex-end;background:rgba(153,69,255,.10);border-color:rgba(153,69,255,.25)}
            .inputbar{padding:14px;border-top:1px solid #1f2937;display:flex;gap:10px}
            input{flex:1;padding:12px;border-radius:10px;border:1px solid #374151;background:#000;color:#fff}
            input:focus{outline:none;border-color:#6b7280}
            footer{border-top:1px solid #111827;padding:44px 0;color:#6b7280;text-align:center}
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
fn Hero() -> impl IntoView {
    let (asset, set_asset) = create_signal("BTC".to_string());

    let mk_card = move |code: &'static str, title: &'static str, subtitle: &'static str| {
        let code_a = code.to_string();
        let code_b = code.to_string();
        let code_c = code.to_string();

        let class_name = move || {
            if asset.get() == code_a {
                "asset-card active".to_string()
            } else {
                "asset-card".to_string()
            }
        };

        view! {
            <div class=class_name on:click=move |_| set_asset.set(code_b.clone())>
                <div style="display:flex; justify-content:space-between; align-items:flex-start; gap:12px;">
                    <div>
                        <div style="font-weight:800; font-size:18px;">{title}</div>
                        <div class="mono muted" style="font-size:10px; letter-spacing:.18em; text-transform:uppercase">{subtitle}</div>
                    </div>
                    <div class="mono" style="font-size:12px; padding:6px 8px; border-radius:10px; border:1px solid #1f2937; color:#d1d5db;">{code_c}</div>
                </div>
                <div class="mono" style="margin-top:14px; font-size:22px; font-weight:800;">"$ —"</div>
                <div class="mono muted" style="margin-top:6px; font-size:11px;">"(preço real entra depois)"</div>
            </div>
        }
    };

    view! {
        <section class="hero">
            <h1>"Crypto Vibe Trade"</h1>
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
fn Ichimoku() -> impl IntoView {
    view! {
        <section class="section" id="ichimoku">
            <div style="display:flex; justify-content:space-between; align-items:center; gap:12px; flex-wrap:wrap;">
                <div>
                    <h2>"Nuvem de Ichimoku"</h2>
                    <div class="mono muted" style="font-size:11px;">"INDICADOR DE TENDÊNCIA E SUPORTE"</div>
                </div>
                <a href="#ai-analyst" class="btn">"Analisar com IA"</a>
            </div>

            <div class="panel" style="margin-top:14px;">
                <div class="panel-head">
                    <div class="mono" style="font-size:11px; color:#d1d5db;">"(gráfico vem na próxima etapa)"</div>
                    <div class="mono muted" style="font-size:10px;">"1D TIMEFRAME"</div>
                </div>
                <div style="padding:16px;" class="muted">
                    "Próximo passo: chart em Rust (SVG/canvas) ou bindings com Chart.js controlado por WASM."
                </div>
            </div>
        </section>
    }
}

#[component]
fn AiTerminal() -> impl IntoView {
    use std::rc::Rc;

    let (asset, set_asset) = create_signal("BTC".to_string());
    let (input, set_input) = create_signal(String::new());
    let (messages, set_messages) = create_signal::<Vec<(bool, String)>>(vec![(
        false,
        "Saudações. Calibrado para BTC/SOL/ZEC. O que queres analisar hoje?".to_string(),
    )]);

    let send = create_action(move |q: &String| {
        let q = q.clone();
        let asset = asset.get();
        async move { ai_analyze(q, asset).await }
    });

    create_effect(move |_| {
        if let Some(result) = send.value().get() {
            match result {
                Ok(text) => set_messages.update(|m| m.push((false, text))),
                Err(e) => set_messages.update(|m| m.push((false, format!("Erro do backend: {e}. DYOR.")))),
            }
        }
    });

    let do_submit: Rc<dyn Fn()> = Rc::new(move || {
        let q = input.get().trim().to_string();
        if q.is_empty() {
            return;
        }
        set_input.set(String::new());
        set_messages.update(|m| m.push((true, q.clone())));
        send.dispatch(q);
    });

    let submit_click = {
        let do_submit = do_submit.clone();
        move |_| (do_submit)()
    };

    let submit_enter = {
        let do_submit = do_submit.clone();
        move |ev: leptos::ev::KeyboardEvent| {
            if ev.key() == "Enter" {
                (do_submit)();
            }
        }
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
                                on:keydown=submit_enter
                                placeholder="Pergunte ao Oráculo (ex: cenário BTC / Ichimoku / RSI...)"
                            />
                            <button class="btn" on:click=submit_click>"Enviar"</button>
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
    use axum::{routing::post, Router};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::net::SocketAddr;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let leptos_options = leptos::config::LeptosOptions::default();
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler)
        .with_state(leptos_options);

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