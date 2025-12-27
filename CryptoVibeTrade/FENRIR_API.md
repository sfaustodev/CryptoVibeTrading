# 🐺 Fenrir AI API Documentation

## Endpoint
**POST** `/api/gemini`

## Description
Dedicated endpoint for Fenrir AI - your technical analysis assistant for cryptocurrency markets. Powered by Google Gemini API.

## Request Format

### Headers
```
Content-Type: application/json
```

### Body
```json
{
  "prompt": "Your question here",
  "asset": "BTC",        // Optional: BTC, SOL, or ZEC
  "indicators": "RSI"    // Optional: Comma-separated indicators
}
```

## Response Format

### Success (200 OK)
```json
{
  "response": "🤖 Fenrir AI Analysis text here..."
}
```

### Error
```json
{
  "error": "Error message here"
}
```

## Example Usage

### cURL
```bash
curl -X POST http://127.0.0.1:3000/api/gemini \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "What is the current trend?",
    "asset": "BTC",
    "indicators": "Ichimoku Cloud, RSI, MACD"
  }'
```

### JavaScript/Fetch
```javascript
const response = await fetch('http://127.0.0.1:3000/api/gemini', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        prompt: 'Analyze the market',
        asset: 'BTC',
        indicators: 'RSI, MACD'
    })
});

const data = await response.json();
console.log(data.response);
```

### Python
```python
import requests

url = "http://127.0.0.1:3000/api/gemini"
payload = {
    "prompt": "What's the trend?",
    "asset": "BTC",
    "indicators": "Ichimoku, RSI"
}

response = requests.post(url, json=payload)
data = response.json()
print(data['response'])
```

## Demo Mode

If `GEMINI_API_KEY` is not set in the `.env` file, the endpoint runs in **demo mode** and returns:

```
🤖 Fenrir AI Analysis for BTC

Indicators: RSI, MACD

⚠️ Please add GEMINI_API_KEY to .env file
Get free key: https://aistudio.google.com/app/apikey

Analysis:
• Price action showing BTC
• Support levels holding
• Momentum indicators bullish
• Wait for confirmation

DYOR!
```

## Setup

1. **Get Free API Key**: Visit https://aistudio.google.com/app/apikey
2. **Add to .env**: Create a `.env` file in the project root:
   ```
   GEMINI_API_KEY=your_actual_api_key_here
   ```
3. **Restart Server**: The endpoint will automatically use the real API

## Features

✅ **Technical Analysis**: Specialized in crypto market analysis
✅ **Indicator Support**: Understands Ichimoku, RSI, MACD, Bollinger Bands
✅ **Multiple Assets**: BTC, SOL, ZEC analysis
✅ **Voice-Ready**: Responses optimized for text-to-speech (under 150 words)
✅ **Demo Mode**: Works without API key for testing
✅ **Fast Response**: Direct API calls without middleware

## Error Codes

- **404**: Gemini API endpoint not found (check API model name)
- **Network Error**: Connection issue with Gemini servers
- **Parse Error**: Invalid response from API

## Tips

1. **Be Specific**: Ask about specific indicators or patterns
2. **Asset Context**: Always specify the asset you're analyzing
3. **Indicator Selection**: Mention which indicators you're using
4. **Short Questions**: Keep prompts under 200 words for best results

## Example Prompts

- "What is the current trend based on Ichimoku?"
- "Is RSI showing overbought conditions?"
- "MACD bullish or bearish right now?"
- "Support and resistance levels?"
- "Should I wait for confirmation?"
