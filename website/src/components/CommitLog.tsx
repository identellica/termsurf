import { useState } from "react";

interface Commit {
  hash: string;
  message: string;
  body: string;
  author: string;
  date: string;
}

interface CommitLogProps {
  commits: Commit[];
}

const GITHUB_REPO = "https://github.com/termsurf/termsurf";

function formatRelativeDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return "today";
  if (diffDays === 1) return "yesterday";
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;
  return `${Math.floor(diffDays / 365)} years ago`;
}

function ChevronIcon({ expanded }: { expanded: boolean }) {
  return (
    <svg
      className={`w-4 h-4 text-muted transition-transform ${expanded ? "rotate-90" : ""}`}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
    </svg>
  );
}

function CommitRow({ commit }: { commit: Commit }) {
  const [expanded, setExpanded] = useState(false);
  const hasBody = commit.body && commit.body.length > 0;

  return (
    <li className="border-b border-background-highlight last:border-b-0">
      <button
        onClick={() => setExpanded(!expanded)}
        className="w-full text-left py-3 grid grid-cols-[auto_auto_1fr_auto] gap-4 items-baseline hover:bg-background-highlight/50 transition-colors cursor-pointer"
      >
        <ChevronIcon expanded={expanded} />
        <code className="font-mono text-sm text-secondary bg-background-dark px-1.5 py-0.5 rounded">
          {commit.hash.slice(0, 7)}
        </code>
        <span className="text-foreground truncate">{commit.message}</span>
        <span className="flex gap-3 text-sm whitespace-nowrap">
          <span className="text-success">{commit.author}</span>
          <span className="text-muted">{formatRelativeDate(commit.date)}</span>
        </span>
      </button>

      {expanded && (
        <div className="pl-8 pr-4 pb-4 space-y-3">
          <div className="bg-background-dark rounded p-3 text-sm">
            {hasBody ? (
              <pre className="whitespace-pre-wrap font-sans text-foreground-dark">{commit.body}</pre>
            ) : (
              <span className="text-muted italic">No commit message</span>
            )}
          </div>
          <a
            href={`${GITHUB_REPO}/commit/${commit.hash}`}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 text-sm text-accent hover:text-primary transition-colors"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
            View on GitHub
          </a>
        </div>
      )}
    </li>
  );
}

export function CommitLog({ commits }: CommitLogProps) {
  return (
    <section>
      <h2 className="text-xl font-semibold text-foreground mb-4">
        Recent Commits
      </h2>
      <ul className="space-y-0">
        {commits.map((commit) => (
          <CommitRow key={commit.hash} commit={commit} />
        ))}
      </ul>
    </section>
  );
}
