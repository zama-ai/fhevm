import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import { initializeDemoLaunchAuthorization } from "./demoAuthorization";
import "./styles.css";

const disposeDemoLaunchAuthorization = initializeDemoLaunchAuthorization();
if (import.meta.hot) {
  import.meta.hot.dispose(disposeDemoLaunchAuthorization);
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
