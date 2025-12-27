#!/bin/bash
echo "Testing Fenrir AI Endpoint..."
echo ""
echo "1. Testing demo mode (no API key):"
curl -s -X POST http://127.0.0.1:3000/api/gemini \
  -H "Content-Type: application/json" \
  -d '{"prompt":"What is the trend?","asset":"BTC","indicators":"Ichimoku, RSI"}' | jq '.'
echo ""
echo ""
echo "2. Testing with simple prompt:"
curl -s -X POST http://127.0.0.1:3000/api/gemini \
  -H "Content-Type: application/json" \
  -d '{"prompt":"Analyze SOL","asset":"SOL","indicators":"MACD"}' | jq '.'
