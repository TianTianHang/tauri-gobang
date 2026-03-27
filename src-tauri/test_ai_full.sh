#!/bin/bash
# Test the full AI integration

echo "🧪 Testing Rapfi AI integration..."
echo ""

# Check if the engine binary exists
ENGINE="./target/debug/build/tauri-gobang-*/out/rapfi-x86_64-unknown-linux-gnu"
ENGINE=$(ls $ENGINE 2>/dev/null | head -1)

if [ ! -f "$ENGINE" ]; then
    echo "❌ Engine binary not found"
    exit 1
fi

echo "✅ Engine binary: $ENGINE"
echo ""

# Check if config and models exist in the same directory
CONFIG_DIR=$(dirname "$ENGINE")/binaries
echo "📁 Checking config in: $CONFIG_DIR"

if [ -f "$CONFIG_DIR/config.toml" ]; then
    echo "✅ Config found ($(du -h "$CONFIG_DIR/config.toml" | cut -f1))"
else
    echo "❌ Config not found"
    exit 1
fi

# Count model files
MODEL_COUNT=$(ls -1 "$CONFIG_DIR"/*.lz4 2>/dev/null | wc -l)
echo "✅ Found $MODEL_COUNT model files"
echo ""

# Test the engine with a simple Piskvork protocol exchange
echo "🎮 Testing engine protocol..."
cd "$CONFIG_DIR"
echo -e "START 15\nINFO 1000\nTURN\nEND" | $(dirname "$ENGINE")/rapfi-* 2>&1 | head -20

echo ""
echo "✨ Test complete!"
