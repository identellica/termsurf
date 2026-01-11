/**
 * Fetches the latest release version from git tags and writes to data/version.json
 * Run with: bun run build:version
 */

import { writeFileSync, mkdirSync } from "fs";
import { join, dirname } from "path";

const REPO_PATH = join(import.meta.dir, "../..");
const OUTPUT_PATH = join(import.meta.dir, "../data/version.json");

interface VersionData {
  version: string;
  tag: string;
  date: string;
  generatedAt: string;
}

async function getLatestVersion(): Promise<VersionData> {
  // Get the latest tag
  const tagProc = Bun.spawn(["git", "describe", "--tags", "--abbrev=0"], {
    cwd: REPO_PATH,
    stdout: "pipe",
  });
  const tag = (await new Response(tagProc.stdout).text()).trim();

  // Get the date of the tag
  const dateProc = Bun.spawn(
    ["git", "log", "-1", "--format=%aI", tag],
    {
      cwd: REPO_PATH,
      stdout: "pipe",
    }
  );
  const date = (await new Response(dateProc.stdout).text()).trim();

  // Strip 'v' prefix for display
  const version = tag.startsWith("v") ? tag.slice(1) : tag;

  return {
    version,
    tag,
    date,
    generatedAt: new Date().toISOString(),
  };
}

async function main() {
  console.log("Fetching latest version from git tags...");

  const data = await getLatestVersion();

  // Ensure data directory exists
  mkdirSync(dirname(OUTPUT_PATH), { recursive: true });

  writeFileSync(OUTPUT_PATH, JSON.stringify(data, null, 2));

  console.log(`Wrote version ${data.version} to ${OUTPUT_PATH}`);
}

main().catch(console.error);
