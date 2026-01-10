interface Commit {
  hash: string;
  message: string;
  author: string;
  date: string;
}

interface CommitLogProps {
  commits: Commit[];
}

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

export function CommitLog({ commits }: CommitLogProps) {
  return (
    <section>
      <h2 className="text-xl font-semibold text-foreground mb-4">
        Recent Commits
      </h2>
      <ul className="space-y-0">
        {commits.map((commit) => (
          <li
            key={commit.hash}
            className="grid grid-cols-[auto_1fr_auto] gap-4 items-baseline py-3 border-b border-background-highlight last:border-b-0"
          >
            <code className="font-mono text-sm text-secondary bg-background-dark px-1.5 py-0.5 rounded">
              {commit.hash.slice(0, 7)}
            </code>
            <span className="text-foreground truncate">{commit.message}</span>
            <span className="flex gap-3 text-sm whitespace-nowrap">
              <span className="text-success">{commit.author}</span>
              <span className="text-muted">{formatRelativeDate(commit.date)}</span>
            </span>
          </li>
        ))}
      </ul>
    </section>
  );
}
