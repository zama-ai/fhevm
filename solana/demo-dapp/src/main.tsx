import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import { consumeDemoLaunchAuthorization } from "./demoAuthorization";
import "./styles.css";

consumeDemoLaunchAuthorization();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
