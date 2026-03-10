#!/usr/bin/env node
'use strict';

const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const PLATFORM_PACKAGES = {
  'linux-x64': 'markdownlint-rs-linux-x64',
  'linux-arm64': 'markdownlint-rs-linux-arm64',
  'darwin-x64': 'markdownlint-rs-darwin-x64',
  'darwin-arm64': 'markdownlint-rs-darwin-arm64',
  'win32-x64': 'markdownlint-rs-win32-x64',
};

function findBinary() {
  const key = `${process.platform}-${process.arch}`;
  const pkgName = PLATFORM_PACKAGES[key];
  const binName = process.platform === 'win32' ? 'mdlint.exe' : 'mdlint';

  if (pkgName) {
    try {
      const pkgDir = path.dirname(require.resolve(`${pkgName}/package.json`));
      const binary = path.join(pkgDir, binName);
      if (fs.existsSync(binary)) return binary;
    } catch {}
  }

  console.error(`mdlint: no binary found for ${key}. Try reinstalling: npm install markdownlint-rs`);
  process.exit(1);
}

const result = spawnSync(findBinary(), process.argv.slice(2), { stdio: 'inherit' });

if (result.error) {
  console.error(`Failed to run mdlint: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status ?? 0);
