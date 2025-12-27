use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;
use web_sys::{SpeechSynthesis, SpeechSynthesisUtterance, Window};

#[component]
pub fn Dragon(
    /// Whether the dragon is currently speaking
    is_speaking: ReadSignal<bool>,
    /// Whether dragon should show fire breath effect
    is_firing: ReadSignal<bool>,
    /// Dragon's horizontal position (0-100%)
    pos_x: ReadSignal<f64>,
    /// Dragon's vertical position (0-100%)
    pos_y: ReadSignal<f64>,
    /// Rotation angle in degrees
    rotation: ReadSignal<f64>,
) -> impl IntoView {
    view! {
        <Style>{r#"
            .grokinho-container {
                position: absolute;
                width: 64px;
                height: 64px;
                z-index: 1000;
                pointer-events: none;
                transition: transform 0.1s ease-out;
            }

            .grokinho-dragon {
                font-size: 64px;
                line-height: 1;
                user-select: none;
                filter: drop-shadow(0 0 10px rgba(255, 107, 53, 0.5));
                animation: dragon-bob 2s ease-in-out infinite;
            }

            .grokinho-dragon.speaking {
                animation: speaking 0.2s ease-in-out infinite;
            }

            .grokinho-dragon.firing {
                filter: drop-shadow(0 0 20px rgba(255, 69, 0, 0.8))
                        drop-shadow(0 0 40px rgba(255, 140, 0, 0.6));
            }

            /* Fire breath particles */
            .fire-particle {
                position: absolute;
                width: 12px;
                height: 12px;
                background: radial-gradient(circle, #ff6b35, #ff3333, transparent);
                border-radius: 50%;
                opacity: 0;
                animation: none;
            }

            .grokinho-dragon.firing ~ .fire-particles .fire-particle {
                animation: fire-breath 0.6s ease-out infinite;
            }

            .fire-particle:nth-child(1) { top: 30px; left: 64px; animation-delay: 0s; }
            .fire-particle:nth-child(2) { top: 25px; left: 64px; animation-delay: 0.1s; }
            .fire-particle:nth-child(3) { top: 35px; left: 64px; animation-delay: 0.2s; }
            .fire-particle:nth-child(4) { top: 28px; left: 64px; animation-delay: 0.3s; }
            .fire-particle:nth-child(5) { top: 32px; left: 64px; animation-delay: 0.4s; }

            @keyframes dragon-bob {
                0%, 100% { transform: translateY(0px) scale(1); }
                50% { transform: translateY(-3px) scale(1.02); }
            }

            @keyframes speaking {
                0%, 100% { transform: scaleY(1); }
                50% { transform: scaleY(0.85); }
            }

            @keyframes fire-breath {
                0% {
                    opacity: 0;
                    transform: translateX(0) scale(0.5);
                }
                20% {
                    opacity: 1;
                    transform: translateX(20px) scale(1);
                }
                100% {
                    opacity: 0;
                    transform: translateX(80px) scale(0.2);
                }
            }

            /* Speech bubble for dragon */
            .dragon-speech {
                position: absolute;
                bottom: 70px;
                left: 50%;
                transform: translateX(-50%);
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.95), rgba(10, 10, 10, 0.98));
                border: 2px solid #ff6b35;
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
            }

            .dragon-speech.visible {
                opacity: 1;
            }

            .dragon-speech::after {
                content: '';
                position: absolute;
                bottom: -8px;
                left: 50%;
                transform: translateX(-50%);
                border-width: 8px 8px 0;
                border-style: solid;
                border-color: #ff6b35 transparent transparent transparent;
            }
        "#}</Style>

        <div
            class="grokinho-container"
            style:left=move || format!("{}%", pos_x.get())
            style:top=move || format!("{}%", pos_y.get())
            style:transform=move || format!("rotate({}deg)", rotation.get())
        >
            <div
                class="grokinho-dragon"
                class:speaking=is_speaking
                class:firing=is_firing
            >
                "🐉"
            </div>

            <div class="fire-particles">
                <div class="fire-particle"></div>
                <div class="fire-particle"></div>
                <div class="fire-particle"></div>
                <div class="fire-particle"></div>
                <div class="fire-particle"></div>
            </div>
        </div>
    }
}

/// Speak text using Web Speech API
#[wasm_bindgen]
pub fn speak(text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let speech = window
        .speech_synthesis()
        .map_err(|e| JsValue::from_str(&format!("Failed to get speech synthesis: {:?}", e)))?;

    let utterance = SpeechSynthesisUtterance::new_with_text(text)
        .map_err(|e| JsValue::from_str(&format!("Failed to create utterance: {:?}", e)))?;

    utterance.set_lang("en-US");
    utterance.set_rate(1.0);
    utterance.set_pitch(1.0);
    utterance.set_volume(1.0);

    speech.speak(&utterance);
    Ok(())
}

/// Cancel current speech
#[wasm_bindgen]
pub fn cancel_speech() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
    let speech = window.speech_synthesis().map_err(|e| JsValue::from_str(&format!("Failed to get speech synthesis: {:?}", e)))?;
    speech.cancel();
    Ok(())
}
