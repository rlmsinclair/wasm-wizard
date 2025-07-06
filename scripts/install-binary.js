#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');
const { execSync } = require('child_process');

const packageJson = require('../package.json');
const version = packageJson.version;

// Platform-specific binary info
const platforms = {
  'darwin-x64': 'wasm-wizard-darwin-x64',
  'darwin-arm64': 'wasm-wizard-darwin-arm64',
  'linux-x64': 'wasm-wizard-linux-x64',
  'linux-arm64': 'wasm-wizard-linux-arm64',
  'win32-x64': 'wasm-wizard-win32-x64.exe',
  'win32-arm64': 'wasm-wizard-win32-arm64.exe'
};

function getPlatformKey() {
  const platform = os.platform();
  const arch = os.arch();
  
  let key = `${platform}-${arch}`;
  
  // Handle specific arch mappings
  if (arch === 'x64') {
    key = `${platform}-x64`;
  } else if (arch === 'arm64') {
    key = `${platform}-arm64`;
  }
  
  return key;
}

function getBinaryPath() {
  const binDir = path.join(__dirname, '..', 'bin');
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }
  
  const platformKey = getPlatformKey();
  const binaryName = platforms[platformKey];
  
  if (!binaryName) {
    throw new Error(`Unsupported platform: ${platformKey}`);
  }
  
  return path.join(binDir, os.platform() === 'win32' ? 'wasm-wizard.exe' : 'wasm-wizard');
}

function downloadBinary() {
  const platformKey = getPlatformKey();
  const binaryName = platforms[platformKey];
  
  if (!binaryName) {
    console.error(`❌ Unsupported platform: ${platformKey}`);
    console.error('Please build from source or open an issue at:');
    console.error('https://github.com/your-username/wasm-wizard/issues');
    process.exit(1);
  }
  
  const binaryPath = getBinaryPath();
  const downloadUrl = `https://github.com/your-username/wasm-wizard/releases/download/v${version}/${binaryName}`;
  
  console.log(`🔽 Downloading wasm-wizard binary for ${platformKey}...`);
  console.log(`📥 From: ${downloadUrl}`);
  
  const file = fs.createWriteStream(binaryPath);
  
  https.get(downloadUrl, (response) => {
    if (response.statusCode === 404) {
      console.error(`❌ Binary not found for version ${version} and platform ${platformKey}`);
      console.error('Falling back to cargo install...');
      installFromCargo();
      return;
    }
    
    if (response.statusCode !== 200) {
      console.error(`❌ Failed to download binary: HTTP ${response.statusCode}`);
      installFromCargo();
      return;
    }
    
    response.pipe(file);
    
    file.on('finish', () => {
      file.close(() => {
        // Make binary executable on Unix systems
        if (os.platform() !== 'win32') {
          fs.chmodSync(binaryPath, 0o755);
        }
        
        console.log('✅ wasm-wizard binary installed successfully!');
        console.log(`📁 Installed at: ${binaryPath}`);
        console.log('🧙‍♂️ Try running: wasm-wizard --help');
      });
    });
  }).on('error', (err) => {
    console.error('❌ Download failed:', err.message);
    installFromCargo();
  });
}

function installFromCargo() {
  console.log('🦀 Installing wasm-wizard from source using cargo...');
  
  try {
    // Check if cargo is installed
    execSync('cargo --version', { stdio: 'ignore' });
  } catch (error) {
    console.error('❌ Cargo is not installed. Please install Rust and Cargo first:');
    console.error('   https://rustup.rs/');
    process.exit(1);
  }
  
  try {
    console.log('⚙️  Building wasm-wizard from source...');
    execSync('cargo install wasm-wizard', { stdio: 'inherit' });
    console.log('✅ wasm-wizard installed successfully via cargo!');
  } catch (error) {
    console.error('❌ Failed to install wasm-wizard via cargo:', error.message);
    console.error('Please check the installation logs above and try again.');
    process.exit(1);
  }
}

function main() {
  console.log('🧙‍♂️ Installing wasm-wizard...');
  
  // Try to download pre-built binary first
  downloadBinary();
}

if (require.main === module) {
  main();
}