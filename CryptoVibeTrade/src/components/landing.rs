use crate::server::ai_analyze;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Clone, Copy)]
struct TradingPair {
    code: &'static str,
    label: &'static str,
    symbol: &'static str,
}

const TRADING_PAIRS: [TradingPair; 3] = [
    TradingPair {
        code: "BTCUSDT",
        label: "BTC/USDT",
        symbol: "BINANCE:BTCUSDT",
    },
    TradingPair {
        code: "SOLUSDT",
        label: "SOL/USDT",
        symbol: "BINANCE:SOLUSDT",
    },
    TradingPair {
        code: "ZECUSDT",
        label: "ZEC/USDT",
        symbol: "BINANCE:ZECUSDT",
    },
];

fn find_pair(code: &str) -> TradingPair {
    TRADING_PAIRS
        .iter()
        .copied()
        .find(|pair| pair.code == code)
        .unwrap_or(TRADING_PAIRS[0])
}

#[component]
pub fn LandingPage() -> impl IntoView {
    let (current_pair, set_current_pair) = create_signal("BTCUSDT".to_string());
    let (ai_analysis, set_ai_analysis) = create_signal("".to_string());
    let (is_analyzing, set_is_analyzing) = create_signal(false);
    let (error_message, set_error_message) = create_signal(String::new());

    // Wallet connection state
    let (wallet_connected, set_wallet_connected) = create_signal(false);
    let (wallet_address, set_wallet_address) = create_signal(String::new());
    let (wallet_type, set_wallet_type) = create_signal(String::new());
    let (wallet_dropdown_open, set_wallet_dropdown_open) = create_signal(false);

    // Toggle wallet dropdown
    let toggle_wallet_dropdown = move |_| {
        let current = wallet_dropdown_open.get();
        set_wallet_dropdown_open.set(!current);
    };

    // Connect wallet handlers
    let connect_metamask = move |_| {
        set_wallet_type.set("MetaMask".to_string());
        set_wallet_address.set("0x1234...5678".to_string());
        set_wallet_connected.set(true);
        set_wallet_dropdown_open.set(false);
    };

    let connect_phantom = move |_| {
        set_wallet_type.set("Phantom".to_string());
        set_wallet_address.set("Solana...Demo".to_string());
        set_wallet_connected.set(true);
        set_wallet_dropdown_open.set(false);
    };

    let connect_jupiter = move |_| {
        set_wallet_type.set("Jupiter".to_string());
        set_wallet_address.set("Solana...Jupiter".to_string());
        set_wallet_connected.set(true);
        set_wallet_dropdown_open.set(false);
    };

    let connect_walletconnect = move |_| {
        set_wallet_type.set("WalletConnect".to_string());
        set_wallet_address.set("wc...1234".to_string());
        set_wallet_connected.set(true);
        set_wallet_dropdown_open.set(false);
    };

    let disconnect_wallet = move |_| {
        set_wallet_connected.set(false);
        set_wallet_address.set(String::new());
        set_wallet_type.set(String::new());
    };

    // TradingView widget - using reactive key to force iframe reload
    let (widget_key, set_widget_key) = create_signal(0);

    let tradingview_symbol = move || {
        let _ = widget_key.get(); // Reactive dependency
        find_pair(&current_pair.get()).symbol
    };

    let current_pair_label = move || find_pair(&current_pair.get()).label;

    let analyze_indicators = move |_| {
        set_is_analyzing.set(true);
        set_error_message.set(String::new());
        set_ai_analysis.set(String::new());

        let asset = current_pair_label().to_string();
        let indicators = "Ichimoku Cloud, RSI, MACD, Bollinger Bands, Volume".to_string();

        spawn_local(async move {
            match ai_analyze(
                "Analyze the current market conditions and provide technical analysis.".to_string(),
                asset.clone(),
                indicators.clone(),
            )
            .await
            {
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
                --neon-gold: #666600;
                --neon-gold-light: #999900;
                --bg-black: #000000;
                --border-dim: #1a1a1a;
                --neon-glow: 0 0 20px rgba(255, 107, 53, 0.5);
                --gold-glow: 0 0 20px rgba(153, 153, 0, 0.5);
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
                border-bottom: 2px solid var(--neon-gold);
                display: flex;
                justify-content: space-between;
                align-items: center;
                background: rgba(0, 0, 0, 0.95);
                backdrop-filter: blur(20px);
                position: sticky;
                top: 0;
                z-index: 1000;
                box-shadow: var(--gold-glow);
            }

            .logo {
                font-size: 24px;
                font-weight: 900;
                background: linear-gradient(135deg, #ff3333, #ff6b35, #999900);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                animation: pulse-glow 3s ease-in-out infinite;
                text-shadow: var(--neon-glow);
            }

            @keyframes pulse-glow {
                0%, 100% {
                    filter: brightness(1) drop-shadow(0 0 5px rgba(255, 107, 53, 0.5));
                }
                50% {
                    filter: brightness(1.4) drop-shadow(0 0 20px rgba(153, 153, 0, 0.8));
                }
            }

            .nav-buttons {
                display: flex;
                gap: 12px;
                align-items: center;
            }

            .hero {
                padding: 60px 32px 40px;
                text-align: center;
            }

            h1 {
                font-size: 64px;
                margin-bottom: 20px;
                background: linear-gradient(135deg, #fff, #999900, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                text-shadow: 0 0 30px rgba(153, 153, 0, 0.5);
                animation: shimmer 3s ease-in-out infinite;
            }

            @keyframes shimmer {
                0%, 100% { filter: brightness(1); }
                50% { filter: brightness(1.3); }
            }

            .sub {
                color: #999900;
                font-size: 18px;
                max-width: 700px;
                margin: 0 auto 40px;
                text-shadow: 0 0 10px rgba(153, 153, 0, 0.5);
            }

            .btn {
                display: inline-block;
                padding: 14px 28px;
                margin: 10px;
                border: 2px solid #666600;
                border-radius: 10px;
                color: #999900;
                text-decoration: none;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s;
                cursor: pointer;
                background: transparent;
                box-shadow: 0 0 10px rgba(102, 102, 0, 0.3);
            }

            .btn:hover {
                border-color: #ff6b35;
                color: #ff6b35;
                box-shadow: 0 0 30px rgba(255, 107, 53, 0.6);
                transform: translateY(-2px);
                text-shadow: 0 0 10px rgba(255, 107, 53, 0.8);
            }

            .btn-primary {
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                border: 2px solid #ff6b35;
                color: #000;
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
            }

            .btn-primary:hover {
                box-shadow: 0 0 40px rgba(255, 107, 53, 0.8);
            }

            .wallet-nav-section {
                display: flex;
                align-items: center;
                margin-left: auto;
                padding-left: 20px;
                border-left: 1px solid var(--neon-gold);
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
                padding: 14px 28px;
                border: 2px solid #666600;
                background: rgba(0, 0, 0, 0.8);
                color: #999900;
                border-radius: 10px;
                cursor: pointer;
                font-family: inherit;
                font-size: 14px;
                font-weight: 700;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s;
                box-shadow: 0 0 15px rgba(102, 102, 0, 0.4);
            }

            .asset-btn:hover {
                border-color: #ff6b35;
                color: #ff6b35;
                box-shadow: 0 0 25px rgba(255, 107, 53, 0.6);
                transform: translateY(-2px);
            }

            .asset-btn.active {
                border-color: #ff3333;
                color: #ff3333;
                background: rgba(255, 51, 51, 0.1);
                box-shadow: 0 0 30px rgba(255, 51, 51, 0.7);
                text-shadow: 0 0 15px rgba(255, 51, 51, 0.8);
            }

            .tradingview-widget-container {
                background: #000;
                border-radius: 16px;
                overflow: hidden;
                border: 2px solid #666600;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8), var(--gold-glow);
            }

            .pair-label {
                text-align: center;
                margin-bottom: 12px;
                font-size: 14px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
                color: #ff6b35;
                text-shadow: 0 0 10px rgba(255, 107, 53, 0.6);
            }

            .pair-subtext {
                text-align: center;
                color: #666600;
                font-size: 11px;
                margin-bottom: 18px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
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
                background: linear-gradient(135deg, #ff3333, #ff6b35, #999900);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                text-shadow: 0 0 20px rgba(153, 153, 0, 0.5);
            }

            .ai-analysis-box {
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.8), rgba(10, 10, 10, 0.9));
                border: 2px solid #666600;
                border-radius: 16px;
                padding: 32px;
                min-height: 200px;
                white-space: pre-wrap;
                line-height: 1.6;
                color: #999900;
                box-shadow: var(--gold-glow);
            }

            .error-message {
                color: var(--neon-red);
                text-align: center;
                padding: 20px;
                text-shadow: 0 0 10px rgba(255, 51, 51, 0.8);
            }

            .loading {
                color: #ff6b35;
                text-align: center;
                animation: blink 1.5s infinite;
                text-shadow: 0 0 10px rgba(255, 107, 53, 0.8);
            }

            @keyframes blink {
                0%, 100% { opacity: 1; }
                50% { opacity: 0.5; }
            }

            .wallet-dropdown {
                position: relative;
                display: inline-block;
            }

            .wallet-dropdown-btn {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 12px 20px;
                border: 2px solid #666600;
                background: rgba(0, 0, 0, 0.8);
                color: #999900;
                border-radius: 8px;
                cursor: pointer;
                font-family: inherit;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.05em;
                text-transform: uppercase;
                transition: all 0.3s;
                box-shadow: 0 0 15px rgba(102, 102, 0, 0.4);
            }

            .wallet-dropdown-btn:hover {
                border-color: #ff6b35;
                color: #ff6b35;
                background: rgba(255, 107, 53, 0.1);
                transform: translateY(-2px);
                box-shadow: 0 0 25px rgba(255, 107, 53, 0.6);
            }

            .dropdown-arrow {
                font-size: 10px;
                transition: transform 0.3s;
            }

            .wallet-dropdown.open .dropdown-arrow {
                transform: rotate(180deg);
            }

            .wallet-dropdown-menu {
                display: none;
                position: absolute;
                top: calc(100% + 8px);
                left: 0;
                background: rgba(10, 10, 10, 0.98);
                border: 2px solid #666600;
                border-radius: 12px;
                min-width: 220px;
                box-shadow: 0 10px 40px rgba(0, 0, 0, 0.8), var(--gold-glow);
                z-index: 1000;
                overflow: hidden;
            }

            .wallet-dropdown.open .wallet-dropdown-menu {
                display: block;
            }

            .dropdown-item {
                padding: 14px 18px;
                cursor: pointer;
                font-size: 13px;
                font-weight: 500;
                border-bottom: 1px solid #666600;
                transition: all 0.2s;
                color: #999900;
            }

            .dropdown-item:last-child {
                border-bottom: none;
            }

            .dropdown-item:hover {
                background: rgba(255, 107, 53, 0.15);
                color: #ff6b35;
                text-shadow: 0 0 10px rgba(255, 107, 53, 0.8);
            }
        "#}</Style>

        <div class="landing-nav">
            <div class="logo">"Crypto Vibe Trade"</div>
            <div class="nav-buttons">
                <a href="/auth/login" class="btn">"Login"</a>
                <a href="/auth/register" class="btn btn-primary">"Register"</a>
                <div class="wallet-nav-section">
                    {move || {
                        if wallet_connected.get() {
                            view! {
                                <div class="wallet-dropdown">
                                    <button class="wallet-dropdown-btn" on:click=toggle_wallet_dropdown>
                                        {format!("{}: {}", wallet_type.get(), wallet_address.get())}
                                        <span class="dropdown-arrow">"▼"</span>
                                    </button>
                                    <div class="wallet-dropdown-menu" class:open=wallet_dropdown_open>
                                        <div class="dropdown-item" on:click=disconnect_wallet>"Disconnect"</div>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="wallet-dropdown" class:open=wallet_dropdown_open>
                                    <button class="wallet-dropdown-btn" on:click=toggle_wallet_dropdown>
                                        "Connect Wallet"
                                        <span class="dropdown-arrow">"▼"</span>
                                    </button>
                                    <div class="wallet-dropdown-menu">
                                        <div class="dropdown-item" on:click=connect_metamask>"🦊 MetaMask"</div>
                                        <div class="dropdown-item" on:click=connect_phantom>"👻 Phantom"</div>
                                        <div class="dropdown-item" on:click=connect_jupiter>"🪐 Jupiter"</div>
                                        <div class="dropdown-item" on:click=connect_walletconnect>"🔗 WalletConnect"</div>
                                    </div>
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
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
                <For
                    each=|| TRADING_PAIRS.iter().copied()
                    key=|pair| pair.code
                    children=move |pair| {
                        let pair_code = pair.code;
                        let label = pair.label;
                        view! {
                            <button
                                class="asset-btn"
                                class:active=move || current_pair.get() == pair_code
                                on:click=move |_| {
                                    set_current_pair.set(pair_code.to_string());
                                    set_widget_key.update(|k| *k += 1);
                                }
                            >
                                {label}
                            </button>
                        }
                    }
                />
            </div>
            <div class="pair-label">{move || current_pair_label()}</div>
            <div class="pair-subtext">"Live Binance spot data via free TradingView widget"</div>

            {move || {
                let symbol = tradingview_symbol();
                let key = widget_key.get();
                let html = format!(r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <meta charset="utf-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <style>
                            body, html {{ margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; }}
                            .tradingview-widget-container {{ width: 100%; height: 100%; }}
                        </style>
                    </head>
                    <body>
                        <div class="tradingview-widget-container">
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
                    </body>
                    </html>
                "#, symbol);

                view! {
                    <iframe
                        srcdoc={html}
                        class="tradingview-widget-container"
                        style="height:500px;width:100%;border:none;border-radius:16px;"
                        sandbox="allow-scripts allow-same-origin"
                        key={format!("tv-{}", key)}
                    ></iframe>
                }
            }}
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
                        let msg = error_message.get();
                        view! { <div class="error-message">{msg}</div> }
                    } else if is_analyzing.get() {
                        view! { <div class="loading">"🔄 AI is analyzing the market..."</div> }
                    } else if ai_analysis.get().is_empty() {
                        view! { <div style="color:#666">"Press the button above to start."</div> }
                    } else {
                        let analysis = ai_analysis.get();
                        view! { <div>{analysis}</div> }
                    }
                }}
            </div>
        </div>
    }
}
