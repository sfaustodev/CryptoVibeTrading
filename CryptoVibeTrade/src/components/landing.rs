use leptos::*;
use leptos_meta::*;

#[component]
pub fn LandingPage() -> impl IntoView {
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
            }

            .landing-nav {
                padding: 20px 32px;
                border-bottom: 1px solid var(--border-dim);
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .logo {
                font-size: 24px;
                font-weight: 900;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .hero {
                padding: 100px 32px;
                text-align: center;
            }

            h1 {
                font-size: 64px;
                margin-bottom: 20px;
                background: linear-gradient(135deg, #fff, #888);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .sub { color: #888; font-size: 18px; max-width: 700px; margin: 0 auto; }

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
            }

            .btn:hover {
                border-color: #ff6b35;
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
            }

            .btn-primary {
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                border: none;
                color: #000;
            }
        "#}</Style>

        <div class="landing-nav">
            <div class="logo">"Crypto Vibe Trade"</div>
            <div>
                <a href="/admin" class="btn">"Admin Login"</a>
            </div>
        </div>

        <div class="hero">
            <h1>"Trading Evolved"</h1>
            <p class="sub">
                "Advanced technical analysis with AI-powered insights. "
                "Real-time charts, predictive indicators, and professional risk analysis."
            </p>
            <div style="margin-top: 40px;">
                <a href="/admin" class="btn btn-primary">"Access Dashboard"</a>
            </div>
        </div>
    }
}
