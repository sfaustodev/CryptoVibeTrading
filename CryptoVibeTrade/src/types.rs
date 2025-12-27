use serde::{Deserialize, Serialize};
use std::fmt;

// =====================
// WHITEBOARD TYPES
// =====================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tool {
    Pen,
    Eraser,
    Line,
    Rectangle,
    Circle,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Custom(u8, u8, u8), // RGB
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub color: Color,
    pub thickness: f64,
    pub tool: Tool,
}

// =====================
// DRAGROK EVENT TYPES
// =====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragrokEvent {
    pub intensity: u8,          // 0-100, quanto maior, mais fogo
    pub event_type: DragrokEventType,
    pub message: String,         // O que o Dragrok fala
    pub chart_data: Option<ChartInvocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragrokEventType {
    #[serde(rename = "FIRE")]
    Fire,
    #[serde(rename = "ALERT")]
    Alert,
    #[serde(rename = "GLOW")]
    Glow,
    #[serde(rename = "ROAR")]
    Roar,
    #[serde(rename = "SPEAK")]
    Speak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartInvocation {
    pub symbol: String,           // "BTCUSDT", "SOLUSDT"
    pub timeframe: String,         // "1m", "5m", "1h"
    pub indicators: Vec<String>,  // ["ichimoku", "ema", "rsi"]
    pub position: ChartPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPosition {
    pub x: f64,  // posição no canvas (0-100%)
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// =====================
// CHART DATA TYPES
// =====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorValue {
    pub name: String,
    pub value: f64,
    pub color: String,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => write!(f, "#000000"),
            Color::Red => write!(f, "#ff3333"),
            Color::Green => write!(f, "#00ff88"),
            Color::Blue => write!(f, "#3366ff"),
            Color::Yellow => write!(f, "#ffff33"),
            Color::Cyan => write!(f, "#33ffff"),
            Color::Magenta => write!(f, "#ff33ff"),
            Color::White => write!(f, "#ffffff"),
            Color::Custom(r, g, b) => write!(f, "#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}
