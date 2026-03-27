#!/bin/bash
# Test Rapfi Piskvork protocol

RAPFI_BIN="./binaries/rapfi-x86_64-unknown-linux-gnu"

echo "Testing Rapfi engine..."
echo "START 15" | $RAPFI_BIN --mode gomocup 2>&1 | head -20

echo ""
echo "If you see output above, Rapfi is working correctly."
