#!/usr/bin/env node

// Post-build script to remove unnecessary Monaco language files
// Keeps only the languages we actually use

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const distAssetsDir = path.join(__dirname, '..', 'dist', 'assets');

// Files to keep (everything else will be deleted)
// Be very specific about what to keep
const keepPatterns = [
  'rust-',           // Rust language support
  'javascript-',     // JavaScript support
  'typescript-',     // TypeScript support
  'css-',           // CSS support
  'html-',          // HTML support
  'markdown-',      // Markdown support
  'scss-',          // SCSS support
  'jsonMode-',      // JSON mode (workers)
  'cssMode-',       // CSS mode (workers)
  'htmlMode-',      // HTML mode (workers)
  'tsMode-',        // TypeScript mode (workers)
  'index-',         // Main bundle
  'codicon-',       // Icon font
];

if (!fs.existsSync(distAssetsDir)) {
  console.log('Assets directory not found, skipping cleanup');
  process.exit(0);
}

const files = fs.readdirSync(distAssetsDir);
let deletedCount = 0;
let keptCount = 0;
let totalSizeSaved = 0;

files.forEach(file => {
  const filePath = path.join(distAssetsDir, file);
  const stats = fs.statSync(filePath);
  
  if (!stats.isFile()) {
    return;
  }
  
  // For JS files, check if we should keep them
  if (file.endsWith('.js')) {
    // Check if it matches any of our keep patterns
    const shouldKeep = keepPatterns.some(pattern => file.startsWith(pattern));
    
    if (!shouldKeep) {
      totalSizeSaved += stats.size;
      fs.unlinkSync(filePath);
      console.log(`Removed: ${file} (${(stats.size / 1024).toFixed(2)} KB)`);
      deletedCount++;
    } else {
      console.log(`Kept: ${file}`);
      keptCount++;
    }
  }
  // Non-JS files (CSS, fonts, etc) are always kept
});

console.log('\n=== Monaco Language Cleanup Complete ===');
console.log(`Files removed: ${deletedCount}`);
console.log(`Files kept: ${keptCount}`);
console.log(`Total size saved: ${(totalSizeSaved / 1024 / 1024).toFixed(2)} MB`);