# Crypto Vibe Trade (CVT)

A full-stack, server-side rendered Leptos application for technical analysis and AI-powered crypto market insights.

## Features

- **Server-Side Rendering (SSR)** with Leptos
- **Ichimoku Cloud**, Bollinger Bands, and Stochastic RSI analysis
- **AI-Powered Analysis** powered by Grok (xAI)
- **Multi-asset Support**: Bitcoin (BTC), Solana (SOL), ZCash (ZEC)
- **Real-time Chat Interface** with market context
- **Dark Mode Terminal UI** with neon animations

## Quick Start

### Prerequisites

- Rust 2024 edition
- `XAI_API_KEY` environment variable (required)

### Setup

1. **Clone and navigate**:
   ```bash
   cd CryptoVibeTrade
   ```

2. **Configure environment** (optional):
   ```bash
   # Create .env file (optional - defaults shown)
   echo "XAI_API_KEY=your_key_here" >> .env
   echo "XAI_BASE_URL=https://api.x.ai/v1" >> .env
   echo "XAI_MODEL=grok-4" >> .env
   echo "CVT_ADDR=0.0.0.0:3000" >> .env
   ```

3. **Build and run**:
   ```bash
   cargo run
   ```
   
   Server runs at `http://localhost:3000` (or `$CVT_ADDR`)

## Architecture

### Backend
- **Server Functions**: `/api/*fn_name` endpoints
- `ai_analyze()` - Grok AI analysis with system prompt
- `connect_wallet()` - Wallet integration stub
- **Framework**: Axum + Tokio
- **Tracing**: Configured with environment-based filters

### Frontend
- **SSR with Hydration**: Full Rust components
- **Components**:
  - `Hero` - Asset selection (BTC/SOL/ZEC)
  - `Ichimoku` - Chart placeholder + analysis button
  - `AiTerminal` - Chat interface with real-time AI responses
- **Styling**: Inline CSS with dark mode, neon effects, animations
- **Responsive**: Mobile-first CSS Grid

## Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `XAI_API_KEY` | **required** | xAI API authentication |
| `XAI_BASE_URL` | `https://api.x.ai/v1` | xAI API endpoint |
| `XAI_MODEL` | `grok-4` | Grok model version |
| `CVT_ADDR` | `127.0.0.1:3000` | Server bind address |
| `RUST_LOG` | (unset) | Tracing log level filter |

## Commands

```bash
cargo build          # Compile project
cargo run            # Run development server (SSR)
cargo test           # Run all tests
cargo check          # Quick syntax/type check
cargo fmt            # Format code
cargo clippy         # Lint checks
```

## Project Structure

```
CryptoVibeTrade/
├── Cargo.toml                 # Project manifest
├── Cargo.lock                 # Dependency lock file
└── src/
    ├── main.rs               # Entry point + all components
    └── Front_End/
        └── landing.html      # Static assets (if needed)
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `leptos` (0.7) | Full-stack framework (SSR) |
| `axum` (0.7) | Web server router |
| `tokio` (1) | Async runtime |
| `reqwest` (0.12) | HTTP client (Grok API) |
| `serde` (1) | Serialization (JSON) |
| `tracing` (0.1) | Structured logging |

## API Routes

### Server Functions

All routes follow pattern: `/api/*fn_name`

- **POST `/api/ConnectWallet`** - Returns hardcoded wallet (stub)
- **POST `/api/AiAnalyze`** - Analyze asset with Grok AI
  - Request: `prompt`, `asset`
  - Response: AI analysis text

## Development Notes

- **No WASM yet**: Current focus is SSR + stable API
- **CSS inline**: All styling embedded in main.rs for simplicity
- **Persona**: "Fenrir AI" - Senior Technical Analyst
- **Language**: Portuguese-first (pt-BR)
- **Disclaimer**: "NAO DAMOS DICAS DE INVESTIMENTO" (We give no investment advice)

## License

MIT

---

**Status**: v0.1.0 (Early Development)
**Last Updated**: 2025-12-22
