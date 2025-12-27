# 🐺 Fenrir Admin Dashboard Guide

## Login Credentials

**Username:** `fenrir`
**Password:** `$4taN`

## Features

### 1. **Admin Dashboard**
After logging in, you'll see a multi-iframe dashboard displaying:
- **Orca** - Whirlpools (top, 60% height)
- **Raydium** - Liquidity Pools (bottom left, 40% height)
- **Meteora** - DEX Top Pairs (bottom right, 40% height)

All iframes are resizable and consume the full wide screen space.

### 2. **Grok API Integration**

The admin dashboard uses **Grok (xAI)** API for professional risk analysis.

#### API Endpoint
**POST** `/api/grok`

**Request:**
```json
{
  "prompt": "Your question here",
  "selectedText": "Selected text from page",
  "includeScreenshot": false
}
```

**Response:**
```json
{
  "response": "Grok's risk analysis..."
}
```

### 3. **System Prompt**

Grok uses this specialized system prompt:

> "You're a traditional professional of risk analysis and on chain analyst using blockchain protocols and explorers official free apis and really calculating the risk and possible PnL."

## Environment Setup

Create a `.env` file in the project root:

```bash
# For public Fenrir AI (Gemini)
GEMINI_API_KEY=your_gemini_key

# For admin Grok analysis
XAI_API_KEY=your_xai_key
```

### Getting API Keys

1. **Gemini (Free):** https://aistudio.google.com/app/apikey
2. **Grok (xAI):** https://x.ai/ (requires XAI account)

## Example Usage

### cURL - Basic Analysis
```bash
curl -X POST http://127.0.0.1:3000/api/grok \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "What's the risk?",
    "selectedText": "SOL-USDC pool 5% APY",
    "includeScreenshot": false
  }'
```

### JavaScript
```javascript
const response = await fetch('http://127.0.0.1:3000/api/grok', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        prompt: 'Analyze this pool',
        selectedText: window.getSelection().toString(),
        includeScreenshot: false
    })
});

const data = await response.json();
console.log(data.response);
```

## Security Notes

⚠️ **Important:**
- Credentials are hardcoded in `src/main.rs:44`
- This is a DEMO setup - **change for production!**
- Use proper session management and JWT tokens in production
- Store passwords hashed in database
- Use HTTPS in production

## Dashboard Layout

```
┌─────────────────────────────────────────┐
│  Navbar: Fenrir Admin Terminal          │
├─────────────────────────────────────────┤
│                                          │
│  ┌────────────────────────────────────┐ │
│  │  ORCA - Whirlpools (60% height)    │ │
│  └────────────────────────────────────┘ │
│                                          │
│  ┌──────────────────┬─────────────────┐ │
│  │  RAYDIUM (50%)   │  METEORA (50%)  │ │
│  │  Liquidity Pools │  DEX Top Pairs │ │
│  └──────────────────┴─────────────────┘ │
│                                          │
│           [Logout Button]                │
└─────────────────────────────────────────┘
```

## TODO (Future Features)

These features were requested but not yet implemented:

1. **Right-Click Context Menu**
   - Select text anywhere on the page
   - Right-click to send to Grok
   - "GROK WTF" option for screenshot analysis

2. **Screenshot Integration**
   - Capture full screen
   - Send to Grok with prompt: "Help me out, explain me wtf is all of this, what am I doing, should I continue?"
   - Requires browser screenshot API

3. **Resizable Iframes**
   - Drag to resize iframe heights
   - Custom split ratios

## Troubleshooting

**Login fails:**
- Check username is exactly `fenrir`
- Check password is exactly `$4taN` (case-sensitive)

**Grok API error:**
- Verify XAI_API_KEY in `.env`
- Check API key is valid at https://x.ai/
- Ensure you have API credits

**Iframes not loading:**
- Some sites block iframe embedding
- Check browser console for CORS errors
- Try accessing sites directly first

## Development

```bash
# Run server
cargo run

# Check for errors
cargo check

# Build optimized
cargo build --release
```

Server runs at: **http://127.0.0.1:3000**

---

**DYOR - Do Your Own Research!** 🐺
