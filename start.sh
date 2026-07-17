#!/bin/bash
set -e

echo "=== LeptosTask - Full-Stack Rust Todo App ==="
echo ""

echo "Starting Axum API server on port 3001..."
cd "$(dirname "$0")"
PORT=3001 cargo run --bin server &
SERVER_PID=$!

# Wait a moment for server to start
sleep 2

# Start the Leptos frontend with Trunk
echo "Starting Leptos frontend with Trunk..."
cd frontend
trunk serve --address 0.0.0.0 --port 8080

# Cleanup
kill $SERVER_PID 2>/dev/null || true
