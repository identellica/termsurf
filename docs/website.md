# termsurf.com Website

Project page for TermSurf showing commit history and project info.

## Tech Stack

- **Runtime:** Bun
- **Framework:** TanStack Start (React)
- **Styling:** Plain CSS (Tokyo Night theme)

## Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `@tanstack/react-start` | ^1.147.0 | Full-stack React framework |
| `@tanstack/react-router` | ^1.147.0 | Type-safe routing |
| `vite` | ^6.0.0 | Build tool (required by TanStack Start) |
| `react` | ^19.0.0 | UI library |
| `react-dom` | ^19.0.0 | React DOM bindings |
| `vinxi` | ^0.5.0 | Server framework (TanStack Start dependency) |

## Directory Structure

```
website/
├── package.json
├── app.config.ts           # TanStack Start configuration
├── tsconfig.json
├── app/
│   ├── routes/
│   │   ├── __root.tsx      # Root layout
│   │   └── index.tsx       # Home page (commit log)
│   ├── components/
│   │   ├── Header.tsx      # Site header
│   │   └── CommitLog.tsx   # Commit list display
│   ├── styles/
│   │   └── global.css      # Tokyo Night theme
│   └── client.tsx          # Client entry
├── data/
│   └── commits.json        # Pre-built commit data
├── scripts/
│   └── build-commits.ts    # Script to fetch/build commit data
└── public/
    └── (static assets)
```

## Scripts

| Command | Description |
|---------|-------------|
| `bun run dev` | Start development server with hot reload |
| `bun run build` | Production build |
| `bun run start` | Start production server |
| `bun run build:commits` | Fetch commits from GitHub and write to data/commits.json |

## MVP Checklist

### Phase 1: Project Setup

- [ ] Create `website/` directory
- [ ] Initialize with `bun create @tanstack/start@latest`
- [ ] Configure app.config.ts for Bun
- [ ] Verify dev server runs with `bun run dev`

### Phase 2: Commit Data Pipeline

- [ ] Create `scripts/build-commits.ts` to fetch commits from GitHub API
- [ ] Output commit data to `data/commits.json`
- [ ] Add `build:commits` script to package.json
- [ ] Test: `bun run build:commits` generates valid JSON

### Phase 3: Styling

- [ ] Create `app/styles/global.css` with Tokyo Night theme
- [ ] Import styles in root layout
- [ ] Add base typography and layout styles

### Phase 4: Components

- [ ] Create `Header.tsx` with TermSurf branding
- [ ] Create `CommitLog.tsx` to render commit list
- [ ] Each commit shows: short hash, message, author, relative date

### Phase 5: Home Page

- [ ] Build `index.tsx` route
- [ ] Load commits from `data/commits.json`
- [ ] Render Header and CommitLog components
- [ ] Verify SSR works correctly

### Phase 6: Polish & Deploy Prep

- [ ] Add meta tags (title, description, og:image)
- [ ] Test production build: `bun run build && bun run start`
- [ ] Document deployment options

## Commit Data Format

`data/commits.json` structure:

```json
{
  "generatedAt": "2025-01-10T12:00:00Z",
  "commits": [
    {
      "hash": "abc1234",
      "message": "Add feature X",
      "author": "ryan",
      "date": "2025-01-10T10:30:00Z"
    }
  ]
}
```

## Tokyo Night Colors

```css
:root {
  --bg: #1a1b26;
  --bg-dark: #16161e;
  --bg-highlight: #292e42;
  --fg: #c0caf5;
  --fg-dark: #a9b1d6;
  --blue: #7aa2f7;
  --cyan: #7dcfff;
  --green: #9ece6a;
  --magenta: #bb9af7;
  --red: #f7768e;
  --yellow: #e0af68;
  --orange: #ff9e64;
  --comment: #565f89;
  --border: #3b4261;
}
```

## Future Enhancements (Post-MVP)

- Pagination / infinite scroll for commits
- Filter by author or date range
- Release notes section
- Download links for latest release
- Project stats (stars, contributors)
- Blog / changelog section
