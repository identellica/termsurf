/**
 * Fetches recent commits from the TermSurf repository and writes them to data/commits.json
 * Run with: bun run build:commits
 */

import { writeFileSync, mkdirSync } from "fs";
import { join, dirname } from "path";

const REPO_PATH = join(import.meta.dir, "../..");
const OUTPUT_PATH = join(import.meta.dir, "../data/commits.json");
const COMMIT_COUNT = 50;

interface Commit {
  hash: string;
  message: string;
  author: string;
  date: string;
}

async function getCommits(): Promise<Commit[]> {
  const proc = Bun.spawn(
    [
      "git",
      "log",
      `--max-count=${COMMIT_COUNT}`,
      "--format=%H|%s|%an|%aI",
    ],
    {
      cwd: REPO_PATH,
      stdout: "pipe",
    }
  );

  const output = await new Response(proc.stdout).text();
  const lines = output.trim().split("\n").filter(Boolean);

  return lines.map((line) => {
    const [hash, message, author, date] = line.split("|");
    return { hash, message, author, date };
  });
}

async function main() {
  console.log("Fetching commits from repository...");

  const commits = await getCommits();

  const data = {
    generatedAt: new Date().toISOString(),
    commits,
  };

  // Ensure data directory exists
  mkdirSync(dirname(OUTPUT_PATH), { recursive: true });

  writeFileSync(OUTPUT_PATH, JSON.stringify(data, null, 2));

  console.log(`Wrote ${commits.length} commits to ${OUTPUT_PATH}`);
}

main().catch(console.error);
