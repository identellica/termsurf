/**
 * Resize the TermSurf logo for web display
 * Run with: bun run scripts/resize-logo.ts
 */

import sharp from "sharp";
import { join } from "path";
import { mkdirSync } from "fs";

const SOURCE = join(import.meta.dir, "../../termsurf-macos/icon-source/termsurf-icon.png");
const OUTPUT_DIR = join(import.meta.dir, "../public");
const OUTPUT = join(OUTPUT_DIR, "logo.png");
const HEIGHT = 192; // 3x display size for Retina crispness

async function main() {
  // Ensure public directory exists
  mkdirSync(OUTPUT_DIR, { recursive: true });

  await sharp(SOURCE)
    .resize({ height: HEIGHT })
    .png({ quality: 90 })
    .toFile(OUTPUT);

  console.log(`Resized logo to ${HEIGHT}px height: ${OUTPUT}`);
}

main().catch(console.error);
