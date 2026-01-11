import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/test-download")({
  component: TestDownloadPage,
});

function TestDownloadPage() {
  const downloadBlob = (content: string, filename: string, type: string) => {
    const blob = new Blob([content], { type });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="max-w-2xl mx-auto p-8">
      <h1 className="text-2xl font-bold mb-6">Download Test</h1>

      <section className="mb-6 p-4 bg-gray-100 rounded">
        <h2 className="font-semibold mb-2">Same-Origin Download</h2>
        <p className="text-sm text-gray-600 mb-2">
          Test download attribute with same-origin file:
        </p>
        <a
          href="/test-logo.png"
          download="my-logo.png"
          className="text-blue-600 underline"
        >
          Download Image
        </a>
      </section>

      <section className="mb-6 p-4 bg-gray-100 rounded">
        <h2 className="font-semibold mb-2">Blob Downloads</h2>
        <p className="text-sm text-gray-600 mb-2">
          Test JavaScript-generated downloads:
        </p>
        <button
          onClick={() =>
            downloadBlob("Hello from TermSurf!", "test.txt", "text/plain")
          }
          className="mr-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Download Text
        </button>
        <button
          onClick={() =>
            downloadBlob(
              JSON.stringify({ test: true, source: "TermSurf" }, null, 2),
              "test.json",
              "application/json"
            )
          }
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Download JSON
        </button>
      </section>
    </div>
  );
}
