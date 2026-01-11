import versionData from "../../data/version.json";

export function Header() {
  return (
    <header className="text-center mb-12 pb-8 border-b border-border">
      <img
        src="/logo.png"
        alt="TermSurf logo"
        className="w-16 h-16 mx-auto mb-4"
      />
      <h1 className="text-4xl font-bold text-primary mb-2">
        TermSurf{" "}
        <span className="text-sm font-normal text-foreground-dark bg-background-highlight px-2 py-0.5 rounded">
          v{versionData.version}
        </span>
      </h1>
      <p className="text-foreground-dark mb-4">Terminal + Browser</p>
      <nav>
        <a
          href="https://github.com/termsurf/termsurf"
          target="_blank"
          rel="noopener noreferrer"
          className="inline-block px-4 py-2 border border-border rounded text-accent hover:bg-background-highlight hover:border-accent transition-colors"
        >
          GitHub
        </a>
      </nav>
    </header>
  );
}
