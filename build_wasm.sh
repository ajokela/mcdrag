#!/bin/bash

# Build script for MCDRAG WASM module

echo "Building MCDRAG WASM module..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
echo "Compiling to WASM..."
wasm-pack build --target web --out-dir pkg

# Create a simple HTTP server script if Python is available
cat > serve.py << 'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import os

PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

    def guess_type(self, path):
        mimetype = super().guess_type(path)
        if path.endswith('.wasm'):
            return 'application/wasm'
        return mimetype

os.chdir(os.path.dirname(os.path.abspath(__file__)))

with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
    print(f"Server running at http://localhost:{PORT}/")
    print("Open http://localhost:8000/index.html in your browser")
    print("Press Ctrl+C to stop the server")
    httpd.serve_forever()
EOF

chmod +x serve.py

echo ""
echo "Build complete!"
echo ""
echo "To run the web application:"
echo "  1. Run: python3 serve.py"
echo "  2. Open http://localhost:8000/index.html in your browser"
echo ""
echo "Alternatively, you can use any static web server to serve the files."