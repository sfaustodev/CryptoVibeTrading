use crate::server::{login, register_user};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let navigate = use_navigate();
    let (full_name, set_full_name) = create_signal(String::new());
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (birth_date, set_birth_date) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (success, set_success) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);

    let handle_register = move |_| {
        let full_name_val = full_name.get();
        let username_val = username.get();
        let email_val = email.get();
        let birth_date_val = birth_date.get();
        let password_val = password.get();
        let confirm_val = confirm_password.get();

        // Validation
        if full_name_val.trim().is_empty() {
            set_error.set("Full name is required".to_string());
            return;
        }

        if username_val.len() < 3 {
            set_error.set("Username must be at least 3 characters".to_string());
            return;
        }

        if password_val.len() < 8 {
            set_error.set("Password must be at least 8 characters".to_string());
            return;
        }

        if password_val != confirm_val {
            set_error.set("Passwords do not match".to_string());
            return;
        }

        set_is_loading.set(true);
        set_error.set(String::new());
        set_success.set(String::new());

        let navigate = navigate.clone();
        spawn_local(async move {
            let full_name_value = full_name_val.trim().to_string();
            let birth_date_value = birth_date_val.trim().to_string();
            let birth_date_payload = if birth_date_value.is_empty() {
                None
            } else {
                Some(birth_date_value)
            };
            match register_user(
                username_val,
                email_val,
                Some(full_name_value),
                birth_date_payload,
                password_val,
            )
            .await
            {
                Ok(response) => {
                    if response.success {
                        set_success.set(response.message);
                        set_is_loading.set(false);
                        // Redirect to login immediately
                        navigate(&"/auth/login".to_string(), Default::default());
                    } else {
                        set_error.set(response.message);
                        set_is_loading.set(false);
                    }
                }
                Err(e) => {
                    set_error.set(format!("Registration failed: {}", e));
                    set_is_loading.set(false);
                }
            }
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
                height: 100vh;
            }

            .register-container {
                height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background: linear-gradient(135deg, #000 0%, #0a0a0a 100%);
            }

            .register-box {
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.8), rgba(10, 10, 10, 0.9));
                border: 1px solid var(--border-dim);
                border-radius: 20px;
                padding: 48px;
                min-width: 450px;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
            }

            .register-title {
                font-size: 32px;
                font-weight: 800;
                margin-bottom: 8px;
                text-align: center;
                background: linear-gradient(135deg, #ff3333, #ff6b35);
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
            }

            .register-subtitle {
                color: #888;
                text-align: center;
                margin-bottom: 32px;
                font-size: 12px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
            }

            .form-group { margin-bottom: 20px; }

            .form-group label {
                display: block;
                margin-bottom: 8px;
                font-size: 11px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
                color: #888;
            }

            .form-group input {
                width: 100%;
                padding: 14px 18px;
                border-radius: 10px;
                border: 1px solid var(--border-dim);
                background: rgba(0, 0, 0, 0.8);
                color: #fff;
                font-family: inherit;
                font-size: 14px;
            }

            .form-group input:focus {
                outline: none;
                border-color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.2);
            }

            .btn {
                width: 100%;
                padding: 14px 20px;
                border: 1px solid #4a4a4a;
                background: transparent;
                color: #fff;
                border-radius: 10px;
                cursor: pointer;
                font-family: inherit;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s;
            }

            .btn:hover {
                border-color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
            }

            .btn-primary {
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                border: none;
                color: #000;
            }

            .error-msg {
                color: var(--neon-red);
                font-size: 12px;
                margin-top: 12px;
                text-align: center;
            }

            .success-msg {
                color: #00ff88;
                font-size: 12px;
                margin-top: 12px;
                text-align: center;
            }

            .login-link {
                text-align: center;
                margin-top: 20px;
                color: #888;
                font-size: 12px;
            }

            .login-link a {
                color: var(--neon-orange);
                text-decoration: none;
            }

            .login-link a:hover {
                text-decoration: underline;
            }

            .social-section {
                margin-top: 24px;
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .social-divider {
                display: flex;
                align-items: center;
                gap: 12px;
                color: #666;
                font-size: 10px;
                letter-spacing: 0.25em;
                text-transform: uppercase;
                margin-top: 16px;
            }

            .social-divider::before,
            .social-divider::after {
                content: "";
                flex: 1;
                height: 1px;
                background: #222;
            }

            .social-btn {
                width: 100%;
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 10px;
                padding: 12px 16px;
                border-radius: 10px;
                border: 1px solid #2a2a2a;
                background: rgba(0, 0, 0, 0.6);
                color: #fff;
                font-family: inherit;
                font-size: 12px;
                letter-spacing: 0.12em;
                text-transform: uppercase;
                cursor: pointer;
                transition: all 0.3s;
            }

            .social-btn:hover {
                border-color: var(--neon-orange);
                box-shadow: 0 0 18px rgba(255, 107, 53, 0.3);
                transform: translateY(-1px);
            }

            .social-icon {
                width: 18px;
                height: 18px;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                border-radius: 50%;
                font-weight: 700;
                background: #111;
                color: #fff;
                font-size: 12px;
            }

            .social-icon.apple {
                font-size: 14px;
            }
        "#}</Style>

        <div class="register-container">
            <div class="register-box">
                <div class="register-title">"Join CVT"</div>
                <div class="register-subtitle">"Create Your Account"</div>

                <form
                    on:submit=move |ev| {
                        ev.prevent_default();
                        handle_register(());
                    }
                >
                    <div class="form-group">
                        <label>"Full Name"</label>
                        <input
                            type="text"
                            prop:value=full_name
                            on:input=move |ev| set_full_name.set(event_target_value(&ev))
                            placeholder="Enter your full name"
                        />
                    </div>

                    <div class="form-group">
                        <label>"Username"</label>
                        <input
                            type="text"
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                            placeholder="Choose a username"
                            minlength=3
                        />
                    </div>

                    <div class="form-group">
                        <label>"Email"</label>
                        <input
                            type="email"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            placeholder="your@email.com"
                        />
                    </div>

                    <div class="form-group">
                        <label>"Birth Date"</label>
                        <input
                            type="date"
                            prop:value=birth_date
                            on:input=move |ev| set_birth_date.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-group">
                        <label>"Password"</label>
                        <input
                            type="password"
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            placeholder="Min 8 characters"
                            minlength=8
                        />
                    </div>

                    <div class="form-group">
                        <label>"Confirm Password"</label>
                        <input
                            type="password"
                            prop:value=confirm_password
                            on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                            placeholder="Confirm password"
                        />
                    </div>

                    <div class="error-msg">{move || error.get()}</div>
                    <div class="success-msg">{move || success.get()}</div>

                    <button
                        class="btn btn-primary"
                        style="margin-top:16px;"
                        type="submit"
                        disabled=is_loading
                    >
                        {move || if is_loading.get() { "Creating Account..." } else { "Register" }}
                    </button>
                </form>

                <div class="social-divider">"Or register with"</div>
                <div class="social-section">
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| {
                            set_error.set("Google registration is coming soon.".to_string());
                            set_success.set(String::new());
                        }
                    >
                        <span class="social-icon">"G"</span>
                        "Continue with Google"
                    </button>
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| {
                            set_error.set("Apple registration is coming soon.".to_string());
                            set_success.set(String::new());
                        }
                    >
                        <span class="social-icon apple">""</span>
                        "Continue with Apple"
                    </button>
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| {
                            set_error.set("X registration is coming soon.".to_string());
                            set_success.set(String::new());
                        }
                    >
                        <span class="social-icon">"X"</span>
                        "Continue with X"
                    </button>
                </div>

                <div class="login-link">
                    "Already have an account? "
                    <a href="/auth/login">"Login"</a>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);

    let handle_login = {
        let navigate = navigate.clone();
        move |_| {
            let username_val = username.get();
            let password_val = password.get();

            if username_val.is_empty() || password_val.is_empty() {
                set_error.set("Please enter both username and password".to_string());
                return;
            }

            set_is_loading.set(true);
            set_error.set(String::new());

            let navigate = navigate.clone();
            spawn_local(async move {
                match login(username_val, password_val).await {
                    Ok(response) => {
                        if response.success {
                            set_is_loading.set(false);
                            if response.is_admin.unwrap_or(false) {
                                navigate(&"/admin/dashboard".to_string(), Default::default());
                            } else {
                                navigate(&"/".to_string(), Default::default());
                            }
                        } else {
                            set_error.set(response.message);
                            set_is_loading.set(false);
                        }
                    }
                    Err(e) => {
                        set_error.set(format!("Login failed: {}", e));
                        set_is_loading.set(false);
                    }
                }
            });
        }
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
                height: 100vh;
            }

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
                color: #888;
                text-align: center;
                margin-bottom: 32px;
                font-size: 12px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
            }

            .form-group { margin-bottom: 20px; }

            .form-group label {
                display: block;
                margin-bottom: 8px;
                font-size: 11px;
                letter-spacing: 0.2em;
                text-transform: uppercase;
                color: #888;
            }

            .form-group input {
                width: 100%;
                padding: 14px 18px;
                border-radius: 10px;
                border: 1px solid var(--border-dim);
                background: rgba(0, 0, 0, 0.8);
                color: #fff;
                font-family: inherit;
                font-size: 14px;
            }

            .form-group input:focus {
                outline: none;
                border-color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.2);
            }

            .btn {
                width: 100%;
                padding: 14px 20px;
                border: 1px solid #4a4a4a;
                background: transparent;
                color: #fff;
                border-radius: 10px;
                cursor: pointer;
                font-family: inherit;
                font-size: 12px;
                font-weight: 600;
                letter-spacing: 0.15em;
                text-transform: uppercase;
                transition: all 0.3s;
            }

            .btn:hover {
                border-color: var(--neon-orange);
                box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
            }

            .btn-primary {
                background: linear-gradient(135deg, var(--neon-red), var(--neon-orange));
                border: none;
                color: #000;
            }

            .error-msg {
                color: var(--neon-red);
                font-size: 12px;
                margin-top: 12px;
                text-align: center;
            }

            .register-link {
                text-align: center;
                margin-top: 20px;
                color: #888;
                font-size: 12px;
            }

            .register-link a {
                color: var(--neon-orange);
                text-decoration: none;
            }

            .register-link a:hover {
                text-decoration: underline;
            }

            .social-section {
                margin-top: 24px;
                display: flex;
                flex-direction: column;
                gap: 12px;
            }

            .social-divider {
                display: flex;
                align-items: center;
                gap: 12px;
                color: #666;
                font-size: 10px;
                letter-spacing: 0.25em;
                text-transform: uppercase;
                margin-top: 16px;
            }

            .social-divider::before,
            .social-divider::after {
                content: "";
                flex: 1;
                height: 1px;
                background: #222;
            }

            .social-btn {
                width: 100%;
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 10px;
                padding: 12px 16px;
                border-radius: 10px;
                border: 1px solid #2a2a2a;
                background: rgba(0, 0, 0, 0.6);
                color: #fff;
                font-family: inherit;
                font-size: 12px;
                letter-spacing: 0.12em;
                text-transform: uppercase;
                cursor: pointer;
                transition: all 0.3s;
            }

            .social-btn:hover {
                border-color: var(--neon-orange);
                box-shadow: 0 0 18px rgba(255, 107, 53, 0.3);
                transform: translateY(-1px);
            }

            .social-icon {
                width: 18px;
                height: 18px;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                border-radius: 50%;
                font-weight: 700;
                background: #111;
                color: #fff;
                font-size: 12px;
            }

            .social-icon.apple {
                font-size: 14px;
            }
        "#}</Style>

        <div class="login-container">
            <div class="login-box">
                <div class="login-title">"Welcome Back"</div>
                <div class="login-subtitle">"Login to Your Account"</div>

                <form
                    on:submit=move |ev| {
                        ev.prevent_default();
                        handle_login(());
                    }
                >
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
                        />
                    </div>

                    <div class="error-msg">{move || error.get()}</div>

                    <button
                        class="btn btn-primary"
                        style="margin-top:16px;"
                        type="submit"
                        disabled=is_loading
                    >
                        {move || if is_loading.get() { "Authenticating..." } else { "Login" }}
                    </button>
                </form>

                <div class="social-divider">"Or login with"</div>
                <div class="social-section">
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| set_error.set("Google login is coming soon.".to_string())
                    >
                        <span class="social-icon">"G"</span>
                        "Continue with Google"
                    </button>
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| set_error.set("Apple login is coming soon.".to_string())
                    >
                        <span class="social-icon apple">""</span>
                        "Continue with Apple"
                    </button>
                    <button
                        class="social-btn"
                        type="button"
                        on:click=move |_| set_error.set("X login is coming soon.".to_string())
                    >
                        <span class="social-icon">"X"</span>
                        "Continue with X"
                    </button>
                </div>

                <div class="register-link">
                    "Don't have an account? "
                    <a href="/auth/register">"Register"</a>
                </div>
            </div>
        </div>
    }
}
