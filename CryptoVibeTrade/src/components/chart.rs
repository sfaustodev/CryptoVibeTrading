use leptos::*;
use leptos_meta::Style;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsValue;
use crate::types::{Candle};

#[component]
pub fn TradingChart(
    /// Candle data (OHLCV) - reactive signal
    candles: Signal<Vec<Candle>>,
    /// Chart symbol
    #[prop(default = "BTCUSDT".to_string())]
    symbol: String,
) -> impl IntoView {
    view! {
        <Style>{r#"
            .simple-chart {
                font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
                background: #0a0a0a;
                border: 2px solid #1a1a1a;
                border-radius: 12px;
                padding: 20px;
                text-align: center;
            }
            .chart-title {
                font-size: 18px;
                font-weight: 700;
                color: #ff6b35;
                margin-bottom: 20px;
            }
            .candle-grid {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                gap: 8px;
            }
            .candle {
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 11px;
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .candle.bullish {
                background: rgba(0, 255, 136, 0.1);
                border: 1px solid #00ff88;
                color: #00ff88;
            }
            .candle.bearish {
                background: rgba(255, 51, 51, 0.1);
                border: 1px solid #ff3333;
                color: #ff3333;
            }
            .candle-price {
                font-weight: 700;
            }
        "#}</Style>

        <div class="simple-chart">
            <div class="chart-title">
                {move || format!("📊 {} Chart ({})", symbol, candles.get().len())}
            </div>

            <div class="candle-grid">
                {move || {
                    // Show last 20 candles or empty message
                    if candles.get().is_empty() {
                        view! {
                            <div style="color: #888; padding: 40px; grid-column: 1 / -1;">
                                "No candle data available. Connect to data source."
                            </div>
                        }.into_view()
                    } else {
                        let candles_vec = candles.get();
                        let display_candles: Vec<_> = candles_vec.iter().rev().take(20).collect();

                        display_candles.into_iter().map(|candle| {
                            let is_bullish = candle.close >= candle.open;
                            let change_pct = if candle.open > 0.0 {
                                ((candle.close - candle.open) / candle.open * 100.0)
                            } else {
                                0.0
                            };

                            view! {
                                <div class=format!("candle {}", if is_bullish { "bullish" } else { "bearish" })>
                                    <div class="candle-price">
                                        {format!("{:.2}", candle.close)}
                                    </div>
                                    <div style="font-size: 9px; opacity: 0.7;">
                                        {format!("{:+.2}%", change_pct)}
                                    </div>
                                    <div style="font-size: 9px; opacity: 0.5;">
                                        {format!("O:{:.2} H:{:.2} L:{:.2}", candle.open, candle.high, candle.low)}
                                    </div>
                                </div>
                            }
                        }).collect_view().into_view()
                    }
                }}
            </div>
        </div>
    }
}
