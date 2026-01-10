import { StrictMode } from "react";
import { hydrateRoot, createRoot } from "react-dom/client";
import { RouterProvider, createBrowserHistory } from "@tanstack/react-router";
import { getRouter } from "./router";

const router = getRouter();

// Create browser history and update router
router.update({
  history: createBrowserHistory(),
});

const rootElement = document.getElementById("root")!;

// Check if we're hydrating SSR content or doing fresh render
if (rootElement.innerHTML.trim() && rootElement.innerHTML !== "<!--ssr-outlet-->") {
  // Hydrate SSR content
  hydrateRoot(
    rootElement,
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  );
} else {
  // Fresh client-side render (development or fallback)
  createRoot(rootElement).render(
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  );
}
