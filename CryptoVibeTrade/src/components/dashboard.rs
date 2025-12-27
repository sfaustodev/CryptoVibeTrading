use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::MouseEvent;
use crate::components::dragon::Dragon;
use crate::server::{grok_analyze, verify_nft};

#[component]
pub fn DashboardPage() -> impl IntoView {
    let navigate = use_navigate();

    // Dragon state signals
    let (is_speaking, set_is_speaking) = create_signal(false);
    let (is_firing, set_is_firing) = create_signal(false);
    let (dragon_x, set_dragon_x) = create_signal(50.0); // % position
    let (dragon_y, set_dragon_y) = create_signal(20.0); // % position
    let (rotation, set_rotation) = create_signal(0.0); // degrees
    let (mouse_x, set_mouse_x) = create_signal(0.0);
    let (mouse_y, set_mouse_y) = create_signal(0.0);

    // Grok analysis state
    let (analysis_text, set_analysis_text) = create_signal(String::new());
    let (is_analyzing, set_is_analyzing) = create_signal(false);
    let (show_speech, set_show_speech) = create_signal(false);

    // NFT verification state
    let (wallet_address, set_wallet_address) = create_signal(String::new());
    let (mint_address, set_mint_address) = create_signal(String::new());
    let (is_nft_holder, set_is_nft_holder) = create_signal(false);
    let (is_verifying, set_is_verifying) = create_signal(false);
    let (nft_message, set_nft_message) = create_signal(String::new());

    // Mouse tracking for dragon cursor following
    let handle_mouse_move = move |ev: MouseEvent| {
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        set_mouse_x.set(x);
        set_mouse_y.set(y);

        // Get window dimensions safely
        let win = window();
        let width = win.inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(800.0);
        let height = win.inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(600.0);

        // Calculate rotation angle to face cursor
        let dx = x - (width * (dragon_x.get() / 100.0));
        let dy = y - (height * (dragon_y.get() / 100.0));
        let angle = (dy.atan2(dx) * 180.0 / std::f64::consts::PI) + 90.0;

        set_rotation.set(angle);

        // Move dragon towards mouse (smooth follow, 5% of the distance)
        let new_x = dragon_x.get() + (x / width * 100.0 - dragon_x.get()) * 0.05;
        let new_y = dragon_y.get() + (y / height * 100.0 - dragon_y.get()) * 0.05;

        // Keep dragon within bounds (10-90%)
        set_dragon_x.set(new_x.clamp(10.0, 90.0));
        set_dragon_y.set(new_y.clamp(10.0, 90.0));
    };

    // Grok analysis + voice synthesis handler
    let handle_analyze = move |_| {
        // Check NFT verification first
        if !is_nft_holder.get() {
            set_nft_message.set("⚠️ NFT verification required for Grok analysis".to_string());
            return;
        }
        let set_is_speaking = set_is_speaking.clone();
        let set_analysis_text = set_analysis_text.clone();
        let set_is_analyzing = set_is_analyzing.clone();
        let set_show_speech = set_show_speech.clone();

        set_is_analyzing.set(true);
        set_show_speech.set(false);

        spawn_local(async move {
            let prompt = "Analyze current market conditions and provide trading insights for Solana DeFi pools".to_string();
            let selected_text = "Orca, Raydium, Meteora liquidity pools".to_string();
            let include_screenshot = false;

            match grok_analyze(prompt, selected_text, include_screenshot).await {
                Ok(analysis) => {
                    set_analysis_text.set(analysis.clone());

                    // Trigger dragon speech
                    set_show_speech.set(true);
                    set_is_speaking.set(true);

                    // Use the speak function from dragon module
                    if let Err(e) = crate::components::dragon::speak(&analysis) {
                        leptos::logging::log!("Speech error: {:?}", e);
                    }

                    // Reset speaking state after estimated duration
                    use std::time::Duration;
                    let duration = Duration::from_millis(analysis.len() as u64 * 80); // ~80ms per character
                    set_timeout(
                        move || {
                            set_is_speaking.set(false);
                            set_show_speech.set(false);
                        },
                        duration,
                    );
                }
                Err(e) => {
                    leptos::logging::log!("Grok analysis error: {:?}", e);
                    set_analysis_text.set(format!("Analysis failed: {}", e));
                }
            }
            set_is_analyzing.set(false);
        });
    };

    // NFT verification handler
    let handle_verify_nft = move |_| {
        let wallet = wallet_address.get();
        let mint = mint_address.get();

        if wallet.is_empty() || mint.is_empty() {
            set_nft_message.set("Please enter both wallet and mint addresses".to_string());
            return;
        }

        set_is_verifying.set(true);
        set_nft_message.set("Verifying NFT ownership...".to_string());

        spawn_local(async move {
            match verify_nft(wallet, mint).await {
                Ok(response) => {
                    set_is_nft_holder.set(response.is_holder);
                    set_nft_message.set(response.message);
                }
                Err(e) => {
                    set_nft_message.set(format!("Verification failed: {}", e));
                    set_is_nft_holder.set(false);
                }
            }
            set_is_verifying.set(false);
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

            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            body {
                font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
                background: var(--bg-black);
                color: #fff;
                overflow: hidden;
            }

            .dashboard-container {
                height: 100vh;
                width: 100vw;
                display: flex;
                flex-direction: column;
                background: var(--bg-black);
                position: relative;
            }

            .dashboard-header {
                padding: 12px 20px;
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.95), rgba(10, 10, 10, 0.98));
                border-bottom: 1px solid var(--border-dim);
                display: flex;
                justify-content: space-between;
                align-items: center;
                flex-shrink: 0;
            }

            .dashboard-title {
                font-size: 18px;
                font-weight: 800;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .logout-btn {
                padding: 8px 16px;
                border: 1px solid #4a4a4a;
                background: transparent;
                color: #fff;
                border-radius: 8px;
                cursor: pointer;
                font-family: inherit;
                font-size: 10px;
                font-weight: 600;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                transition: all 0.3s;
            }

            .logout-btn:hover {
                border-color: var(--neon-orange);
                box-shadow: 0 0 15px rgba(255, 107, 53, 0.3);
            }

            .iframes-container {
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
                position: relative;
            }

            .iframe-wrapper {
                flex: 1;
                display: flex;
                flex-direction: column;
                border-bottom: 2px solid var(--border-dim);
                overflow: hidden;
                min-height: 0;
            }

            .iframe-wrapper:last-child {
                border-bottom: none;
            }

            .iframe-header {
                padding: 8px 16px;
                background: rgba(26, 26, 26, 0.8);
                font-size: 11px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                color: #888;
                display: flex;
                align-items: center;
                gap: 8px;
                flex-shrink: 0;
            }

            .iframe-content {
                flex: 1;
                width: 100%;
                border: none;
                background: #000;
            }

            .resize-handle {
                height: 4px;
                background: var(--border-dim);
                cursor: ns-resize;
                flex-shrink: 0;
                transition: background 0.2s;
            }

            .resize-handle:hover {
                background: var(--neon-orange);
            }

            /* Grokinho Dragon Overlay */
            .grokinho-overlay {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                pointer-events: none;
                z-index: 9999;
            }

            .nft-required {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.98), rgba(10, 10, 10, 0.99));
                border: 2px solid var(--neon-orange);
                border-radius: 20px;
                padding: 40px;
                text-align: center;
                z-index: 10000;
                box-shadow: 0 0 60px rgba(255, 107, 53, 0.3);
            }

            .nft-required h2 {
                font-size: 32px;
                margin-bottom: 16px;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .nft-required p {
                color: #888;
                font-size: 14px;
                line-height: 1.6;
                margin-bottom: 24px;
            }

            .nft-badge {
                display: inline-block;
                padding: 6px 12px;
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                border-radius: 6px;
                font-size: 10px;
                font-weight: 700;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                margin-top: 16px;
            }

            .analyze-btn {
                padding: 8px 16px;
                border: 2px solid var(--neon-orange);
                background: linear-gradient(135deg, rgba(255, 51, 51, 0.1), rgba(255, 107, 53, 0.1));
                color: var(--neon-orange);
                border-radius: 8px;
                cursor: pointer;
                font-family: inherit;
                font-size: 10px;
                font-weight: 700;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                transition: all 0.3s;
                display: flex;
                align-items: center;
                gap: 6px;
            }

            .analyze-btn:hover:not(:disabled) {
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                color: #000;
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.5);
                transform: translateY(-1px);
            }

            .analyze-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }

            .analyze-btn .loading {
                animation: spin 1s linear infinite;
            }

            @keyframes spin {
                from { transform: rotate(0deg); }
                to { transform: rotate(360deg); }
            }

            .nft-verify-section {
                padding: 16px 20px;
                background: rgba(26, 26, 26, 0.8);
                border-bottom: 1px solid var(--border-dim);
                display: flex;
                flex-direction: column;
                gap: 12px;
                flex-shrink: 0;
            }

            .nft-verify-title {
                font-size: 11px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                color: #888;
            }

            .nft-inputs {
                display: flex;
                gap: 12px;
                align-items: center;
            }

            .nft-input {
                flex: 1;
                padding: 10px 14px;
                border: 1px solid var(--border-dim);
                border-radius: 8px;
                background: rgba(0, 0, 0, 0.8);
                color: #fff;
                font-family: inherit;
                font-size: 11px;
            }

            .nft-input:focus {
                outline: none;
                border-color: var(--neon-orange);
                box-shadow: 0 0 15px rgba(255, 107, 53, 0.2);
            }

            .verify-btn {
                padding: 10px 20px;
                border: 1px solid #4a4a4a;
                background: transparent;
                color: #fff;
                border-radius: 8px;
                cursor: pointer;
                font-family: inherit;
                font-size: 10px;
                font-weight: 600;
                letter-spacing: 0.1em;
                text-transform: uppercase;
                transition: all 0.3s;
                white-space: nowrap;
            }

            .verify-btn:hover:not(:disabled) {
                border-color: var(--neon-orange);
                box-shadow: 0 0 15px rgba(255, 107, 53, 0.3);
            }

            .verify-btn:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }

            .nft-status {
                font-size: 11px;
                padding: 8px 12px;
                border-radius: 6px;
                background: rgba(0, 0, 0, 0.5);
            }

            .nft-status.verified {
                border: 1px solid #00ff88;
                color: #00ff88;
            }

            .nft-status.error {
                border: 1px solid var(--neon-red);
                color: var(--neon-red);
            }

            .nft-status.neutral {
                border: 1px solid #4a4a4a;
                color: #888;
            }

            .dragon-speech-bubble {
                position: absolute;
                bottom: 80px;
                left: 50%;
                transform: translateX(-50%);
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.95), rgba(10, 10, 10, 0.98));
                border: 2px solid var(--neon-orange);
                border-radius: 12px;
                padding: 12px 16px;
                min-width: 200px;
                max-width: 300px;
                color: #fff;
                font-size: 12px;
                line-height: 1.4;
                box-shadow: 0 4px 20px rgba(255, 107, 53, 0.3);
                opacity: 0;
                transition: opacity 0.3s;
                pointer-events: none;
                z-index: 10001;
            }

            .dragon-speech-bubble.visible {
                opacity: 1;
            }

            .dragon-speech-bubble::after {
                content: '';
                position: absolute;
                bottom: -8px;
                left: 50%;
                transform: translateX(-50%);
                border-width: 8px 8px 0;
                border-style: solid;
                border-color: var(--neon-orange) transparent transparent transparent;
            }
        "#}</Style>

        <div
            class="dashboard-container"
            on:mousemove=handle_mouse_move
        >
            <div class="dashboard-header">
                <div class="dashboard-title">"🐺 Fenrir Dashboard"</div>
                <div style="display: flex; gap: 12px; align-items: center;">
                    <button
                        class="analyze-btn"
                        on:click=handle_analyze
                        disabled=is_analyzing
                    >
                        {move || if is_analyzing.get() { "🔄 Analyzing..." } else { "🐉 Grok Analysis" }}
                    </button>
                    <button
                        class="logout-btn"
                        on:click=move |_| {
                            navigate(&"/admin".to_string(), Default::default());
                        }
                    >
                        "Logout"
                    </button>
                </div>
            </div>

            // NFT Verification Section
            <div class="nft-verify-section">
                <div class="nft-verify-title">"🔮 NFT Verification Required"</div>
                <div class="nft-inputs">
                    <input
                        type="text"
                        class="nft-input"
                        placeholder="Wallet Address"
                        prop:value=wallet_address
                        on:input=move |ev| set_wallet_address.set(event_target_value(&ev))
                    />
                    <input
                        type="text"
                        class="nft-input"
                        placeholder="NFT Mint Address"
                        prop:value=mint_address
                        on:input=move |ev| set_mint_address.set(event_target_value(&ev))
                    />
                    <button
                        class="verify-btn"
                        on:click=handle_verify_nft
                        disabled=is_verifying
                    >
                        {move || if is_verifying.get() { "🔄 Verifying..." } else { "Verify" }}
                    </button>
                </div>
                <div
                    class="nft-status"
                    class:verified=is_nft_holder
                    class:error=move || !is_nft_holder.get() && !nft_message.get().is_empty()
                    class:neutral=move || nft_message.get().is_empty()
                >
                    {move || nft_message.get()}
                </div>
            </div>

            <div class="iframes-container">
                <div class="iframe-wrapper">
                    <div class="iframe-header">"Orca.so Pools"</div>
                    <iframe
                        src="https://orca.so/pools"
                        class="iframe-content"
                        title="Orca Pools"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen=true
                    ></iframe>
                </div>

                <div class="resize-handle"></div>

                <div class="iframe-wrapper">
                    <div class="iframe-header">"Raydium Liquidity Pools"</div>
                    <iframe
                        src="https://raydium.io/liquidity-pools/"
                        class="iframe-content"
                        title="Raydium Pools"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen=true
                    ></iframe>
                </div>

                <div class="resize-handle"></div>

                <div class="iframe-wrapper">
                    <div class="iframe-header">"Meteora Top Pools"</div>
                    <iframe
                        src="https://meteora.ag/?tab=top"
                        class="iframe-content"
                        title="Meteora Top"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                        allowfullscreen=true
                    ></iframe>
                </div>

                // Grokinho the Dragon (always visible for now, will be NFT-gated later)
                <Dragon
                    is_speaking=is_speaking.into()
                    is_firing=is_firing.into()
                    pos_x=dragon_x.into()
                    pos_y=dragon_y.into()
                    rotation=rotation.into()
                />

                // Speech bubble for dragon's analysis
                <div
                    class="dragon-speech-bubble"
                    class:visible=show_speech
                    style:left=move || format!("{}%", dragon_x.get())
                    style:top=move || format!("{}%", dragon_y.get() - 10.0)
                >
                    {move || analysis_text.get()}
                </div>
            </div>
        </div>
    }
}
