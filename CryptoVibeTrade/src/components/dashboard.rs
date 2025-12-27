use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let navigate = use_navigate();
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
        "#}</Style>

        <div class="dashboard-container">
            <div class="dashboard-header">
                <div class="dashboard-title">"🐺 Fenrir Dashboard"</div>
                <button
                    class="logout-btn"
                    on:click=move |_| {
                        navigate(&"/admin".to_string(), Default::default());
                    }
                >
                    "Logout"
                </button>
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
            </div>
        </div>
    }
}
