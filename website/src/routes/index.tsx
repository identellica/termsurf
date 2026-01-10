import { createFileRoute } from "@tanstack/react-router";
import { Header } from "../components/Header";
import { CommitLog } from "../components/CommitLog";
import commitsData from "../../data/commits.json";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  return (
    <div className="max-w-3xl mx-auto px-4 py-8">
      <Header />
      <main>
        <CommitLog commits={commitsData.commits} />
      </main>
    </div>
  );
}
