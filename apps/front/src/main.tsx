import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import moesifBrowser from "moesif-browser-js";
import "./main.css";
import App from "./App.js";
import "https://js.stripe.com/v3/pricing-table.js";
import "./styles/styles.scss";

import config from "./config.js";

let root = document.getElementById("root");
if (root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>
  );
}

if (config.moesif.publishableApplicationId) {
  moesifBrowser.init({
    applicationId: config.moesif.publishableApplicationId,
    // add other option here
  });
  if (window) {
    window.moesif = moesifBrowser;
  }
} else {
  console.log(
    "Please add using REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID to .env to enable Moesif Browser JS to track actions such as sign up"
  );
}
