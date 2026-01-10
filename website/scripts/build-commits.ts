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
  body: string;
  author: string;
  date: string;
}

// Use unlikely delimiters
const FIELD_SEP = "<<<FIELD>>>";
const RECORD_SEP = "<<<RECORD>>>";

async function getCommits(): Promise<Commit[]> {
  const proc = Bun.spawn(
    [
      "git",
      "log",
      `--max-count=${COMMIT_COUNT}`,
      `--format=%H${FIELD_SEP}%s${FIELD_SEP}%b${FIELD_SEP}%an${FIELD_SEP}%aI${RECORD_SEP}`,
    ],
    {
      cwd: REPO_PATH,
      stdout: "pipe",
    }
  );

  const output = await new Response(proc.stdout).text();
  const records = output.split(RECORD_SEP).filter((r) => r.trim());

  return records
    .map((record) => {
      const parts = record.split(FIELD_SEP);
      if (parts.length < 5) return null;
      const [hash, message, body, author, date] = parts;
      return {
        hash: (hash || "").trim(),
        message: (message || "").trim(),
        body: (body || "").trim(),
        author: (author || "").trim(),
        date: (date || "").trim(),
      };
    })
    .filter((c): c is Commit => c !== null && c.hash.length > 0);
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
