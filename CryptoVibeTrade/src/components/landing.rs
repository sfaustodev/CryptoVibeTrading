use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::AiAnalyze;

#[component]
pub fn LandingPage() -> impl IntoView {
    let (current_asset, set_current_asset) = create_signal("BTC".to_string());
    let (ai_analysis, set_ai_analysis) = create_signal("".to_string());
    let (is_analyzing, set_is_analyzing) = create_signal(false);
    let (error_message, set_error_message) = create_signal(String::new());

    // TradingView widget integration
    let tradingview_widget = move || {
        let symbol = match current_asset.get().as_str() {
            "BTC" => "BINANCE:BTCUSDT",
            "SOL" => "BINANCE:SOLUSDT",
            "ZEC" => "BINANCE:ZECUSDT",
            _ => "BINANCE:BTCUSDT",
        };

        format!(r#"
            <div class="tradingview-widget-container" style="height:500px;width:100%">
                <div class="tradingview-widget-container__widget"></div>
                <script type="text/javascript" src="https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js" async>
                {{
                    "autosize": true,
                    "symbol": "{}",
                    "interval": "D",
                    "timezone": "Etc/UTC",
                    "theme": "dark",
                    "style": "1",
                    "locale": "en",
                    "enable_publishing": false,
                    "hide_side_toolbar": false,
                    "allow_symbol_change": true,
                    "calendar": false,
                    "support_host": "https://www.tradingview.com"
                }}
                <\/script>
            </div>
        "#, symbol)
    };

    let analyze_indicators = move |_| {
        set_is_analyzing.set(true);
        set_error_message.set(String::new());
        set_ai_analysis.set(String::new());

        let asset = current_asset.get();
        let indicators = "Ichimoku Cloud, RSI, MACD, Bollinger Bands, Volume".to_string();

        spawn_local(async move {
            match AiAnalyze(
                "Analyze the current market conditions and provide technical analysis.".to_string(),
                asset.clone(),
                indicators.clone(),
            ).await {
                Ok(response) => {
                    set_ai_analysis.set(response);
                    // Note: Voice synthesis requires JavaScript integration
                }
                Err(e) => {
                    set_error_message.set(format!("AI Analysis failed: {}", e));
                }
            }
            set_is_analyzing.set(false);
        });
    };

    view! {
        <Style>{r#"
            :root {
                --neon-red: #ff3333;
                --neon-orange: #ff6b35;
                --bg-black: #000000;
                --border-dim: #1a1a1a;
            }

            * { margin: 0; padding: 0; box-sizing: border-box; }

            body {
                font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
                background: var(--bg-black);
                color: #fff;
                min-height: 100vh;
            }

            .landing-nav {
                padding: 20px 32px;
                border-bottom: 1px solid var(--border-dim);
                display: flex;
                justify-content: space-between;
                align-items: center;
                background: rgba(0, 0, 0, 0.95);
                backdrop-filter: blur(20px);
                position: sticky;
                top: 0;
                z-index: 1000;
            }

            .logo {
                font-size: 24px;
                font-weight: 900;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                animation: pulse 3s ease-in-out infinite;
            }

            @keyframes pulse {
                0%, 100% { filter: brightness(1); }
                50% { filter: brightness(1.3); }
            }

            .nav-buttons {
                display: flex;
                gap: 12px;
            }

            .hero {
                padding: 60px 32px;
                text-align: center;
            }

            h1 {
                font-size: 64px;
                margin-bottom: 20px;
                background: linear-gradient(135deg, #fff, #888);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .sub {
                color: #888;
                font-size: 18px;
                max-width: 700px;
                margin: 0 auto 40px;
            }

            .btn {
                display: inline-block;
                padding: 14px 28px;
                margin: 10px;
                border: 1px solid #4a4a4a;
                border-radius: 10px;
                color: #fff;
                text-decoration: none;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s;
                cursor: pointer;
                background: transparent;
            }

            .btn:hover {
                border-color: #ff6b35;
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
                transform: translateY(-2px);
            }

            .btn-primary {
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                border: none;
                color: #000;
            }

            .chart-container {
                max-width: 1400px;
                margin: 40px auto;
                padding: 0 32px;
            }

            .asset-selector {
                display: flex;
                justify-content: center;
                gap: 16px;
                margin-bottom: 24px;
            }

            .asset-btn {
                padding: 12px 24px;
                border: 2px solid var(--border-dim);
                background: rgba(0, 0, 0, 0.8);
                color: #888;
                border-radius: 8px;
                cursor: pointer;
                font-family: inherit;
                font-size: 14px;
                font-weight: 600;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                transition: all 0.3s;
            }

            .asset-btn:hover, .asset-btn.active {
                border-color: var(--neon-orange);
                color: #fff;
                box-shadow: 0 0 15px rgba(255, 107, 53, 0.3);
            }

            .tradingview-widget-container {
                background: #000;
                border-radius: 16px;
                overflow: hidden;
                border: 1px solid var(--border-dim);
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
            }

            .ai-section {
                max-width: 900px;
                margin: 60px auto;
                padding: 0 32px;
            }

            .ai-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 24px;
            }

            .ai-title {
                font-size: 24px;
                font-weight: 800;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .ai-analysis-box {
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.8), rgba(10, 10, 10, 0.9));
                border: 1px solid var(--border-dim);
                border-radius: 16px;
                padding: 32px;
                min-height: 200px;
                white-space: pre-wrap;
                line-height: 1.6;
                color: #ccc;
            }

            .error-message {
                color: var(--neon-red);
                text-align: center;
                padding: 20px;
            }

            .loading {
                color: var(--neon-orange);
                text-align: center;
                animation: blink 1.5s infinite;
            }

            @keyframes blink {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.5; }
            }
        "#}</Style>

        <div class="landing-nav">
            <div class="logo">"Crypto Vibe Trade"</div>
            <div class="nav-buttons">
                <a href="/auth/login" class="btn">"Login"</a>
                <a href="/auth/register" class="btn btn-primary">"Register"</a>
            </div>
        </div>

        <div class="hero">
            <h1>"Trading Evolved"</h1>
            <p class="sub">
                "Advanced technical analysis with AI-powered insights. "
                "Real-time charts, predictive indicators, and professional risk analysis."
            </p>
        </div>

        <div class="chart-container">
            <div class="asset-selector">
                <button
                    class="asset-btn"
                    class:active=move || current_asset.get() == "BTC"
                    on:click=move |_| set_current_asset.set("BTC".to_string())
                >
                    "BTC"
                </button>
                <button
                    class="asset-btn"
                    class:active=move || current_asset.get() == "SOL"
                    on:click=move |_| set_current_asset.set("SOL".to_string())
                >
                    "SOL"
                </button>
                <button
                    class="asset-btn"
                    class:active=move || current_asset.get() == "ZEC"
                    on:click=move |_| set_current_asset.set("ZEC".to_string())
                >
                    "ZEC"
                </button>
            </div>

            <div inner_html=tradingview_widget></div>
        </div>

        <div class="ai-section">
            <div class="ai-header">
                <div class="ai-title">"🤖 Fenrir AI Analysis"</div>
                <button
                    class="btn btn-primary"
                    on:click=analyze_indicators
                    disabled=is_analyzing
                >
                    {move || if is_analyzing.get() { "Analyzing..." } else { "AI Analysis" }}
                </button>
            </div>

            <div class="ai-analysis-box">
                {move || {
                    if !error_message.get().is_empty() {
                        view! { <div class="error-message">{error_message.get()}</div> }.into_view()
                    } else if is_analyzing.get() {
                        view! { <div class="loading">"🔄 AI is analyzing the market...</div> }.into_view()
                    } else if ai_analysis.get().is_empty() {
                        view! { <div style="color:#666">"Use the button above for Gemini market analysis"</div> }.into_view()
                    } else {
                        view! { <div>{ai_analysis.get()}</div> }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
