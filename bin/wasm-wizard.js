#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');

const binary = path.join(__dirname, 'wasm-wizard');
const args = process.argv.slice(2);

const child = spawn(binary, args, { stdio: 'inherit' });

child.on('exit', (code) => {
  process.exit(code);
});
