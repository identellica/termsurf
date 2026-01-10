import { file } from "bun";
import { join } from "path";
import { renderToReadableStream } from "react-dom/server";
import { RouterProvider, createMemoryHistory } from "@tanstack/react-router";
import { getRouter } from "./src/router";

const PORT = process.env.PORT || 3000;
const CLIENT_DIR = join(import.meta.dir, "dist/client");
const isDev = process.env.NODE_ENV !== "production";

// Read the built index.html template
let indexHtml: string;
try {
  indexHtml = await Bun.file(join(CLIENT_DIR, "index.html")).text();
} catch {
  // In development, read from source
  indexHtml = await Bun.file(join(import.meta.dir, "index.html")).text();
}

// Extract parts of the HTML template for SSR injection
const [htmlStart, htmlEnd] = indexHtml.split("<!--ssr-outlet-->");

Bun.serve({
  port: Number(PORT),
  async fetch(req) {
    const url = new URL(req.url);
    const pathname = url.pathname;

    // Serve static assets from dist/client
    if (
      pathname.startsWith("/assets/") ||
      pathname.endsWith(".png") ||
      pathname.endsWith(".jpg") ||
      pathname.endsWith(".svg") ||
      pathname.endsWith(".ico") ||
      pathname.endsWith(".css") ||
      pathname.endsWith(".js")
    ) {
      const filePath = join(CLIENT_DIR, pathname);
      const staticFile = file(filePath);
      if (await staticFile.exists()) {
        return new Response(staticFile);
      }
    }

    // Also serve files from public directory
    const publicFile = file(join(CLIENT_DIR, pathname));
    if (await publicFile.exists()) {
      return new Response(publicFile);
    }

    // SSR for all other routes
    try {
      const router = getRouter();
      const memoryHistory = createMemoryHistory({
        initialEntries: [pathname + url.search],
      });

      router.update({
        history: memoryHistory,
      });

      // Wait for router to load data
      await router.load();

      // Render the app to a stream
      const stream = await renderToReadableStream(
        <RouterProvider router={router} />
      );

      // Convert stream to string for injection into template
      const appHtml = await new Response(stream).text();

      // Inject rendered HTML into template
      const fullHtml = htmlStart + appHtml + htmlEnd;

      return new Response(fullHtml, {
        headers: { "Content-Type": "text/html; charset=utf-8" },
      });
    } catch (error) {
      console.error("SSR Error:", error);
      // Fallback to client-side rendering
      return new Response(indexHtml, {
        headers: { "Content-Type": "text/html; charset=utf-8" },
      });
    }
  },
});

console.log(`Server running at http://localhost:${PORT}`);
