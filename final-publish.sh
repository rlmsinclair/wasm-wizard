#!/bin/bash

echo "🧙‍♂️ Final Check Before Publishing WASM Wizard"
echo ""

# Check if logged in
if npm whoami >/dev/null 2>&1; then
    USER=$(npm whoami)
    echo "✅ Logged in as: $USER"
else
    echo "❌ Not logged in to npm"
    echo "Please run: npm login"
    exit 1
fi

echo ""
echo "📦 Package details:"
echo "  Name: wasm-wizard"
echo "  Version: 0.1.0"
echo "  Size: ~1.5MB"
echo ""

# Test the binary works
echo "🧪 Testing binary..."
if ./bin/wasm-wizard --version >/dev/null 2>&1; then
    echo "✅ Binary works!"
else
    echo "❌ Binary test failed"
    exit 1
fi

echo ""
echo "🚀 Ready to publish!"
echo ""
read -p "Publish to npm? (y/n): " confirm

if [[ $confirm == "y" ]]; then
    echo ""
    echo "📤 Publishing..."
    npm publish
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "🎉 SUCCESS! WASM Wizard is now live on npm!"
        echo ""
        echo "People can now install it with:"
        echo "  npm install -g wasm-wizard"
        echo ""
        echo "Test it yourself:"
        echo "  npx wasm-wizard --help"
        echo ""
        echo "View on npm: https://www.npmjs.com/package/wasm-wizard"
    else
        echo ""
        echo "❌ Publishing failed. Check the error above."
    fi
else
    echo "Publishing cancelled."
fi